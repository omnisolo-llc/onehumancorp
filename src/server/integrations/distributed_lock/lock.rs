use async_trait::async_trait;
use redis::AsyncCommands;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use uuid::Uuid;
use tracing::{info, debug, error};

#[async_trait]
pub trait LockManagerInterface: Send + Sync {
    async fn acquire_lock(&self, organization_id: &str, resource: &str, ttl: Duration) -> Result<Option<String>, String>;
    async fn release_lock(&self, organization_id: &str, resource: &str, token: &str) -> Result<bool, String>;
}

pub struct LockManager {
    is_cloud: bool,
    redis_client: Option<redis::Client>,
    local_locks: Arc<Mutex<HashMap<String, String>>>,
}

impl LockManager {
    pub fn new() -> Result<Self, String> {
        let is_cloud = std::env::var("OHC_MULTITENANT").unwrap_or_else(|_| "false".to_string()) == "true";

        let redis_client = if is_cloud {
            let redis_url = std::env::var("OHC_REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());
            Some(redis::Client::open(redis_url).map_err(|e| e.to_string())?)
        } else {
            None
        };

        Ok(Self {
            is_cloud,
            redis_client,
            local_locks: Arc::new(Mutex::new(HashMap::new())),
        })
    }
}

#[async_trait]
impl LockManagerInterface for LockManager {
    async fn acquire_lock(&self, organization_id: &str, resource: &str, ttl: Duration) -> Result<Option<String>, String> {
        let key = if self.is_cloud {
            format!("ohc:lock:{}:{}", organization_id, resource)
        } else {
            resource.to_string()
        };

        let token = Uuid::new_v4().to_string();

        if self.is_cloud {
            if let Some(client) = &self.redis_client {
                let mut conn = client.get_multiplexed_tokio_connection().await.map_err(|e| e.to_string())?;

                let ttl_secs = ttl.as_secs();
                let acquired: bool = redis::cmd("SET")
                    .arg(&key)
                    .arg(&token)
                    .arg("NX")
                    .arg("EX")
                    .arg(ttl_secs)
                    .query_async(&mut conn)
                    .await
                    .map_err(|e| e.to_string())?;

                if acquired {
                    debug!("Acquired cloud lock for key: {}", key);
                    return Ok(Some(token));
                } else {
                    debug!("Failed to acquire cloud lock for key: {}", key);
                    return Ok(None);
                }
            } else {
                return Err("Cloud mode enabled but redis client not initialized".to_string());
            }
        } else {
            let mut locks = self.local_locks.lock().await;
            if locks.contains_key(&key) {
                debug!("Failed to acquire local lock for key: {}", key);
                Ok(None)
            } else {
                locks.insert(key.clone(), token.clone());
                debug!("Acquired local lock for key: {}", key);

                // For standalone, we simulate TTL by spawning a task to remove it
                let local_locks_clone = self.local_locks.clone();
                let key_clone = key.clone();
                let token_clone = token.clone();
                tokio::spawn(async move {
                    tokio::time::sleep(ttl).await;
                    let mut locks = local_locks_clone.lock().await;
                    // Only remove if the token matches
                    if let Some(current_token) = locks.get(&key_clone) {
                        if current_token == &token_clone {
                            locks.remove(&key_clone);
                            debug!("Local lock expired for key: {}", key_clone);
                        }
                    }
                });

                Ok(Some(token))
            }
        }
    }

    async fn release_lock(&self, organization_id: &str, resource: &str, token: &str) -> Result<bool, String> {
        let key = if self.is_cloud {
            format!("ohc:lock:{}:{}", organization_id, resource)
        } else {
            resource.to_string()
        };

        if self.is_cloud {
            if let Some(client) = &self.redis_client {
                let mut conn = client.get_multiplexed_tokio_connection().await.map_err(|e| e.to_string())?;

                let script = redis::Script::new(
                    r#"
                    if redis.call("get", KEYS[1]) == ARGV[1] then
                        return redis.call("del", KEYS[1])
                    else
                        return 0
                    end
                    "#,
                );

                let result: i32 = script
                    .key(&key)
                    .arg(token)
                    .invoke_async(&mut conn)
                    .await
                    .map_err(|e| e.to_string())?;

                let success = result == 1;
                debug!("Released cloud lock for key: {}, success: {}", key, success);
                Ok(success)
            } else {
                Err("Cloud mode enabled but redis client not initialized".to_string())
            }
        } else {
            let mut locks = self.local_locks.lock().await;
            if let Some(current_token) = locks.get(&key) {
                if current_token == token {
                    locks.remove(&key);
                    debug!("Released local lock for key: {}", key);
                    Ok(true)
                } else {
                    debug!("Failed to release local lock for key: {} (token mismatch)", key);
                    Ok(false)
                }
            } else {
                debug!("Failed to release local lock for key: {} (not found)", key);
                Ok(false)
            }
        }
    }
}
