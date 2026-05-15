use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use async_trait::async_trait;
use super::provider::{DistributedLock, LockManager, LockConfig};

struct LockEntry {
    owner: String,
    expires_at: Instant,
}

#[derive(Clone)]
pub struct MemoryLockManager {
    locks: Arc<Mutex<HashMap<String, LockEntry>>>,
    owner_id: String,
}

impl MemoryLockManager {
    pub fn new(owner_id: String) -> Self {
        Self {
            locks: Arc::new(Mutex::new(HashMap::new())),
            owner_id,
        }
    }
}

pub struct MemoryLock {
    manager: MemoryLockManager,
    key: String,
    released: bool,
}

#[async_trait]
impl DistributedLock for MemoryLock {
    async fn release(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if self.released {
            return Ok(());
        }

        let mut locks = self.manager.locks.lock().await;
        if let Some(entry) = locks.get(&self.key) {
            if entry.owner == self.manager.owner_id {
                locks.remove(&self.key);
            }
        }
        self.released = true;
        Ok(())
    }
}

impl Drop for MemoryLock {
    fn drop(&mut self) {
        if !self.released {
            let manager = self.manager.clone();
            let key = self.key.clone();
            tokio::spawn(async move {
                let mut locks = manager.locks.lock().await;
                if let Some(entry) = locks.get(&key) {
                    if entry.owner == manager.owner_id {
                        locks.remove(&key);
                    }
                }
            });
        }
    }
}

#[async_trait]
impl LockManager for MemoryLockManager {
    async fn acquire(&self, config: LockConfig) -> Result<Box<dyn DistributedLock>, Box<dyn std::error::Error + Send + Sync>> {
        let mut attempts = 0;

        loop {
            let now = Instant::now();
            let mut acquired = false;

            {
                let mut locks = self.locks.lock().await;

                // Cleanup expired locks first
                locks.retain(|_, entry| entry.expires_at > now);

                if !locks.contains_key(&config.key) {
                    locks.insert(config.key.clone(), LockEntry {
                        owner: self.owner_id.clone(),
                        expires_at: now + config.ttl,
                    });
                    acquired = true;
                }
            }

            if acquired {
                return Ok(Box::new(MemoryLock {
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
