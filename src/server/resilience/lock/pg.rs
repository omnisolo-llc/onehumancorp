use std::sync::Arc;
use sqlx::{PgPool, Executor};
use async_trait::async_trait;
use uuid::Uuid;
use super::provider::{DistributedLock, LockManager, LockConfig};

#[derive(Clone)]
pub struct PostgresLockManager {
    pool: Arc<PgPool>,
    owner_id: String,
}

impl PostgresLockManager {
    pub fn new(pool: PgPool, owner_id: String) -> Self {
        Self {
            pool: Arc::new(pool),
            owner_id,
        }
    }

    // Create the required table if it doesn't exist
    pub async fn ensure_table(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let query = r#"
            CREATE TABLE IF NOT EXISTS distributed_locks (
                key VARCHAR(255) PRIMARY KEY,
                owner VARCHAR(255) NOT NULL,
                expires_at TIMESTAMPTZ NOT NULL
            )
        "#;
        self.pool.execute(query).await?;
        Ok(())
    }
}

pub struct PostgresLock {
    manager: PostgresLockManager,
    key: String,
    released: bool,
}

#[async_trait]
impl DistributedLock for PostgresLock {
    async fn release(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if self.released {
            return Ok(());
        }

        let query = "DELETE FROM distributed_locks WHERE key = $1 AND owner = $2";
        sqlx::query(query)
            .bind(&self.key)
            .bind(&self.manager.owner_id)
            .execute(&*self.manager.pool)
            .await?;

        self.released = true;
        Ok(())
    }
}

impl Drop for PostgresLock {
    fn drop(&mut self) {
        if !self.released {
            let manager = self.manager.clone();
            let key = self.key.clone();
            tokio::spawn(async move {
                let query = "DELETE FROM distributed_locks WHERE key = $1 AND owner = $2";
                let _ = sqlx::query(query)
                    .bind(&key)
                    .bind(&manager.owner_id)
                    .execute(&*manager.pool)
                    .await;
            });
        }
    }
}

#[async_trait]
impl LockManager for PostgresLockManager {
    async fn acquire(&self, config: LockConfig) -> Result<Box<dyn DistributedLock>, Box<dyn std::error::Error + Send + Sync>> {
        let mut attempts = 0;

        loop {
            // First, cleanup expired locks
            let cleanup_query = "DELETE FROM distributed_locks WHERE expires_at < NOW()";
            let _ = sqlx::query(cleanup_query).execute(&*self.pool).await;

            // Try to acquire the lock
            let ttl_secs = config.ttl.as_secs_f64();
            let acquire_query = r#"
                INSERT INTO distributed_locks (key, owner, expires_at)
                VALUES ($1, $2, NOW() + interval '1 second' * $3)
                ON CONFLICT (key) DO NOTHING
                RETURNING key
            "#;

            let result = sqlx::query(acquire_query)
                .bind(&config.key)
                .bind(&self.owner_id)
                .bind(ttl_secs)
                .fetch_optional(&*self.pool)
                .await?;

            if result.is_some() {
                return Ok(Box::new(PostgresLock {
                    manager: self.clone(),
                    key: config.key,
                    released: false,
                }));
            }

            attempts += 1;
            if attempts > config.retry_count {
                return Err(format!("Failed to acquire lock for key {} after {} attempts", config.key, attempts).into());
            }

            tokio::time::sleep(config.retry_delay).await;
        }
    }
}
