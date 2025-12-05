use redis::{aio::ConnectionManager, AsyncCommands, Client};
use serde::{Deserialize, Serialize};

/// Payment job structure for Redis queue

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentJob {
    pub job_id: String,
    pub amount: i64,
    pub memo: Option<String>,
    pub sender: Option<String>,
}

/// Redis queue manager

pub struct QueueService {
    connection: ConnectionManager,
}

impl QueueService {
    /// Create new queue service
    pub async fn new(redis_url: &str) -> Result<Self, redis::RedisError> {
        let client = Client::open(redis_url)?;
        let connection = ConnectionManager::new(client).await?;
        
        println!("✅ Redis connected successfully!");
        
        Ok(QueueService { connection })
    }

    /// Push payment job to queue
    
    pub async fn push_payment_job(&mut self, job: PaymentJob) -> Result<(), redis::RedisError> {
        let job_json = serde_json::to_string(&job)
            .map_err(|e| redis::RedisError::from((redis::ErrorKind::TypeError, "Serialization failed", e.to_string())))?;

        self.connection
            .lpush::<_, _, ()>("payment_queue", job_json)
            .await?;

        println!("📤 Job pushed to queue: {}", job.job_id);
        
        Ok(())
    }

    /// Pop payment job from queue (for worker)
    
    pub async fn pop_payment_job(&mut self) -> Result<Option<PaymentJob>, redis::RedisError> {
        // Use BRPOP (blocking pop) - waits for jobs
        let result: Option<(String, String)> = self.connection
            .brpop("payment_queue", 0.0)
            .await?;

        match result {
            Some((_key, job_json)) => {
                let job: PaymentJob = serde_json::from_str(&job_json)
                    .map_err(|e| redis::RedisError::from((redis::ErrorKind::TypeError, "Deserialization failed", e.to_string())))?;
                
                println!("📥 Job popped from queue: {}", job.job_id);
                Ok(Some(job))
            }
            None => Ok(None),
        }
    }

    /// Check if job already processed (idempotency)
    
    pub async fn is_job_processed(&mut self, job_id: &str) -> Result<bool, redis::RedisError> {
        let exists: bool = self.connection
            .exists(format!("processed:{}", job_id))
            .await?;
        
        Ok(exists)
    }

    /// Mark job as processed (idempotency)
    pub async fn mark_job_processed(&mut self, job_id: &str) -> Result<(), redis::RedisError> {
        let _: () = self.connection
            .set_ex(format!("processed:{}", job_id), "1", 86400) // 24 hours TTL
            .await?;
        
        Ok(())
    }
/// Push payment confirmation job to queue
    pub async fn push_confirmation_job(&mut self, job: serde_json::Value) -> Result<(), redis::RedisError> {
        let job_json = serde_json::to_string(&job)
            .map_err(|e| redis::RedisError::from((redis::ErrorKind::TypeError, "Serialization failed", e.to_string())))?;

        self.connection
            .lpush::<_, _, ()>("confirmation_queue", job_json)
            .await?;

        println!("📤 Confirmation job pushed to queue");
        
        Ok(())
    }

    /// Pop confirmation job from queue (for worker)
    pub async fn pop_confirmation_job(&mut self) -> Result<Option<serde_json::Value>, redis::RedisError> {
        let result: Option<(String, String)> = self.connection
            .brpop("confirmation_queue", 0.0)
            .await?;

        match result {
            Some((_key, job_json)) => {
                let job: serde_json::Value = serde_json::from_str(&job_json)
                    .map_err(|e| redis::RedisError::from((redis::ErrorKind::TypeError, "Deserialization failed", e.to_string())))?;
                
                println!("📥 Confirmation job popped from queue");
                Ok(Some(job))
            }
            None => Ok(None),
        }
    }







}
