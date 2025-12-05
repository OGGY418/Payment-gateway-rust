mod api;
mod config;
mod database;
mod services;

use tokio::net::TcpListener;
use axum::Router;  
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    // Load configuration
    let config = match config::Config::from_env() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("❌ Failed to load config: {}", e);
            std::process::exit(1);
        }
    };

    println!("✅ Config loaded successfully!");

    // Connect to database
    let db = match database::Database::new(&config.database_url).await {
        Ok(database) => database,
        Err(e) => {
            eprintln!("❌ Failed to connect to database: {}", e);
            std::process::exit(1);
        }
    };

    // Initialize database (create tables)
    if let Err(e) = db.init().await {
        eprintln!("❌ Failed to initialize database: {}", e);
        std::process::exit(1);
    }

    // Get wallet address from environment
    let wallet_address = std::env::var("WALLET_ADDRESS")
        .expect("WALLET_ADDRESS must be set in .env");

    println!("✅ Database initialized!");
    println!("💳 Merchant wallet: {}", wallet_address);

    // Create app state
    let state = api::payments::AppState {
        db,
        wallet_address,
    };

    
   // Create API router with static file serving
    let api_routes = api::create_router(state);
    let app = Router::new()
        .fallback_service(ServeDir::new("public"))
        .nest("/", api_routes); 

    // Start server
    let addr = format!("{}:{}", config.server_host, config.server_port);
    let listener = TcpListener::bind(&addr).await.unwrap();

    println!("\n🚀 API Server running on http://{}", addr);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📡 POST /payments/create - Create payment request");
    println!("📡 GET  /payments/:id    - Check payment status");
    println!("📡 GET  /payments        - List all payments");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    axum::serve(listener, app).await.unwrap();
}
