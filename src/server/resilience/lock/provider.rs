use std::time::Duration;
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct LockConfig {
    pub key: String,
    pub ttl: Duration,
    pub retry_count: u32,
    pub retry_delay: Duration,
}

impl Default for LockConfig {
    fn default() -> Self {
        Self {
            key: String::new(),
            ttl: Duration::from_secs(30),
            retry_count: 3,
            retry_delay: Duration::from_millis(500),
        }
    }
}

#[async_trait]
pub trait DistributedLock: Send + Sync {
    /// Release the lock. Returns Ok(()) if the lock was successfully released or wasn't held.
    async fn release(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}

#[async_trait]
pub trait LockManager: Send + Sync {
    /// Attempt to acquire a lock with the given configuration.
    /// If the lock cannot be acquired after retries, returns an Error.
    async fn acquire(&self, config: LockConfig) -> Result<Box<dyn DistributedLock>, Box<dyn std::error::Error + Send + Sync>>;
}
