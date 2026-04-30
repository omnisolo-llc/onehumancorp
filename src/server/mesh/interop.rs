use std::time::Duration;
use uuid::Uuid;
use std::path::PathBuf;
use std::fs::{OpenOptions};
use tokio::time::sleep;

pub enum HybridLock {
    Cloud {
        client: redis::Client,
        key: String,
        value: String,
    },
    Standalone {
        file_path: PathBuf,
        value: String,
    },
}

impl HybridLock {
    pub fn new_cloud(client: redis::Client, key: &str) -> Self {
        HybridLock::Cloud {
            client,
            key: format!("lock:{}", key),
            value: Uuid::new_v4().to_string(),
        }
    }

    pub fn new_standalone(key: &str) -> Self {
        let mut dir = std::env::temp_dir();
        dir.push("ohc-locks");
        std::fs::create_dir_all(&dir).unwrap_or_default();
        dir.push(format!("lock_{}.lock", key));
        HybridLock::Standalone {
            file_path: dir,
            value: Uuid::new_v4().to_string(),
        }
    }

    pub async fn acquire(&self, timeout: Duration, expiration: Duration) -> Result<(), String> {
        match self {
            HybridLock::Cloud { client, key, value } => {
                let mut con = client.get_multiplexed_async_connection().await.map_err(|e| e.to_string())?;
                let start = std::time::Instant::now();
                loop {
                    if start.elapsed() > timeout {
                        return Err("timeout acquiring lock".to_string());
                    }
                    let res: Option<String> = redis::cmd("SET")
                        .arg(key)
                        .arg(value)
                        .arg("NX")
                        .arg("PX")
                        .arg(expiration.as_millis() as u64)
                        .query_async(&mut con)
                        .await
                        .map_err(|e| e.to_string())?;
                    if res.is_some() {
                        return Ok(());
                    }
                    sleep(Duration::from_millis(100)).await;
                }
            }
            HybridLock::Standalone { file_path, value } => {
                let start = std::time::Instant::now();
                loop {
                    if start.elapsed() > timeout {
                        return Err("timeout acquiring lock".to_string());
                    }
                    let file = OpenOptions::new()
                        .write(true)
                        .create_new(true)
                        .open(file_path);

                    if let Ok(mut f) = file {
                        use std::io::Write;
                        let _ = f.write_all(value.as_bytes());
                        return Ok(());
                    }
                    sleep(Duration::from_millis(100)).await;
                }
            }
        }
    }

    pub async fn release(&self) -> Result<(), String> {
        match self {
            HybridLock::Cloud { client, key, value } => {
                let mut con = client.get_multiplexed_async_connection().await.map_err(|e| e.to_string())?;
                let script = redis::Script::new(r#"
                    if redis.call("get", KEYS[1]) == ARGV[1] then
                        return redis.call("del", KEYS[1])
                    else
                        return 0
                    end
                "#);
                let res: i32 = script.key(key).arg(value).invoke_async(&mut con).await.map_err(|e| e.to_string())?;
                if res == 1 { Ok(()) } else { Err("failed to release lock: not owner or lock expired".to_string()) }
            }
            HybridLock::Standalone { file_path, value } => {
                if let Ok(content) = std::fs::read_to_string(file_path) {
                    if content == *value {
                        let _ = std::fs::remove_file(file_path);
                        return Ok(());
                    }
                }
                Err("failed to release lock: not owner".to_string())
            }
        }
    }
}

pub struct StateHandoff;

impl StateHandoff {
    pub fn new() -> Self {
        StateHandoff
    }

    pub async fn sync_cloud_to_standalone(&self, _tenant_id: &str) -> Result<(), String> {
        Ok(())
    }

    pub async fn sync_standalone_to_cloud(&self, _tenant_id: &str) -> Result<(), String> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_standalone_lock() {
        let lock = HybridLock::new_standalone("test_resource_1");
        assert!(lock.acquire(Duration::from_secs(1), Duration::from_secs(5)).await.is_ok());

        let lock2 = HybridLock::new_standalone("test_resource_1");
        assert!(lock2.acquire(Duration::from_millis(200), Duration::from_secs(5)).await.is_err());

        assert!(lock.release().await.is_ok());
        assert!(lock2.acquire(Duration::from_millis(200), Duration::from_secs(5)).await.is_ok());
        assert!(lock2.release().await.is_ok());
    }

    #[tokio::test]
    async fn test_handoff() {
        let handoff = StateHandoff::new();
        assert!(handoff.sync_cloud_to_standalone("tenant_1").await.is_ok());
        assert!(handoff.sync_standalone_to_cloud("tenant_1").await.is_ok());
    }
}
