use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;

#[async_trait]
pub trait DistributedLock: Send + Sync {
    /// Attempts to acquire a lock with a specific TTL (in seconds)
    async fn acquire(&self, resource_id: &str, ttl_seconds: u64) -> Result<bool, String>;

    /// Releases the lock if the current process holds it
    async fn release(&self, resource_id: &str) -> Result<(), String>;
}

pub struct MemoryLock {
    // Maps resource_id to expiration timestamp in seconds
    locks: Mutex<HashMap<String, u64>>,
}

impl MemoryLock {
    pub fn new() -> Self {
        MemoryLock {
            locks: Mutex::new(HashMap::new()),
        }
    }

    fn current_time() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }
}

#[async_trait]
impl DistributedLock for MemoryLock {
    async fn acquire(&self, resource_id: &str, ttl_seconds: u64) -> Result<bool, String> {
        let mut locks = self.locks.lock().await;
        let now = Self::current_time();

        if let Some(&expires_at) = locks.get(resource_id) {
            if now < expires_at {
                return Ok(false); // Lock is currently held
            }
        }

        // Lock is free or expired, acquire it
        locks.insert(resource_id.to_string(), now + ttl_seconds);
        Ok(true)
    }

    async fn release(&self, resource_id: &str) -> Result<(), String> {
        let mut locks = self.locks.lock().await;
        locks.remove(resource_id);
        Ok(())
    }
}

pub struct RedisLock {
    client: redis::Client,
    node_id: String, // Unique identifier for the instance holding the lock
}

impl RedisLock {
    pub fn new(client: redis::Client) -> Self {
        RedisLock {
            client,
            node_id: uuid::Uuid::new_v4().to_string(),
        }
    }
}

#[async_trait]
impl DistributedLock for RedisLock {
    async fn acquire(&self, resource_id: &str, ttl_seconds: u64) -> Result<bool, String> {
        use redis::AsyncCommands;

        // Pattern: ohc:lock:{tenant_id}:{resource_type}:{resource_id}
        // In this method we just use resource_id directly as the key
        let key = format!("ohc:lock:{}", resource_id);

        let mut conn = self.client.get_multiplexed_async_connection().await.map_err(|e| e.to_string())?;

        // SET key node_id NX EX ttl
        let result: Option<String> = redis::cmd("SET")
            .arg(&key)
            .arg(&self.node_id)
            .arg("NX")
            .arg("EX")
            .arg(ttl_seconds)
            .query_async(&mut conn)
            .await
            .map_err(|e| e.to_string())?;

        Ok(result.is_some())
    }

    async fn release(&self, resource_id: &str) -> Result<(), String> {
        let key = format!("ohc:lock:{}", resource_id);
        let mut conn = self.client.get_multiplexed_async_connection().await.map_err(|e| e.to_string())?;

        // Safe release via Lua script (only delete if node_id matches)
        let script = redis::Script::new(r#"
            if redis.call("get", KEYS[1]) == ARGV[1] then
                return redis.call("del", KEYS[1])
            else
                return 0
            end
        "#);

        let _: () = script.key(&key).arg(&self.node_id).invoke_async(&mut conn).await.map_err(|e| e.to_string())?;

        Ok(())
    }
}

pub struct HybridLock {
    inner: Arc<dyn DistributedLock>,
}

impl HybridLock {
    pub fn new(redis_url: Option<&str>, standalone: bool) -> Self {
        if standalone || redis_url.is_none() {
            return HybridLock {
                inner: Arc::new(MemoryLock::new()),
            };
        }

        if let Some(url) = redis_url {
            if let Ok(client) = redis::Client::open(url) {
                return HybridLock {
                    inner: Arc::new(RedisLock::new(client)),
                };
            }
        }

        HybridLock {
            inner: Arc::new(MemoryLock::new()),
        }
    }

    pub async fn acquire(&self, resource_id: &str, ttl_seconds: u64) -> Result<bool, String> {
        self.inner.acquire(resource_id, ttl_seconds).await
    }

    pub async fn release(&self, resource_id: &str) -> Result<(), String> {
        self.inner.release(resource_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_memory_lock() {
        let lock = MemoryLock::new();
        let res_id = "tenant_1:invoice_123";

        // Acquire lock
        assert_eq!(lock.acquire(res_id, 10).await.unwrap(), true);

        // Attempt to re-acquire (should fail)
        assert_eq!(lock.acquire(res_id, 10).await.unwrap(), false);

        // Release lock
        lock.release(res_id).await.unwrap();

        // Acquire lock again
        assert_eq!(lock.acquire(res_id, 10).await.unwrap(), true);
    }
}
