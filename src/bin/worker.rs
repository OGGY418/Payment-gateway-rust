use payment_gateway_rust::{Config, Database, QueueService};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    println!("🔄 Starting Payment Confirmation Worker...\n");

    // Load configuration
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

    println!("✅ Worker connected to database and queue!");
    println!("👂 Listening for payment confirmation jobs...\n");

    // Main worker loop
    loop {
        // Pop confirmation job from queue (blocking)
        match queue.pop_confirmation_job().await {
            Ok(Some(job)) => {
                println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                println!("📦 Processing Confirmation Job");
                println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

                // Extract job data
                let payment_id = job["payment_id"].as_str().unwrap_or("");
                let memo = job["memo"].as_str().unwrap_or("");
                let sender_address = job["sender_address"].as_str().unwrap_or("");
                let tx_sig = job["tx_sig"].as_str().unwrap_or("");
                let amount_lamports = job["amount_lamports"].as_i64().unwrap_or(0);
                
                // Parse paid_at timestamp
                let paid_at: Option<DateTime<Utc>> = job["paid_at"].as_str()
                    .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                    .map(|dt| dt.with_timezone(&Utc));

                println!("💳 Payment ID: {}", payment_id);
                println!("📝 Memo: {}", memo);
                println!("👤 Sender: {}", sender_address);
                println!("🔗 Signature: {}", tx_sig);
                println!("💰 Amount: {} lamports", amount_lamports);
                if let Some(pt) = paid_at {
                    println!("⏰ Paid at: {}", pt);
                }

                // Parse payment_id
                let payment_uuid = match Uuid::parse_str(payment_id) {
                    Ok(uuid) => uuid,
                    Err(e) => {
                        eprintln!("❌ Invalid payment_id: {}", e);
                        continue;
                    }
                };

                // Update payment request in database
                match update_payment_confirmation(
                    &db,
                    payment_uuid,
                    sender_address,
                    tx_sig,
                    paid_at,
                ).await {
                    Ok(_) => {
                        println!("✅ Payment confirmed successfully!");
                        println!("   Status: pending → confirmed");
                        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to update payment: {}", e);
                        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
                    }
                }
            }
            Ok(None) => {
                // No job available (shouldn't happen with blocking pop)
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }
            Err(e) => {
                eprintln!("❌ Queue error: {}", e);
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            }
        }
    }
}

/// Update payment request with confirmation details
async fn update_payment_confirmation(
    db: &Database,
    payment_id: Uuid,
    sender_address: &str,
    tx_sig: &str,
    paid_at: Option<DateTime<Utc>>,
) -> Result<(), sqlx::Error> {
    let now = Utc::now();

    sqlx::query(
        r#"
        UPDATE payment_requests
        SET status = 'confirmed',
            sender_address = $2,
            tx_sig = $3,
            paid_at = $4,
            updated_at = $5
        WHERE id = $1
        "#,
    )
    .bind(payment_id)
    .bind(sender_address)
    .bind(tx_sig)
    .bind(paid_at)
    .bind(now)
    .execute(&db.pool)
    .await?;

    Ok(())
}
