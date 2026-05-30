use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};
use serde::{de::DeserializeOwned, Serialize};
use std::time::Duration;
use dashmap::DashMap;
use std::sync::Arc;

pub struct HybridCache<T> {
    local: OnceLock<RwLock<HashMap<String, (T, std::time::Instant)>>>,
    redis_client: Option<redis::Client>,
    redis_conn: tokio::sync::OnceCell<redis::aio::MultiplexedConnection>,
    max_local_capacity: usize,
    locks: DashMap<String, Arc<tokio::sync::Mutex<()>>>,
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
            max_local_capacity: 1000,
            locks: DashMap::new(),
        }
    }

    pub fn with_capacity(redis_client: Option<redis::Client>, capacity: usize) -> Self {
        Self {
            local: OnceLock::new(),
            redis_client,
            redis_conn: tokio::sync::OnceCell::new(),
            max_local_capacity: capacity,
            locks: DashMap::new(),
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
            if guard.len() >= self.max_local_capacity && !guard.contains_key(key) {
                let now = std::time::Instant::now();
                let keys_to_remove: Vec<String> = guard.iter()
                    .filter(|(_, (_, expiry))| *expiry <= now)
                    .map(|(k, _)| k.clone())
                    .collect();

                for k in keys_to_remove {
                    guard.remove(&k);
                }

                if guard.len() >= self.max_local_capacity {
                    if let Some(k) = guard.keys().next().cloned() {
                        guard.remove(&k);
                    }
                }
            }
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

    pub async fn get_or_insert_with<F, Fut, E>(&self, key: &str, fetch: F, ttl: Duration) -> Result<T, E>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<T, E>>,
    {
        // Fast path
        if let Some(val) = self.get(key).await {
            return Ok(val);
        }

        let lock = {
            let ref_mut = self.locks.entry(key.to_string()).or_insert_with(|| Arc::new(tokio::sync::Mutex::new(())));
            ref_mut.clone()
        };

        let _guard = lock.lock().await;

        // Double check
        if let Some(val) = self.get(key).await {
            return Ok(val);
        }

        // Fetch
        let val = match fetch().await {
            Ok(v) => v,
            Err(e) => {
                self.locks.remove(key);
                return Err(e);
            }
        };

        self.set(key, val.clone(), ttl).await;
        self.locks.remove(key);

        Ok(val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[tokio::test]
    async fn test_hybrid_cache_capacity_eviction() {
        let cache = HybridCache::<String>::with_capacity(None, 2);
        cache.set("k1", "v1".to_string(), Duration::from_secs(60)).await;
        cache.set("k2", "v2".to_string(), Duration::from_secs(60)).await;
        cache.set("k3", "v3".to_string(), Duration::from_secs(60)).await;

        let local = cache.get_local().read().unwrap();
        assert_eq!(local.len(), 2);
    }

    #[tokio::test]
    async fn test_get_or_insert_with_concurrency() {
        let cache = Arc::new(HybridCache::<String>::with_capacity(None, 10));
        let counter = Arc::new(AtomicUsize::new(0));

        let mut handles = vec![];
        for _ in 0..10 {
            let cache_clone = cache.clone();
            let counter_clone = counter.clone();

            handles.push(tokio::spawn(async move {
                let res = cache_clone.get_or_insert_with("concurrent_key", || async {
                    counter_clone.fetch_add(1, Ordering::SeqCst);
                    tokio::time::sleep(Duration::from_millis(50)).await;
                    Ok::<_, String>("value".to_string())
                }, Duration::from_secs(60)).await;
                assert_eq!(res.unwrap(), "value");
            }));
        }

        for handle in handles {
            handle.await.unwrap();
        }

        // The fetch closure should only be executed once despite 10 concurrent requests
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }
}
