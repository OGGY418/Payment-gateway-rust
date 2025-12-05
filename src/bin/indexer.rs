use payment_gateway_rust::{Config, Database, QueueService};
use payment_gateway_rust::indexer::{SolanaIndexer, parse_transaction, payment_to_confirmation_job};
use std::collections::HashSet;
use std::time::Duration;
use tokio::time;

#[tokio::main]
async fn main() {
    println!("⛓️  Starting Solana Blockchain Indexer...\n");

    // Load config
    let config = match Config::from_env() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("❌ Failed to load config: {}", e);
            std::process::exit(1);
        }
    };

    // Connect to database
    let db = match Database::new(&config.database_url).await {
        Ok(database) => database,
        Err(e) => {
            eprintln!("❌ Failed to connect to database: {}", e);
            std::process::exit(1);
        }
    };

    // Connect to Redis queue
    let mut queue = match QueueService::new(&config.redis_url).await {
        Ok(q) => q,
        Err(e) => {
            eprintln!("❌ Failed to connect to Redis: {}", e);
            std::process::exit(1);
        }
    };

    // Get wallet address from config
    let wallet_address = std::env::var("WALLET_ADDRESS")
        .expect("WALLET_ADDRESS must be set in .env");

    // Create Solana indexer
    let indexer = match SolanaIndexer::new(&config.solana_rpc_url, &wallet_address) {
        Ok(idx) => idx,
        Err(e) => {
            eprintln!("❌ Failed to create indexer: {}", e);
            std::process::exit(1);
        }
    };

    // Check wallet balance
    match indexer.get_balance() {
        Ok(balance) => {
            let sol = balance as f64 / 1_000_000_000.0;
            println!("💰 Wallet balance: {} SOL\n", sol);
        }
        Err(e) => {
            eprintln!("⚠️  Could not fetch balance: {}", e);
        }
    }

    println!("👀 Monitoring blockchain for payments...");
    println!("📋 Watching wallet: {}", wallet_address);
    println!("💡 Waiting for transactions with memos...\n");

    // Track processed signatures to avoid duplicates
    let mut processed_signatures: HashSet<String> = HashSet::new();

    // Main indexing loop
    let mut interval = time::interval(Duration::from_secs(5));

    loop {
        interval.tick().await;

        // Get recent transaction signatures
        let signatures = match indexer.get_recent_signatures(10) {
            Ok(sigs) => sigs,
            Err(e) => {
                eprintln!("❌ Error fetching signatures: {}", e);
                continue;
            }
        };

        // Process each signature
        for signature in signatures {
            // Skip if already processed
            if processed_signatures.contains(&signature) {
                continue;
            }

            println!("🔍 Found new transaction: {}", signature);

            // Get full transaction details
            let tx = match indexer.get_transaction(&signature) {
                Ok(Some(transaction)) => transaction,
                Ok(None) => {
                    println!("⚠️  Transaction not found (may still be processing)\n");
                    continue;
                }
                Err(e) => {
                    eprintln!("❌ Error fetching transaction: {}\n", e);
                    continue;
                }
            };

            // Parse transaction to extract payment data
            if let Some(payment) = parse_transaction(&tx, &signature, &wallet_address) {
                // Check if memo exists
                if payment.memo.is_none() {
                    println!("⚠️  No memo found, skipping transaction\n");
                    processed_signatures.insert(signature.clone());
                    continue;
                }

                let memo = payment.memo.as_ref().unwrap();

                // Check if payment request exists with this memo
                let payment_exists: Result<(uuid::Uuid,), sqlx::Error> = sqlx::query_as(
                    "SELECT id FROM payment_requests WHERE memo = $1 AND status = 'pending'"
                )
                .bind(memo)
                .fetch_one(&db.pool)
                .await;

                match payment_exists {
                    Ok((payment_id,)) => {
                        println!("✅ Found matching payment request: {}", payment_id);
                        
                        // Convert to confirmation job
                        if let Some(job) = payment_to_confirmation_job(payment) {
                            let job_json = serde_json::json!({
                                "payment_id": payment_id.to_string(),
                                "memo": job.memo,
                                "sender_address": job.sender_address,
                                "tx_sig": job.tx_sig,
                                "amount_lamports": job.amount_lamports,
                                "paid_at": job.paid_at,
                            });

                            // Check if already processed (idempotency)
                            match queue.is_job_processed(&job.job_id).await {
                                Ok(true) => {
                                    println!("⚠️  Payment {} already processed, skipping\n", job.job_id);
                                    processed_signatures.insert(signature.clone());
                                    continue;
                                }
                                Ok(false) => {
                                    // Push to confirmation queue
                                    match queue.push_confirmation_job(job_json).await {
                                        Ok(_) => {
                                            println!("✅ Confirmation job queued for payment: {}\n", payment_id);
                                            processed_signatures.insert(signature.clone());
                                            
                                            // Mark as processed
                                            let _ = queue.mark_job_processed(&job.job_id).await;
                                        }
                                        Err(e) => {
                                            eprintln!("❌ Failed to queue confirmation: {}\n", e);
                                        }
                                    }
                                }
                                Err(e) => {
                                    eprintln!("❌ Failed to check job status: {}\n", e);
                                }
                            }
                        }
                    }
                    Err(sqlx::Error::RowNotFound) => {
                        println!("⚠️  No matching payment request found for memo: {}", memo);
                        println!("   (Payment may be for different merchant or memo is invalid)\n");
                        processed_signatures.insert(signature.clone());
                    }
                    Err(e) => {
                        eprintln!("❌ Database error: {}\n", e);
                    }
                }
            } else {
                // No payment detected (might be outgoing or failed tx)
                processed_signatures.insert(signature.clone());
            }
        }

        // Clean up old signatures to prevent memory growth
        if processed_signatures.len() > 1000 {
            processed_signatures.clear();
            println!("🧹 Cleared processed signatures cache\n");
        }
    }
}
