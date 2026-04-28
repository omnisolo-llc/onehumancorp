use async_trait::async_trait;
use redis::AsyncCommands;
use std::time::Duration;
use uuid::Uuid;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;

#[async_trait]
pub trait DistributedLockProvider: Send + Sync {
    async fn acquire(&self, key: &str, timeout: Duration, expiration: Duration) -> Result<String, String>;
    async fn release(&self, key: &str, lock_value: &str) -> Result<(), String>;
}

pub struct CloudLockProvider {
    client: redis::Client,
}

impl CloudLockProvider {
    pub fn new(client: redis::Client) -> Self {
        CloudLockProvider { client }
    }
}

#[async_trait]
impl DistributedLockProvider for CloudLockProvider {
    async fn acquire(&self, key: &str, timeout: Duration, expiration: Duration) -> Result<String, String> {
        let lock_key = format!("lock:{}", key);
        let lock_value = Uuid::new_v4().to_string();
        let mut con = self.client.get_async_connection().await.map_err(|e| e.to_string())?;
        let start = std::time::Instant::now();

        loop {
            if start.elapsed() > timeout {
                return Err("timeout acquiring lock".to_string());
            }

            let res: Option<String> = redis::cmd("SET")
                .arg(&lock_key)
                .arg(&lock_value)
                .arg("NX")
                .arg("PX")
                .arg(expiration.as_millis() as u64)
                .query_async(&mut con)
                .await
                .map_err(|e| e.to_string())?;

            if res.is_some() {
                return Ok(lock_value);
            }

            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    async fn release(&self, key: &str, lock_value: &str) -> Result<(), String> {
        let lock_key = format!("lock:{}", key);
        let mut con = self.client.get_async_connection().await.map_err(|e| e.to_string())?;
        let script = redis::Script::new(r#"
            if redis.call("get", KEYS[1]) == ARGV[1] then
                return redis.call("del", KEYS[1])
            else
                return 0
            end
        "#);

        let res: i32 = script.key(&lock_key).arg(lock_value).invoke_async(&mut con).await.map_err(|e| e.to_string())?;

        if res == 1 {
            Ok(())
        } else {
            Err("failed to release lock: not owner or lock expired".to_string())
        }
    }
}

pub struct StandaloneLockProvider {
    // Tracks lock value and expiration time
    locks: Arc<Mutex<HashMap<String, (String, std::time::Instant)>>>,
}

impl StandaloneLockProvider {
    pub fn new() -> Self {
        StandaloneLockProvider {
            locks: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl DistributedLockProvider for StandaloneLockProvider {
    async fn acquire(&self, key: &str, timeout: Duration, expiration: Duration) -> Result<String, String> {
        let lock_key = format!("lock:{}", key);
        let lock_value = Uuid::new_v4().to_string();
        let start = std::time::Instant::now();

        loop {
            if start.elapsed() > timeout {
                return Err("timeout acquiring lock".to_string());
            }

            let mut locks = self.locks.lock().await;

            // Check if lock exists and is not expired
            let is_available = match locks.get(&lock_key) {
                Some((_, exp_time)) => {
                    if std::time::Instant::now() > *exp_time {
                        // Lock expired, we can take it
                        true
                    } else {
                        false
                    }
                }
                None => true,
            };

            if is_available {
                locks.insert(lock_key.clone(), (lock_value.clone(), std::time::Instant::now() + expiration));
                return Ok(lock_value);
            }

            // We must drop the lock before sleeping to avoid deadlocks
            drop(locks);
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    }

    async fn release(&self, key: &str, lock_value: &str) -> Result<(), String> {
        let lock_key = format!("lock:{}", key);
        let mut locks = self.locks.lock().await;

        if let Some((current_value, _)) = locks.get(&lock_key) {
            if current_value == lock_value {
                locks.remove(&lock_key);
                return Ok(());
            }
        }

        Err("failed to release lock: not owner or lock expired".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_standalone_lock() {
        let provider = StandaloneLockProvider::new();

        // Test acquire
        let lock_val1 = provider.acquire("test1", Duration::from_secs(1), Duration::from_secs(1)).await;
        assert!(lock_val1.is_ok());
        let val1 = lock_val1.unwrap();

        // Test concurrent acquire fails (timeout)
        let lock_val2 = provider.acquire("test1", Duration::from_millis(200), Duration::from_secs(1)).await;
        assert!(lock_val2.is_err());

        // Test release
        let release_res = provider.release("test1", &val1).await;
        assert!(release_res.is_ok());

        // Test acquire again after release
        let lock_val3 = provider.acquire("test1", Duration::from_secs(1), Duration::from_secs(1)).await;
        assert!(lock_val3.is_ok());
    }

    #[tokio::test]
    async fn test_standalone_lock_expiration() {
        let provider = StandaloneLockProvider::new();

        // Test acquire with short expiration
        let _ = provider.acquire("test_exp", Duration::from_secs(1), Duration::from_millis(100)).await.unwrap();

        // Wait for it to expire
        tokio::time::sleep(Duration::from_millis(150)).await;

        // Test acquire again should succeed because it expired
        let lock_val2 = provider.acquire("test_exp", Duration::from_secs(1), Duration::from_secs(1)).await;
        assert!(lock_val2.is_ok());
    }

    #[tokio::test]
    async fn test_standalone_lock_wrong_value() {
        let provider = StandaloneLockProvider::new();

        // Test acquire
        let _lock_val = provider.acquire("test2", Duration::from_secs(1), Duration::from_secs(1)).await.unwrap();

        // Test release with wrong value
        let release_res = provider.release("test2", "wrong_value").await;
        assert!(release_res.is_err());
    }
}
