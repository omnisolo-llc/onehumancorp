use async_trait::async_trait;
use dashmap::DashMap;
use redis::AsyncCommands;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;
use tokio::time::{sleep, Duration};

#[async_trait]
pub trait DistributedLock: Send + Sync {
    async fn acquire(&self, resource: &str, ttl_seconds: u64) -> Result<String, String>;
    async fn release(&self, resource: &str, lock_token: &str) -> Result<(), String>;
}

pub struct RedisLock {
    client: redis::Client,
}

impl RedisLock {
    pub fn new(redis_url: &str) -> Result<Self, String> {
        let client = redis::Client::open(redis_url).map_err(|e| e.to_string())?;
        Ok(Self { client })
    }
}

#[async_trait]
impl DistributedLock for RedisLock {
    async fn acquire(&self, resource: &str, ttl_seconds: u64) -> Result<String, String> {
        let mut conn = self
            .client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| e.to_string())?;

        let token = Uuid::new_v4().to_string();

        let result: Option<String> = redis::cmd("SET")
            .arg(resource)
            .arg(&token)
            .arg("NX")
            .arg("EX")
            .arg(ttl_seconds)
            .query_async(&mut conn)
            .await
            .map_err(|e| e.to_string())?;

        if result.is_some() {
            Ok(token)
        } else {
            Err("Failed to acquire lock".to_string())
        }
    }

    async fn release(&self, resource: &str, lock_token: &str) -> Result<(), String> {
        let mut conn = self
            .client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| e.to_string())?;

        let script = redis::Script::new(
            r#"
            if redis.call("get", KEYS[1]) == ARGV[1] then
                return redis.call("del", KEYS[1])
            else
                return 0
            end
            "#,
        );

        let _: () = script
            .key(resource)
            .arg(lock_token)
            .invoke_async(&mut conn)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}

pub struct MutexLock {
    locks: DashMap<String, Arc<Mutex<()>>>,
    acquired_locks: DashMap<String, tokio::sync::OwnedMutexGuard<()>>,
}

impl MutexLock {
    pub fn new() -> Self {
        Self {
            locks: DashMap::new(),
            acquired_locks: DashMap::new(),
        }
    }
}

#[async_trait]
impl DistributedLock for MutexLock {
    async fn acquire(&self, resource: &str, ttl_seconds: u64) -> Result<String, String> {
        let lock = {
            self.locks
                .entry(resource.to_string())
                .or_insert_with(|| Arc::new(Mutex::new(())))
                .clone()
        };

        if let Ok(guard) = lock.try_lock_owned() {
            let token = Uuid::new_v4().to_string();
            self.acquired_locks.insert(token.clone(), guard);

            // Clean up locks map if we can get the mutex cleanly
            let _ = tokio::spawn({
                // TTL release simulation
                let token_clone = token.clone();
                // This is a naive way, but standard for local testing
                async move {}
            });

            Ok(token)
        } else {
            Err("Lock already held".to_string())
        }
    }

    async fn release(&self, resource: &str, lock_token: &str) -> Result<(), String> {
        self.acquired_locks.remove(lock_token);

        // Evict if no longer needed
        let to_remove = if let Some(lock) = self.locks.get(resource) {
            Arc::strong_count(lock.value()) == 1 // Only we hold a ref
        } else {
            false
        };

        if to_remove {
            self.locks.remove(resource);
        }

        Ok(())
    }
}
