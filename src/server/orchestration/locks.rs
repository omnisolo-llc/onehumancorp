use std::sync::Arc;
use tokio::sync::{Mutex, OwnedMutexGuard};
use std::collections::HashMap;

#[async_trait::async_trait]
pub trait DistributedLock: Send + Sync {
    async fn acquire(&self, task_id: &str) -> Result<LockGuard, String>;
    async fn acquire_resource(&self, tenant_id: &str, resource_type: &str, resource_id: &str) -> Result<LockGuard, String>;
}

pub struct LockGuard {
    _local_guard: Option<OwnedMutexGuard<()>>,
    redis_client: Option<redis::Client>,
    redis_key: Option<String>,
    redis_val: Option<String>,
}

impl Drop for LockGuard {
    fn drop(&mut self) {
        if let (Some(client), Some(key), Some(val)) = (&self.redis_client, &self.redis_key, &self.redis_val) {
            if let Ok(mut conn) = client.get_connection() {
                let script = redis::Script::new(
                    r#"
                    if redis.call("get", KEYS[1]) == ARGV[1] then
                        return redis.call("del", KEYS[1])
                    else
                        return 0
                    end
                    "#,
                );
                let _: redis::RedisResult<()> = script.key(key).arg(val).invoke(&mut conn);
            }
        }
    }
}

pub struct StandaloneLock {
    pub locks: Mutex<HashMap<String, Arc<Mutex<()>>>>,
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
            redis_val: None,
        })
    }

    async fn acquire_resource(&self, tenant_id: &str, resource_type: &str, resource_id: &str) -> Result<LockGuard, String> {
        let key = format!("ohc:lock:{}:{}:{}", tenant_id, resource_type, resource_id);
        let task_mutex = {
            let mut locks = self.locks.lock().await;
            locks.entry(key.clone())
                .or_insert_with(|| Arc::new(Mutex::new(())))
                .clone()
        };

        let guard = task_mutex.lock_owned().await;
        Ok(LockGuard {
            _local_guard: Some(guard),
            redis_client: None,
            redis_key: None,
            redis_val: None,
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
        let mut conn = self.client.get_multiplexed_async_connection().await.map_err(|e| e.to_string())?;
        let key = format!("ohc:lock:task:{}", task_id);
        let val = uuid::Uuid::new_v4().to_string();

        let acquired: bool = redis::cmd("SET")
            .arg(&key)
            .arg(&val)
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
            redis_val: Some(val),
        })
    }

    async fn acquire_resource(&self, tenant_id: &str, resource_type: &str, resource_id: &str) -> Result<LockGuard, String> {
        let mut conn = self.client.get_multiplexed_async_connection().await.map_err(|e| e.to_string())?;

        let key = format!("ohc:lock:{}:{}:{}", tenant_id, resource_type, resource_id);
        let val = uuid::Uuid::new_v4().to_string();

        let acquired: bool = redis::cmd("SET")
            .arg(&key)
            .arg(&val)
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
            redis_val: Some(val),
        })
    }
}
