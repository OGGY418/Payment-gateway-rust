use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Payment Request model (matches database table)
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PaymentRequest {
    pub id: Uuid,
    pub amount_lamports: i64,
    pub token_symbol: String,
    pub memo: String,
    pub status: String,
    pub receiver_address: String,
    pub sender_address: Option<String>,
    pub tx_sig: Option<String>,
    pub block_height: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub paid_at: Option<DateTime<Utc>>,
    pub updated_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub order_id: Option<String>,
    pub customer_email: Option<String>,
}

/// Create payment request body
#[derive(Debug, Deserialize)]
pub struct CreatePaymentRequest {
    pub amount_lamports: i64,
    pub token_symbol: Option<String>,
    pub order_id: Option<String>,
    pub customer_email: Option<String>,
}

/// Payment response for API
#[derive(Debug, Serialize)]
pub struct PaymentResponse {
    pub payment_id: String,
    pub amount_lamports: i64,
    pub token_symbol: String,
    pub receiver_address: String,
    pub memo: String,
    pub instructions: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

/// Payment status response
#[derive(Debug, Serialize)]
pub struct PaymentStatusResponse {
    pub id: String,
    pub status: String,
    pub amount_lamports: i64,
    pub token_symbol: String,
    pub memo: String,
    pub sender_address: Option<String>,
    pub receiver_address: String,
    pub tx_sig: Option<String>,
    pub paid_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl From<PaymentRequest> for PaymentStatusResponse {
    fn from(payment: PaymentRequest) -> Self {
        PaymentStatusResponse {
            id: payment.id.to_string(),
            status: payment.status,
            amount_lamports: payment.amount_lamports,
            token_symbol: payment.token_symbol,
            memo: payment.memo,
            sender_address: payment.sender_address,
            receiver_address: payment.receiver_address,
            tx_sig: payment.tx_sig,
            paid_at: payment.paid_at,
            created_at: payment.created_at,
        }
    }
}

/// Generate unique memo
pub fn generate_memo() -> String {
    let uuid = Uuid::new_v4().to_string();
    format!("PAY-{}", &uuid[..8].to_uppercase())
}
