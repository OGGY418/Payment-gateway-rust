pub mod models;

use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;

/// Database connection pool
#[derive(Clone)]
pub struct Database {
    pub pool: PgPool,
}

impl Database {
    /// Create a new database connection pool
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(3))
            .connect(database_url)
            .await?;

        println!("✅ Database connected successfully!");

        Ok(Database { pool })
    }

    /// Initialize database (create tables)
    pub async fn init(&self) -> Result<(), sqlx::Error> {
        // Create payment_requests table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS payment_requests (
                id UUID PRIMARY KEY,
                amount_lamports BIGINT NOT NULL,
                token_symbol TEXT DEFAULT 'SOL',
                memo TEXT UNIQUE NOT NULL,
                status TEXT NOT NULL DEFAULT 'pending',
                receiver_address TEXT NOT NULL,
                sender_address TEXT,
                tx_sig TEXT,
                block_height BIGINT,
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                paid_at TIMESTAMPTZ,
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                expires_at TIMESTAMPTZ,
                order_id TEXT,
                customer_email TEXT,
                metadata JSONB
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Create indexes
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_memo ON payment_requests(memo)")
            .execute(&self.pool)
            .await?;
        
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_status ON payment_requests(status)")
            .execute(&self.pool)
            .await?;
        
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_tx_sig ON payment_requests(tx_sig)")
            .execute(&self.pool)
            .await?;

        println!("✅ Payment_requests table created/verified!");

        Ok(())
    }
}
