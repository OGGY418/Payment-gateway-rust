use axum::{
    extract::{State, Path},
    http::StatusCode,
    Json,
};
use uuid::Uuid;
use chrono::{Utc, Duration};




use crate::database::models::{
    CreatePaymentRequest, PaymentRequest, PaymentResponse, PaymentStatusResponse, generate_memo
};
use crate::database::Database;
use std::sync::Arc;

/// App state with database
#[derive(Clone)]
pub struct AppState {
    pub db: Database,
    pub wallet_address: String,
}

/// POST /payments/create - Create payment request with unique memo
pub async fn create_payment(
    State(state): State<AppState>,
    Json(payload): Json<CreatePaymentRequest>,
) -> Result<Json<PaymentResponse>, StatusCode> {
    
    let payment_id = Uuid::new_v4();
    let memo = generate_memo();
    let now = Utc::now();
    let expires_at = now + Duration::minutes(15); // 15 minute expiry
    let token_symbol = payload.token_symbol.unwrap_or_else(|| "SOL".to_string());

    // Insert payment request into database
    sqlx::query(
        r#"
        INSERT INTO payment_requests 
        (id, amount_lamports, token_symbol, memo, status, receiver_address, created_at, expires_at, order_id, customer_email)
        VALUES ($1, $2, $3, $4, 'pending', $5, $6, $7, $8, $9)
        "#,
    )
    .bind(payment_id)
    .bind(payload.amount_lamports)
    .bind(&token_symbol)
    .bind(&memo)
    .bind(&state.wallet_address)
    .bind(now)
    .bind(expires_at)
    .bind(payload.order_id)
    .bind(payload.customer_email)
    .execute(&state.db.pool)
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let amount_sol = payload.amount_lamports as f64 / 1_000_000_000.0;
    let instructions = format!(
        "Send {} SOL to {} with memo: {}",
        amount_sol,
        state.wallet_address,
        memo
    );

    println!("✅ Payment request created: {} - Memo: {}", payment_id, memo);

    Ok(Json(PaymentResponse {
        payment_id: payment_id.to_string(),
        amount_lamports: payload.amount_lamports,
        token_symbol,
        receiver_address: state.wallet_address.clone(),
        memo,
        instructions,
        status: "pending".to_string(),
        created_at: now,
        expires_at: Some(expires_at),
    }))
}

/// GET /payments/:id - Get payment status
pub async fn get_payment_status(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<PaymentStatusResponse>, StatusCode> {
    
    let payment_id = Uuid::parse_str(&id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let payment = sqlx::query_as::<_, PaymentRequest>(
        r#"
        SELECT * FROM payment_requests
        WHERE id = $1
        "#,
    )
    .bind(payment_id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        StatusCode::NOT_FOUND
    })?;

    println!("📊 Payment status checked: {} - Status: {}", id, payment.status);

    Ok(Json(PaymentStatusResponse::from(payment)))
}

/// GET /payments - List all payments
pub async fn list_payments(
    State(state): State<AppState>,
) -> Result<Json<Vec<PaymentStatusResponse>>, StatusCode> {
    
    let payments = sqlx::query_as::<_, PaymentRequest>(
        r#"
        SELECT * FROM payment_requests
        ORDER BY created_at DESC
        LIMIT 100
        "#,
    )
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    println!("📊 Fetched {} payment requests", payments.len());

    let response: Vec<PaymentStatusResponse> = payments
        .into_iter()
        .map(PaymentStatusResponse::from)
        .collect();

    Ok(Json(response))
}
