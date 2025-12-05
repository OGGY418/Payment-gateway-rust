pub mod payments;

use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::CorsLayer;

use payments::AppState;

/// Create the API router with all routes
pub fn create_router(state: AppState) -> Router {
    Router::new()
        // Payment routes
        .route("/payments/create", post(payments::create_payment))
        .route("/payments/:id", get(payments::get_payment_status))
        .route("/payments", get(payments::list_payments))
        // Add CORS support
        .layer(CorsLayer::permissive())
        // Share state with all routes
        .with_state(state)
}
