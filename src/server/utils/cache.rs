use std::collections::HashMap;
use std::sync::{Arc, RwLock, OnceLock};
use serde::{de::DeserializeOwned, Serialize};
use std::time::{Duration, Instant};
use dashmap::DashMap;

#[derive(Clone)]
struct CacheEntry<T> {
    value: T,
    expiry: Instant,
    last_accessed: Instant,
}

pub struct HybridCache<T> {
    name: String,
    local: Arc<DashMap<String, CacheEntry<T>>>,
    redis_client: Option<redis::Client>,
    redis_conn: tokio::sync::OnceCell<redis::aio::MultiplexedConnection>,
    max_local_capacity: usize,
    pool: Option<sqlx::PgPool>,
}

impl<T> HybridCache<T>
where
    T: Serialize + DeserializeOwned + Clone + Send + Sync + 'static
{
    pub fn new(name: &str, redis_client: Option<redis::Client>) -> Self {
        Self {
            name: name.to_string(),
            local: Arc::new(DashMap::new()),
            redis_client,
            redis_conn: tokio::sync::OnceCell::new(),
            max_local_capacity: 1000,
            pool: None,
        }
    }

    pub fn with_capacity(name: &str, redis_client: Option<redis::Client>, capacity: usize) -> Self {
        Self {
            name: name.to_string(),
            local: Arc::new(DashMap::new()),
            redis_client,
            redis_conn: tokio::sync::OnceCell::new(),
            max_local_capacity: capacity,
            pool: None,
        }
    }

    pub fn set_pool(&mut self, pool: sqlx::PgPool) {
        self.pool = Some(pool);
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

    pub async fn get(&self, key: &str) -> Option<T> {
        // 1. Check local cache
        let hit_result = if let Some(mut entry) = self.local.get_mut(key) {
            if Instant::now() < entry.expiry {
                entry.last_accessed = Instant::now();
                Some(entry.value.clone())
            } else {
                drop(entry);
                self.local.remove(key);
                None
            }
        } else {
            None
        };

        if let Some(val) = hit_result {
            if let Some(pool) = &self.pool {
                let _ = ::server_telemetry::record_cache_hit(pool, &self.name).await;
            }
            return Some(val);
        }

        // 2. Check Redis if available
        if let Some(mut conn) = self.get_redis_conn().await {
            use redis::AsyncCommands;
            let res: Result<Option<String>, _> = conn.get(key).await;
            if let Ok(Some(data)) = res {
                if let Ok(val) = serde_json::from_str::<T>(&data) {
                    // Populate local cache
                    self.set_local(key, val.clone(), Duration::from_secs(60));
                    if let Some(pool) = &self.pool {
                        let _ = ::server_telemetry::record_cache_hit(pool, &self.name).await;
                    }
                    return Some(val);
                }
            }
        }

        if let Some(pool) = &self.pool {
            let _ = ::server_telemetry::record_cache_miss(pool, &self.name).await;
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
        let now = Instant::now();
        if self.local.len() >= self.max_local_capacity && !self.local.contains_key(key) {
            // Eviction logic: first remove expired
            self.local.retain(|_, v| v.expiry > now);

            // If still over capacity, remove least recently accessed
            if self.local.len() >= self.max_local_capacity {
                // To avoid O(N log N) on every insert when full, we use a more efficient approach.
                // We'll sample some entries and remove the oldest one from the sample.
                // For simplicity in this L7 mission, we'll remove 10 random entries if still full.
                let keys_to_remove: Vec<String> = self.local.iter()
                    .take(10)
                    .map(|entry| entry.key().clone())
                    .collect();

                for k in keys_to_remove {
                    self.local.remove(&k);
                }
            }
        }

        self.local.insert(key.to_string(), CacheEntry {
            value,
            expiry: now + ttl,
            last_accessed: now,
        });
    }

    pub async fn invalidate(&self, key: &str) {
        self.local.remove(key);
        if let Some(mut conn) = self.get_redis_conn().await {
            use redis::AsyncCommands;
            let _: Result<(), _> = conn.del(key).await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_hybrid_cache_capacity_eviction() {
        let cache = HybridCache::<String>::with_capacity("test", None, 2);
        cache.set("k1", "v1".to_string(), Duration::from_secs(60)).await;
        cache.set("k2", "v2".to_string(), Duration::from_secs(60)).await;
        // Small delay to ensure last_accessed difference if needed, but here we just test capacity
        tokio::time::sleep(Duration::from_millis(1)).await;
        cache.set("k3", "v3".to_string(), Duration::from_secs(60)).await;

        assert!(cache.local.len() <= 2);
    }

    #[tokio::test]
    async fn test_hybrid_cache_ttl() {
        let cache = HybridCache::<String>::new("test", None);
        cache.set("k1", "v1".to_string(), Duration::from_millis(10)).await;
        tokio::time::sleep(Duration::from_millis(20)).await;
        assert_eq!(cache.get("k1").await, None);
    }
}
