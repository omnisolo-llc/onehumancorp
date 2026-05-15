use dashmap::DashMap;
use serde::{de::DeserializeOwned, Serialize};
use std::sync::OnceLock;
use std::time::{Duration, Instant};

pub struct HybridCache<T> {
    local: OnceLock<DashMap<String, (T, Instant)>>,
    redis_client: Option<redis::Client>,
    redis_conn: tokio::sync::OnceCell<redis::aio::MultiplexedConnection>,
    max_local_capacity: usize,
}

impl<T> HybridCache<T>
where
    T: Serialize + DeserializeOwned + Clone + Send + Sync + 'static,
{
    pub fn new(redis_client: Option<redis::Client>) -> Self {
        Self {
            local: OnceLock::new(),
            redis_client,
            redis_conn: tokio::sync::OnceCell::new(),
            max_local_capacity: 1000,
        }
    }

    pub fn with_capacity(redis_client: Option<redis::Client>, capacity: usize) -> Self {
        Self {
            local: OnceLock::new(),
            redis_client,
            redis_conn: tokio::sync::OnceCell::new(),
            max_local_capacity: capacity,
        }
    }

    async fn get_redis_conn(&self) -> Option<redis::aio::MultiplexedConnection> {
        if let Some(client) = &self.redis_client {
            let conn = self
                .redis_conn
                .get_or_try_init(|| async { client.get_multiplexed_tokio_connection().await })
                .await
                .ok()?;
            Some(conn.clone())
        } else {
            None
        }
    }

    fn get_local(&self) -> &DashMap<String, (T, Instant)> {
        self.local.get_or_init(|| DashMap::new())
    }

    pub async fn get(&self, key: &str) -> Option<T> {
        // 1. Check local cache (DashMap provides thread-safe access without manual RwLock)
        if let Some(entry) = self.get_local().get(key) {
            let (val, expiry) = entry.value();
            if Instant::now() < *expiry {
                return Some(val.clone());
            }
        }
        // If we found an expired entry, we could remove it here,
        // but the set_local logic handles capacity-based eviction.

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
        let local = self.get_local();
        if local.len() >= self.max_local_capacity && !local.contains_key(key) {
            // Sampling-based O(1) eviction
            let mut sample = Vec::with_capacity(5);
            let mut keys_to_remove = Vec::new();
            let now = Instant::now();

            // Check a few entries for natural expiration
            let mut count = 0;
            for entry in local.iter() {
                let expiry = entry.value().1;
                if expiry <= now {
                    keys_to_remove.push(entry.key().clone());
                } else {
                    sample.push((entry.key().clone(), expiry));
                }
                count += 1;
                if count >= 20 { break; }
            }

            for k in keys_to_remove {
                local.remove(&k);
            }

            // If still over capacity, evict the oldest from our sample
            if local.len() >= self.max_local_capacity && !sample.is_empty() {
                sample.sort_by_key(|s| s.1);
                if let Some((k_to_evict, _)) = sample.first() {
                    local.remove(k_to_evict);
                }
            }
        }
        local.insert(key.to_string(), (value, Instant::now() + ttl));
    }

    pub async fn invalidate(&self, key: &str) {
        self.get_local().remove(key);
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
        let cache = HybridCache::<String>::with_capacity(None, 2);
        cache.set("k1", "v1".to_string(), Duration::from_secs(60)).await;
        cache.set("k2", "v2".to_string(), Duration::from_secs(60)).await;
        cache.set("k3", "v3".to_string(), Duration::from_secs(60)).await;

        let local = cache.get_local();
        assert_eq!(local.len(), 2);
    }
}
