use std::collections::HashMap;
use std::sync::{Arc, RwLock, OnceLock};
use serde::{de::DeserializeOwned, Serialize};
use std::time::Duration;

pub struct HybridCache<T> {
    local: OnceLock<RwLock<HashMap<String, (T, std::time::Instant)>>>,
    redis_client: Option<redis::Client>,
    redis_conn: tokio::sync::OnceCell<redis::aio::MultiplexedConnection>,
}

impl<T> HybridCache<T>
where
    T: Serialize + DeserializeOwned + Clone + Send + Sync + 'static
{
    pub fn new(redis_client: Option<redis::Client>) -> Self {
        Self {
            local: OnceLock::new(),
            redis_client,
            redis_conn: tokio::sync::OnceCell::new(),
        }
    }

    async fn get_redis_conn(&self) -> Option<redis::aio::MultiplexedConnection> {
        if let Some(client) = &self.redis_client {
            let conn = self.redis_conn.get_or_try_init(|| async {
                client.get_multiplexed_tokio_connection().await
            }).await.ok()?;
            Some(conn.clone())
        } else {
            None
        }
    }

    fn get_local(&self) -> &RwLock<HashMap<String, (T, std::time::Instant)>> {
        self.local.get_or_init(|| RwLock::new(HashMap::new()))
    }

    pub async fn get(&self, key: &str) -> Option<T> {
        // 1. Check local cache
        {
            let guard = self.get_local().read().ok()?;
            if let Some((val, expiry)) = guard.get(key) {
                if std::time::Instant::now() < *expiry {
                    return Some(val.clone());
                }
            }
        }

        // 2. Check Redis if available
        if let Some(mut conn) = self.get_redis_conn().await {
            use redis::AsyncCommands;
            let res: Result<Option<String>, _> = conn.get(key).await;
            if let Ok(Some(data)) = res {
                if let Ok(val) = serde_json::from_str::<T>(&data) {
                    // Populate local cache
                    self.set_local(key, val.clone(), Duration::from_secs(60));
                    return Some(val);
                }
            }
        }

        None
    }

    pub async fn set(&self, key: &str, value: T, ttl: Duration) {
        // 1. Set local cache
        self.set_local(key, value.clone(), ttl);

        // 2. Set Redis if available
        if let Some(mut conn) = self.get_redis_conn().await {
            use redis::AsyncCommands;
            if let Ok(data) = serde_json::to_string(&value) {
                let _: Result<(), _> = conn.set_ex(key, data, ttl.as_secs() as u64).await;
            }
        }
    }

    fn set_local(&self, key: &str, value: T, ttl: Duration) {
        if let Ok(mut guard) = self.get_local().write() {
            guard.insert(key.to_string(), (value, std::time::Instant::now() + ttl));
        }
    }

    pub async fn invalidate(&self, key: &str) {
        if let Ok(mut guard) = self.get_local().write() {
            guard.remove(key);
        }
        if let Some(mut conn) = self.get_redis_conn().await {
            use redis::AsyncCommands;
            let _: Result<(), _> = conn.del(key).await;
        }
    }
}
