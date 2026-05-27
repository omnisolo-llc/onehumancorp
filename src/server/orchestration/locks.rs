use std::sync::Arc;
use tokio::sync::{Mutex, OwnedMutexGuard};
use std::collections::HashMap;
use tokio::time::{sleep, Duration};

#[async_trait::async_trait]
pub trait DistributedLock: Send + Sync {
    async fn acquire(&self, task_id: &str) -> Result<LockGuard, String>;
}

pub struct LockGuard {
    _local_guard: Option<OwnedMutexGuard<()>>,
    redis_client: Option<redis::Client>,
    redis_key: Option<String>,
}

impl LockGuard {
    pub async fn release(mut self) {
        if let (Some(client), Some(key)) = (&self.redis_client, &self.redis_key) {
            if let Ok(mut conn) = client.get_multiplexed_async_connection().await {
                let _: redis::RedisResult<()> = redis::cmd("DEL").arg(key).query_async(&mut conn).await;
            }
            self.redis_client = None;
            self.redis_key = None;
        }
    }
}

impl Drop for LockGuard {
    fn drop(&mut self) {
        if let (Some(client), Some(key)) = (&self.redis_client, &self.redis_key) {
            // Synchronous fallback cleanup in drop to avoid tokio::spawn panics
            // if we are outside of a runtime context, but prefer calling `release()` explicitly.
            if let Ok(mut conn) = client.get_connection() {
                let _: redis::RedisResult<()> = redis::cmd("DEL").arg(key).query(&mut conn);
            }
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
    multiplexed_conn: tokio::sync::OnceCell<redis::aio::MultiplexedConnection>,
}

impl RedisLock {
    pub fn new(client: redis::Client) -> Self {
        Self { client, multiplexed_conn: tokio::sync::OnceCell::new() }
    }
}

#[async_trait::async_trait]
impl DistributedLock for RedisLock {
    async fn acquire(&self, task_id: &str) -> Result<LockGuard, String> {
        let mut conn = self.multiplexed_conn.get_or_try_init(|| async {
            self.client.get_multiplexed_async_connection().await
        }).await.map_err(|e| e.to_string())?.clone();
        let key = format!("ohc:lock:task:{}", task_id);

        let mut retries = 5;
        let mut acquired = false;

        while retries > 0 {
            acquired = redis::cmd("SET")
                .arg(&key)
                .arg("1")
                .arg("NX")
                .arg("EX")
                .arg(5)
                .query_async(&mut conn)
                .await
                .unwrap_or(false);

            if acquired {
                break;
            }

            sleep(Duration::from_millis(100)).await;
            retries -= 1;
        }

        if !acquired {
            return Err("failed to acquire redis lock after retries".to_string());
        }

        Ok(LockGuard {
            _local_guard: None,
            redis_client: Some(self.client.clone()),
            redis_key: Some(key),
        })
    }
}
