use std::env;

/// Configuration struct - holds all environment variables
#[derive(Debug, Clone)]
pub struct Config {
    pub server_host: String,
    pub server_port: u16,
    pub database_url: String,
    pub redis_url: String,
    pub solana_rpc_url: String,
    pub jwt_secret: String,
}

impl Config {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self, String> {
        // Load .env file
        dotenv::dotenv().ok();

        Ok(Config {
            server_host: env::var("HOST")
                .unwrap_or_else(|_| "0.0.0.0".to_string()),
            
            server_port: env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .map_err(|_| "Invalid PORT")?,
            
            database_url: env::var("DATABASE_URL")
                .map_err(|_| "DATABASE_URL must be set")?,
            
            redis_url: env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string()),
            
            solana_rpc_url: env::var("SOLANA_RPC_URL")
                .unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".to_string()),
            
            jwt_secret: env::var("JWT_SECRET")
                .map_err(|_| "JWT_SECRET must be set")?,
        })
    }
}     
