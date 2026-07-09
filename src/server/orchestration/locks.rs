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
    sqlite_pool: Option<sqlx::SqlitePool>,
    released: bool,
}

impl LockGuard {
    pub async fn release(&mut self) {
        if self.released {
            return;
        }
        self.released = true;
        if let (Some(client), Some(key), Some(val)) = (&self.redis_client, &self.redis_key, &self.redis_val) {
            if let Ok(mut conn) = client.get_multiplexed_async_connection().await {
                let script = redis::Script::new(
                    r#"
                    if redis.call("get", KEYS[1]) == ARGV[1] then
                        return redis.call("del", KEYS[1])
                    else
                        return 0
                    end
                    "#,
                );
                let _ = script.key(key).arg(val).invoke_async::<()>(&mut conn).await;
            }
        } else if let (Some(pool), Some(key), Some(val)) = (&self.sqlite_pool, &self.redis_key, &self.redis_val) {
            let _ = sqlx::query("DELETE FROM distributed_locks WHERE id = $1 AND lock_val = $2")
                .bind(key)
                .bind(val)
                .execute(pool)
                .await;
        }
        self._local_guard.take();
    }
}

impl Drop for LockGuard {
    fn drop(&mut self) {
        if !self.released {
            if self.redis_client.is_some() || self.sqlite_pool.is_some() {
                // Warning: Dropped without release. The distributed lock will expire via TTL naturally.
            }
        }
    }
}

pub struct StandaloneLock {
    pool: Option<sqlx::SqlitePool>,
    pub local_locks: Mutex<HashMap<String, Arc<Mutex<()>>>>,
}

impl StandaloneLock {
    pub fn new() -> Self {
        Self {
            pool: None,
            local_locks: Mutex::new(HashMap::new()),
        }
    }

    pub fn with_pool(pool: sqlx::SqlitePool) -> Self {
        Self {
            pool: Some(pool),
            local_locks: Mutex::new(HashMap::new()),
        }
    }

    async fn do_acquire(&self, key: &str) -> Result<LockGuard, String> {
        let val = uuid::Uuid::new_v4().to_string();

        if let Some(pool) = &self.pool {
            // First cleanup expired locks
            let _ = sqlx::query("DELETE FROM distributed_locks WHERE expires_at < CURRENT_TIMESTAMP")
                .execute(pool)
                .await;

            let result = sqlx::query("INSERT INTO distributed_locks (id, lock_val, expires_at) VALUES ($1, $2, datetime('now', '+15 seconds'))")
                .bind(key)
                .bind(&val)
                .execute(pool)
                .await;

            if result.is_ok() {
                return Ok(LockGuard {
                    _local_guard: None,
                    redis_client: None,
                    redis_key: Some(key.to_string()),
                    redis_val: Some(val),
                    sqlite_pool: Some(pool.clone()),
                    released: false,
                });
            } else {
                return Err("Failed to acquire SQLite lock".to_string());
            }
        }

        // Fallback to local memory lock if no pool is provided
        let task_mutex = {
            let mut locks = self.local_locks.lock().await;
            locks.entry(key.to_string())
                .or_insert_with(|| Arc::new(Mutex::new(())))
                .clone()
        };

        let guard = task_mutex.lock_owned().await;
        Ok(LockGuard {
            _local_guard: Some(guard),
            redis_client: None,
            redis_key: None,
            redis_val: None,
            sqlite_pool: None,
            released: false,
        })
    }
}

#[async_trait::async_trait]
impl DistributedLock for StandaloneLock {
    async fn acquire(&self, task_id: &str) -> Result<LockGuard, String> {
        self.do_acquire(&format!("ohc:lock:task:{}", task_id)).await
    }

    async fn acquire_resource(&self, tenant_id: &str, resource_type: &str, resource_id: &str) -> Result<LockGuard, String> {
        self.do_acquire(&format!("ohc:lock:{}:{}:{}", tenant_id, resource_type, resource_id)).await
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
            sqlite_pool: None,
            released: false,
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
            sqlite_pool: None,
            released: false,
        })
    }
}
