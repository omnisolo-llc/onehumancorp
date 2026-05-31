use std::sync::Arc;
use tokio::sync::{Mutex, OwnedMutexGuard};
use std::collections::HashMap;

#[async_trait::async_trait]
pub trait DistributedLock: Send + Sync {
    async fn acquire(&self, task_id: &str) -> Result<LockGuard, String>;
}

pub struct LockGuard {
    _local_guard: Option<OwnedMutexGuard<()>>,
    redis_client: Option<redis::Client>,
    redis_key: Option<String>,
    standalone_map: Option<Arc<std::sync::Mutex<HashMap<String, Arc<Mutex<()>>>>>>,
    standalone_key: Option<String>,
}

impl Drop for LockGuard {
    fn drop(&mut self) {
        // Drop local guard first to free the mutex
        self._local_guard.take();

        if let (Some(map_arc), Some(key)) = (&self.standalone_map, &self.standalone_key) {
            if let Ok(mut map) = map_arc.lock() {
                if let Some(arc_mutex) = map.get(key) {
                    if Arc::strong_count(arc_mutex) == 1 {
                        map.remove(key);
                    }
                }
            }
        }
        if let (Some(client), Some(key)) = (&self.redis_client, &self.redis_key) {
            let mut conn = client.get_connection().unwrap();
            let _: redis::RedisResult<()> = redis::cmd("DEL").arg(key).query(&mut conn);
        }
    }
}

pub struct StandaloneLock {
    locks: Arc<std::sync::Mutex<HashMap<String, Arc<Mutex<()>>>>>,
}

impl StandaloneLock {
    pub fn new() -> Self {
        Self {
            locks: Arc::new(std::sync::Mutex::new(HashMap::new())),
        }
    }
}

#[async_trait::async_trait]
impl DistributedLock for StandaloneLock {
    async fn acquire(&self, task_id: &str) -> Result<LockGuard, String> {
        let task_mutex = {
            let mut locks = self.locks.lock().unwrap();
            locks.entry(task_id.to_string())
                .or_insert_with(|| Arc::new(Mutex::new(())))
                .clone()
        };

        let guard = task_mutex.lock_owned().await;
        Ok(LockGuard {
            _local_guard: Some(guard),
            redis_client: None,
            redis_key: None,
            standalone_map: Some(self.locks.clone()),
            standalone_key: Some(task_id.to_string()),
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
            standalone_map: None,
            standalone_key: None,
        })
    }
}
