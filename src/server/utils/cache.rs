use std::collections::HashMap;
use std::sync::{Arc, RwLock, OnceLock};
use tokio::sync::OnceCell;
use serde::{de::DeserializeOwned, Serialize};
use std::time::Duration;

pub struct HybridCache<T> {
    local: OnceLock<RwLock<HashMap<String, (T, std::time::Instant)>>>,
    redis_client: Option<redis::Client>,
    redis_conn: OnceCell<redis::aio::MultiplexedConnection>,
}

impl<T> HybridCache<T>
where
    T: Serialize + DeserializeOwned + Clone + Send + Sync + 'static
{
    pub fn new(redis_client: Option<redis::Client>) -> Self {
        Self {
            local: OnceLock::new(),
            redis_client,
            redis_conn: OnceCell::new(),
        }
    }

    fn get_local(&self) -> &RwLock<HashMap<String, (T, std::time::Instant)>> {
        self.local.get_or_init(|| RwLock::new(HashMap::new()))
    }


    async fn get_connection(&self) -> Result<redis::aio::MultiplexedConnection, String> {
        let client = self.redis_client.as_ref().ok_or("No redis client")?;
        let conn = self.redis_conn.get_or_try_init(|| async {
            client.get_multiplexed_tokio_connection().await
        }).await.map_err(|e| e.to_string())?;
        Ok(conn.clone())
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
        if self.redis_client.is_some() {
            if let Ok(mut conn) = self.get_connection().await {
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
        }

        None
    }

    pub async fn set(&self, key: &str, value: T, ttl: Duration) {
        // 1. Set local cache
        self.set_local(key, value.clone(), ttl);

        // 2. Set Redis if available
        if self.redis_client.is_some() {
            if let Ok(mut conn) = self.get_connection().await {
                use redis::AsyncCommands;
                if let Ok(data) = serde_json::to_string(&value) {
                    let _: Result<(), _> = conn.set_ex(key, data, ttl.as_secs() as u64).await;
                }
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
        if self.redis_client.is_some() {
            if let Ok(mut conn) = self.get_connection().await {
                use redis::AsyncCommands;
                let _: Result<(), _> = conn.del(key).await;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_hybrid_cache_get_connection() {
        if let Ok(redis_url) = std::env::var("REDIS_URL") {
            if let Ok(client) = redis::Client::open(redis_url) {
                let cache = HybridCache::<String>::new(Some(client));
                let conn1 = cache.get_connection().await;
                assert!(conn1.is_ok());
                let conn2 = cache.get_connection().await;
                assert!(conn2.is_ok());
            }
        }
    }
}
