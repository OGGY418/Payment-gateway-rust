use solana_transaction_status::EncodedConfirmedTransactionWithStatusMeta;
use chrono::{DateTime, Utc};

/// Parsed payment data from blockchain transaction
#[derive(Debug, Clone)]
pub struct ParsedPayment {
    pub signature: String,
    pub amount_lamports: i64,
    pub sender_address: String,
    pub receiver_address: String,
    pub memo: Option<String>,
    pub block_time: Option<DateTime<Utc>>,
}

/// Payment confirmation job for queue
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PaymentConfirmationJob {
    pub job_id: String,
    pub memo: String,
    pub sender_address: String,
    pub tx_sig: String,
    pub amount_lamports: i64,
    pub paid_at: Option<DateTime<Utc>>,
}

/// Parse Solana transaction to extract payment info
pub fn parse_transaction(
    tx: &EncodedConfirmedTransactionWithStatusMeta,
    signature: &str,
    wallet_address: &str,
) -> Option<ParsedPayment> {
    // Get transaction metadata
    let meta = tx.transaction.meta.as_ref()?;
    
    // Check if transaction was successful
    if meta.err.is_some() {
        println!("⚠️  Transaction {} failed, skipping", signature);
        return None;
    }

    // Extract pre and post balances
    let pre_balances = &meta.pre_balances;
    let post_balances = &meta.post_balances;

    // Get account keys and wallet index - handle both Parsed and Raw message types
    let tx_data = &tx.transaction.transaction;
    
    let (sender, wallet_index) = match tx_data {
        solana_transaction_status::EncodedTransaction::Json(ui_tx) => {
            match &ui_tx.message {
                // Handle Parsed message type
                solana_transaction_status::UiMessage::Parsed(parsed_msg) => {
                    let keys = &parsed_msg.account_keys;
                    
                    // Find our wallet's index
                    let idx = keys.iter().position(|acc| acc.pubkey == wallet_address)?;
                    
                    // Get sender (first account)
                    let sender_pubkey = keys.first()?.pubkey.clone();
                    
                    (sender_pubkey, idx)
                }
                // Handle Raw message type
                solana_transaction_status::UiMessage::Raw(raw_msg) => {
                    let keys = &raw_msg.account_keys;
                    
                    // Find our wallet's index
                    let idx = keys.iter().position(|key| key == wallet_address)?;
                    
                    // Get sender (first account)
                    let sender_pubkey = keys.first()?.clone();
                    
                    (sender_pubkey, idx)
                }
            }
        }
        _ => {
            println!("⚠️  Unsupported transaction encoding");
            return None;
        }
    };

    // Calculate amount received (post - pre balance)
    let pre_balance = *pre_balances.get(wallet_index)? as i64;
    let post_balance = *post_balances.get(wallet_index)? as i64;
    
    let amount_lamports = post_balance.saturating_sub(pre_balance);

    // Skip if no funds received
    if amount_lamports <= 0 {
        return None;
    }
    
    // Get receiver (our wallet)
    let receiver = wallet_address.to_string();

    // Extract memo from log messages
    let logs_option: Option<Vec<String>> = match &meta.log_messages {
        solana_transaction_status::option_serializer::OptionSerializer::Some(logs) => Some(logs.clone()),
        solana_transaction_status::option_serializer::OptionSerializer::None => None,
        solana_transaction_status::option_serializer::OptionSerializer::Skip => None,
    };
    
    let memo = extract_memo_from_logs(&logs_option);

    // Get block time
    let block_time = tx.block_time.map(|t| {
        DateTime::from_timestamp(t, 0).unwrap_or_else(|| Utc::now())
    });

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📥 NEW PAYMENT DETECTED");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("💰 Amount: {} lamports ({} SOL)", amount_lamports, amount_lamports as f64 / 1_000_000_000.0);
    println!("👤 From: {}", sender);
    println!("📍 To: {}", receiver);
    if let Some(ref m) = memo {
        println!("📝 Memo: {}", m);
    } else {
        println!("⚠️  No memo attached");
    }
    println!("🔗 Signature: {}", signature);
    if let Some(bt) = block_time {
        println!("⏰ Block time: {}", bt);
    }
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    Some(ParsedPayment {
        signature: signature.to_string(),
        amount_lamports,
        sender_address: sender,
        receiver_address: receiver,
        memo,
        block_time,
    })
}

/// Extract memo from transaction log messages
fn extract_memo_from_logs(logs: &Option<Vec<String>>) -> Option<String> {
    let logs = logs.as_ref()?;

    for log in logs {
        // Memos appear in logs like: "Program log: Memo (len 13): "PAY-ABC123""
        if log.contains("Program log: Memo") {
            // Extract the memo content between quotes
            if let Some(start) = log.find('"') {
                if let Some(end) = log.rfind('"') {
                    if start < end {
                        let memo = &log[start + 1..end];
                        return Some(memo.to_string());
                    }
                }
            }
        }
    }

    None
}

/// Convert ParsedPayment to PaymentConfirmationJob
pub fn payment_to_confirmation_job(payment: ParsedPayment) -> Option<PaymentConfirmationJob> {
    // Only create job if memo exists
    let memo = payment.memo?;

    Some(PaymentConfirmationJob {
        job_id: payment.signature.clone(),
        memo,
        sender_address: payment.sender_address,
        tx_sig: payment.signature,
        amount_lamports: payment.amount_lamports,
        paid_at: payment.block_time,
    })
}
