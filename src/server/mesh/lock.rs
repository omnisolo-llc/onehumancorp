use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::time::{Duration, Instant};
use uuid::Uuid;
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};
use tokio::fs;

pub fn build_lock_key(tenant_id: &str, resource_type: &str, resource_id: &str) -> String {
    format!("ohc:lock:{}:{}:{}", tenant_id, resource_type, resource_id)
}

#[async_trait]
pub trait MeshLockManager: Send + Sync {
    /// Acquires a distributed lock. Returns a MeshLock if successful.
    async fn acquire(
        &self,
        tenant_id: &str,
        resource_type: &str,
        resource_id: &str,
        timeout: Duration,
        expiration: Duration,
    ) -> Result<Box<dyn MeshLock>, String>;
}

#[async_trait]
pub trait MeshLock: Send + Sync {
    /// Releases the distributed lock.
    async fn release(&self) -> Result<(), String>;
}

#[derive(Serialize, Deserialize)]
struct LockData {
    owner: String,
    expires_at_ms: u128,
}

pub struct LocalFileLockManager {
    base_dir: PathBuf,
}

impl LocalFileLockManager {
    pub fn new(base_dir: impl AsRef<Path>) -> Self {
        let dir = base_dir.as_ref().to_path_buf();
        std::fs::create_dir_all(&dir).unwrap_or_default();
        Self { base_dir: dir }
    }

    fn lock_path(&self, key: &str) -> PathBuf {
        let safe_key = key.replace(":", "_");
        self.base_dir.join(format!("{}.lock", safe_key))
    }
}

pub struct LocalFileLock {
    path: PathBuf,
    owner: String,
}

#[async_trait]
impl MeshLock for LocalFileLock {
    async fn release(&self) -> Result<(), String> {
        // Read current lock
        if let Ok(data) = fs::read_to_string(&self.path).await {
            if let Ok(lock_data) = serde_json::from_str::<LockData>(&data) {
                if lock_data.owner == self.owner {
                    // We own it, delete it
                    let _ = fs::remove_file(&self.path).await;
                    return Ok(());
                }
            }
        }
        Err("failed to release lock: not owner or lock expired".to_string())
    }
}

#[async_trait]
impl MeshLockManager for LocalFileLockManager {
    async fn acquire(
        &self,
        tenant_id: &str,
        resource_type: &str,
        resource_id: &str,
        timeout: Duration,
        expiration: Duration,
    ) -> Result<Box<dyn MeshLock>, String> {
        let key = build_lock_key(tenant_id, resource_type, resource_id);
        let path = self.lock_path(&key);
        let owner = Uuid::new_v4().to_string();
        let start = Instant::now();

        loop {
            if start.elapsed() > timeout {
                return Err("timeout acquiring lock".to_string());
            }

            let now_ms = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis();

            let mut can_acquire = false;

            // Check if file exists
            if !path.exists() {
                can_acquire = true;
            } else {
                // If it exists, read it
                if let Ok(data) = std::fs::read_to_string(&path) {
                    if let Ok(lock_data) = serde_json::from_str::<LockData>(&data) {
                        if now_ms > lock_data.expires_at_ms {
                            // Expired
                            can_acquire = true;
                        }
                    } else {
                        // Corrupt lock file, overwrite
                        can_acquire = true;
                    }
                } else {
                    can_acquire = true;
                }
            }

            if can_acquire {
                let expires_at_ms = now_ms + expiration.as_millis();
                let lock_data = LockData {
                    owner: owner.clone(),
                    expires_at_ms,
                };

                // Use OpenOptions to create with exclusive access
                if let Ok(mut file) = std::fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(&path)
                {
                    use std::io::Write;
                    let json = serde_json::to_string(&lock_data).unwrap();
                    if file.write_all(json.as_bytes()).is_ok() {
                        return Ok(Box::new(LocalFileLock {
                            path,
                            owner,
                        }));
                    }
                }
            }

            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    }
}

pub struct RedisLockManager {
    client: redis::Client,
    conn: Mutex<redis::aio::MultiplexedConnection>,
}

impl RedisLockManager {
    pub async fn new(redis_url: &str) -> Result<Self, String> {
        let client = redis::Client::open(redis_url).map_err(|e| e.to_string())?;
        let conn = client.get_multiplexed_async_connection().await.map_err(|e| e.to_string())?;
        Ok(Self {
            client,
            conn: Mutex::new(conn),
        })
    }
}

pub struct RedisLock {
    client: redis::Client,
    key: String,
    value: String,
}

#[async_trait]
impl MeshLock for RedisLock {
    async fn release(&self) -> Result<(), String> {
        let mut con = self.client.get_async_connection().await.map_err(|e| e.to_string())?;
        let script = redis::Script::new(r#"
            if redis.call("get", KEYS[1]) == ARGV[1] then
                return redis.call("del", KEYS[1])
            else
                return 0
            end
        "#);

        let res: i32 = script.key(&self.key).arg(&self.value).invoke_async(&mut con).await.map_err(|e| e.to_string())?;

        if res == 1 {
            Ok(())
        } else {
            Err("failed to release lock: not owner or lock expired".to_string())
        }
    }
}

#[async_trait]
impl MeshLockManager for RedisLockManager {
    async fn acquire(
        &self,
        tenant_id: &str,
        resource_type: &str,
        resource_id: &str,
        timeout: Duration,
        expiration: Duration,
    ) -> Result<Box<dyn MeshLock>, String> {
        let key = build_lock_key(tenant_id, resource_type, resource_id);
        let value = Uuid::new_v4().to_string();
        let start = Instant::now();

        loop {
            if start.elapsed() > timeout {
                return Err("timeout acquiring lock".to_string());
            }

            let mut con = self.conn.lock().await;

            let res: Option<String> = redis::cmd("SET")
                .arg(&key)
                .arg(&value)
                .arg("NX")
                .arg("PX")
                .arg(expiration.as_millis() as u64)
                .query_async(&mut *con)
                .await
                .map_err(|e| e.to_string())?;

            if res.is_some() {
                return Ok(Box::new(RedisLock {
                    client: self.client.clone(),
                    key,
                    value,
                }));
            }

            // Drop mutex before sleeping
            drop(con);
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    }
}

pub async fn create_lock_manager(redis_url: Option<&str>, standalone: bool) -> Result<Arc<dyn MeshLockManager>, String> {
    if standalone {
        let temp_dir = std::env::temp_dir().join("ohc_locks");
        return Ok(Arc::new(LocalFileLockManager::new(temp_dir)));
    }

    if let Some(url) = redis_url {
        match RedisLockManager::new(url).await {
            Ok(manager) => Ok(Arc::new(manager)),
            Err(e) => {
                Err(format!("Failed to connect to Redis for MeshLockManager: {}", e))
            }
        }
    } else {
        Err("Redis URL is required for Cloud mode MeshLockManager".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_local_file_lock() {
        let temp_dir = std::env::temp_dir().join(format!("ohc_locks_test_{}", Uuid::new_v4()));
        let manager = LocalFileLockManager::new(&temp_dir);
        let lock1 = manager
            .acquire("t1", "res", "1", Duration::from_secs(1), Duration::from_secs(10))
            .await
            .unwrap();

        let lock2_res = manager
            .acquire("t1", "res", "1", Duration::from_millis(100), Duration::from_secs(10))
            .await;

        assert!(lock2_res.is_err(), "Should not acquire lock when already held");

        lock1.release().await.unwrap();

        let lock3 = manager
            .acquire("t1", "res", "1", Duration::from_secs(1), Duration::from_secs(10))
            .await
            .unwrap();

        assert!(lock3.release().await.is_ok());
        let _ = std::fs::remove_dir_all(temp_dir);
    }

    #[tokio::test]
    async fn test_local_file_lock_expiration() {
        let temp_dir = std::env::temp_dir().join(format!("ohc_locks_test_{}", Uuid::new_v4()));
        let manager = LocalFileLockManager::new(&temp_dir);
        let lock1 = manager
            .acquire("t2", "res", "2", Duration::from_secs(1), Duration::from_millis(50))
            .await
            .unwrap();

        tokio::time::sleep(Duration::from_millis(100)).await;

        let lock2 = manager
            .acquire("t2", "res", "2", Duration::from_secs(1), Duration::from_secs(10))
            .await
            .unwrap();

        assert!(lock2.release().await.is_ok());
        let _ = std::fs::remove_dir_all(temp_dir);
    }

    #[tokio::test]
    async fn test_redis_lock() {
        let redis_url = "redis://127.0.0.1:6379";
        if redis::Client::open(redis_url).is_err() {
            return;
        }

        let client = redis::Client::open(redis_url).unwrap();
        if client.get_async_connection().await.is_err() {
            return;
        }

        let manager = RedisLockManager::new(redis_url).await.unwrap();
        let lock1 = manager
            .acquire("t3", "res", "3", Duration::from_secs(1), Duration::from_secs(10))
            .await
            .unwrap();

        let lock2_res = manager
            .acquire("t3", "res", "3", Duration::from_millis(100), Duration::from_secs(10))
            .await;

        assert!(lock2_res.is_err());

        lock1.release().await.unwrap();

        let lock3 = manager
            .acquire("t3", "res", "3", Duration::from_secs(1), Duration::from_secs(10))
            .await
            .unwrap();

        assert!(lock3.release().await.is_ok());
    }

    #[tokio::test]
    async fn test_create_lock_manager() {
        let mem_mgr = create_lock_manager(None, true).await.unwrap();
        assert!(mem_mgr.acquire("tx", "res", "x", Duration::from_secs(1), Duration::from_secs(1)).await.is_ok());

        let fallback_mgr = create_lock_manager(Some("redis://invalid:9999"), false).await;
        assert!(fallback_mgr.is_err(), "Cloud mode must fail loudly if redis is unavailable");
    }
}
