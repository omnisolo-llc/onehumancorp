use std::sync::Arc;
use tokio::sync::{Mutex, OwnedMutexGuard};
use std::collections::HashMap;
use std::time::Duration;

#[async_trait::async_trait]
pub trait DistributedLock: Send + Sync {
    async fn acquire(&self, task_id: &str) -> Result<LockGuard, String>;
}

pub struct LockGuard {
    _local_guard: Option<OwnedMutexGuard<()>>,
    redis_client: Option<redis::Client>,
    redis_key: Option<String>,
}

impl Drop for LockGuard {
    fn drop(&mut self) {
        if let (Some(client), Some(key)) = (&self.redis_client, &self.redis_key) {
            let mut conn = client.get_connection().unwrap();
            let _: redis::RedisResult<()> = redis::cmd("DEL").arg(key).query(&mut conn);
        }
    }
}

pub struct StandaloneLock {
    locks: Mutex<HashMap<String, Arc<Mutex<()>>>>,
}

impl StandaloneLock {
    pub fn new() -> Self {
        Self {
            locks: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait::async_trait]
impl DistributedLock for StandaloneLock {
    async fn acquire(&self, task_id: &str) -> Result<LockGuard, String> {
        let task_mutex = {
            let mut locks = self.locks.lock().await;
            locks.entry(task_id.to_string())
                .or_insert_with(|| Arc::new(Mutex::new(())))
                .clone()
        };

        let guard = task_mutex.lock_owned().await;
        Ok(LockGuard {
            _local_guard: Some(guard),
            redis_client: None,
            redis_key: None,
        })
    }
}

pub struct RedisLock {
    client: redis::Client,
}

impl RedisLock {
    pub fn new(client: redis::Client) -> Self {
        Self { client }
    }
}

#[async_trait::async_trait]
impl DistributedLock for RedisLock {
    async fn acquire(&self, task_id: &str) -> Result<LockGuard, String> {
        let mut conn = self.client.get_async_connection().await.map_err(|e| e.to_string())?;
        let key = format!("ohc:lock:task:{}", task_id);

        let acquired: bool = redis::cmd("SET")
            .arg(&key)
            .arg("1")
            .arg("NX")
            .arg("EX")
            .arg(5)
            .query_async(&mut conn)
            .await
            .unwrap_or(false);

        if !acquired {
            return Err("failed to acquire redis lock".to_string());
        }

        Ok(LockGuard {
            _local_guard: None,
            redis_client: Some(self.client.clone()),
            redis_key: Some(key),
        })
    }
}
