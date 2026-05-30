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
    multiplexed_conn: tokio::sync::OnceCell<redis::aio::MultiplexedConnection>,
    fallback_lock: StandaloneLock,
}

impl RedisLock {
    pub fn new(client: redis::Client) -> Self {
        Self {
            client,
            multiplexed_conn: tokio::sync::OnceCell::new(),
            fallback_lock: StandaloneLock::new(),
        }
    }
}

#[async_trait::async_trait]
impl DistributedLock for RedisLock {
    async fn acquire(&self, task_id: &str) -> Result<LockGuard, String> {
        let conn_res = self.multiplexed_conn.get_or_try_init(|| async {
            self.client.get_multiplexed_async_connection().await
        }).await;

        match conn_res {
            Ok(conn) => {
                let mut conn_clone = conn.clone();
                let key = format!("ohc:lock:task:{}", task_id);

                let cmd_res: redis::RedisResult<bool> = redis::cmd("SET")
                    .arg(&key)
                    .arg("1")
                    .arg("NX")
                    .arg("EX")
                    .arg(5)
                    .query_async(&mut conn_clone)
                    .await;

                match cmd_res {
                    Ok(acquired) => {
                        if !acquired {
                            return Err("failed to acquire redis lock".to_string());
                        }

                        Ok(LockGuard {
                            _local_guard: None,
                            redis_client: Some(self.client.clone()),
                            redis_key: Some(key),
                        })
                    }
                    Err(_) => {
                        // Fallback on redis command error
                        self.fallback_lock.acquire(task_id).await
                    }
                }
            }
            Err(_) => {
                // Fallback on connection error
                self.fallback_lock.acquire(task_id).await
            }
        }
    }
}
