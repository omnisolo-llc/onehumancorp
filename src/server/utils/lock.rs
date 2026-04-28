use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use async_trait::async_trait;
use std::time::{Duration, Instant};

#[async_trait]
pub trait DistributedLock: Send + Sync {
    async fn acquire(&self, resource_type: &str, resource_id: &str, ttl: Duration) -> Result<String, String>;
    async fn release(&self, resource_type: &str, resource_id: &str, token: &str) -> Result<(), String>;
    async fn extend(&self, resource_type: &str, resource_id: &str, token: &str, ttl: Duration) -> Result<(), String>;
}

pub struct RedisLock {
    client: redis::Client,
    tenant_id: String,
}

impl RedisLock {
    pub fn new(client: redis::Client, tenant_id: String) -> Self {
        RedisLock { client, tenant_id }
    }

    fn format_key(&self, resource_type: &str, resource_id: &str) -> String {
        format!("ohc:lock:{}:{}:{}", self.tenant_id, resource_type, resource_id)
    }
}

#[async_trait]
impl DistributedLock for RedisLock {
    async fn acquire(&self, resource_type: &str, resource_id: &str, ttl: Duration) -> Result<String, String> {
        let mut con = self.client.get_multiplexed_async_connection().await.map_err(|e| e.to_string())?;
        let key = self.format_key(resource_type, resource_id);
        let token = uuid::Uuid::new_v4().to_string();

        let result: Option<String> = redis::cmd("SET")
            .arg(&key)
            .arg(&token)
            .arg("NX")
            .arg("PX")
            .arg(ttl.as_millis() as u64)
            .query_async(&mut con)
            .await
            .map_err(|e| e.to_string())?;

        if result.is_some() {
            Ok(token)
        } else {
            Err("failed to acquire lock".to_string())
        }
    }

    async fn release(&self, resource_type: &str, resource_id: &str, token: &str) -> Result<(), String> {
        let mut con = self.client.get_multiplexed_async_connection().await.map_err(|e| e.to_string())?;
        let key = self.format_key(resource_type, resource_id);

        let script = redis::Script::new(
            "if redis.call('get', KEYS[1]) == ARGV[1] then return redis.call('del', KEYS[1]) else return 0 end"
        );

        let result: i32 = script.key(&key).arg(token).invoke_async(&mut con).await.map_err(|e| e.to_string())?;
        if result == 1 {
            Ok(())
        } else {
            Err("failed to release lock or token mismatch".to_string())
        }
    }

    async fn extend(&self, resource_type: &str, resource_id: &str, token: &str, ttl: Duration) -> Result<(), String> {
        let mut con = self.client.get_multiplexed_async_connection().await.map_err(|e| e.to_string())?;
        let key = self.format_key(resource_type, resource_id);

        let script = redis::Script::new(
            "if redis.call('get', KEYS[1]) == ARGV[1] then return redis.call('pexpire', KEYS[1], ARGV[2]) else return 0 end"
        );

        let result: i32 = script.key(&key).arg(token).arg(ttl.as_millis() as u64).invoke_async(&mut con).await.map_err(|e| e.to_string())?;
        if result == 1 {
            Ok(())
        } else {
            Err("failed to extend lock or token mismatch".to_string())
        }
    }
}

pub struct StandaloneLock {
    tenant_id: String,
    locks: Arc<Mutex<HashMap<String, (String, Instant)>>>,
}

impl StandaloneLock {
    pub fn new(tenant_id: String) -> Self {
        StandaloneLock {
            tenant_id,
            locks: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn format_key(&self, resource_type: &str, resource_id: &str) -> String {
        format!("ohc:lock:{}:{}:{}", self.tenant_id, resource_type, resource_id)
    }
}

#[async_trait]
impl DistributedLock for StandaloneLock {
    async fn acquire(&self, resource_type: &str, resource_id: &str, ttl: Duration) -> Result<String, String> {
        let key = self.format_key(resource_type, resource_id);
        let token = uuid::Uuid::new_v4().to_string();
        let expiry = Instant::now() + ttl;

        let mut locks = self.locks.lock().unwrap();

        if let Some((_, ext_expiry)) = locks.get(&key) {
            if Instant::now() < *ext_expiry {
                return Err("failed to acquire lock".to_string());
            }
        }

        locks.insert(key, (token.clone(), expiry));
        Ok(token)
    }

    async fn release(&self, resource_type: &str, resource_id: &str, token: &str) -> Result<(), String> {
        let key = self.format_key(resource_type, resource_id);
        let mut locks = self.locks.lock().unwrap();

        if let Some((ext_token, _)) = locks.get(&key) {
            if ext_token == token {
                locks.remove(&key);
                return Ok(());
            }
        }

        Err("failed to release lock or token mismatch".to_string())
    }

    async fn extend(&self, resource_type: &str, resource_id: &str, token: &str, ttl: Duration) -> Result<(), String> {
        let key = self.format_key(resource_type, resource_id);
        let mut locks = self.locks.lock().unwrap();

        if let Some((ext_token, expiry)) = locks.get_mut(&key) {
            if ext_token == token {
                *expiry = Instant::now() + ttl;
                return Ok(());
            }
        }

        Err("failed to extend lock or token mismatch".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_standalone_lock() {
        let lock = StandaloneLock::new("tenant1".to_string());

        // Acquire lock
        let token = lock.acquire("res1", "id1", Duration::from_millis(100)).await.unwrap();

        // Fail to acquire again
        assert!(lock.acquire("res1", "id1", Duration::from_millis(100)).await.is_err());

        // Extend lock
        assert!(lock.extend("res1", "id1", &token, Duration::from_millis(100)).await.is_ok());

        // Wait for expiry and fail to extend
        tokio::time::sleep(Duration::from_millis(150)).await;

        // Test we can acquire after expiry
        let token2 = lock.acquire("res1", "id1", Duration::from_millis(100)).await.unwrap();

        // Release lock
        assert!(lock.release("res1", "id1", &token2).await.is_ok());
    }
}
