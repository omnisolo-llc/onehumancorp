
use std::sync::OnceLock;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::time::Duration;
use dashmap::DashMap;
use dashmap::DashSet;
// use tokio::sync::broadcast; // removed unused


#[derive(Clone, Serialize, Deserialize)]
struct CacheItem<T> {
    value: T,
    tags: Vec<String>,
}

use std::sync::atomic::Ordering;


struct CacheValue<T> {
    val: T,
    expiry: std::time::Instant,
    access_count: std::sync::atomic::AtomicU64,
}

pub struct HybridCacheInner<T> {
    local: OnceLock<DashMap<String, CacheValue<T>>>,
    local_tags: OnceLock<DashMap<String, DashSet<String>>>,
    flight_group: OnceLock<DashMap<String, tokio::sync::watch::Sender<Option<T>>>>,
    redis_client: Option<redis::Client>,
    redis_conn: tokio::sync::OnceCell<redis::aio::MultiplexedConnection>,
    max_local_capacity: usize,
}

#[derive(Clone)]
pub struct HybridCache<T> {
    inner: std::sync::Arc<HybridCacheInner<T>>,
}

impl<T> HybridCache<T>
where
    T: Serialize + DeserializeOwned + Clone + Send + Sync + 'static
{
    pub fn new(redis_client: Option<redis::Client>) -> Self {
        Self { inner: std::sync::Arc::new(HybridCacheInner {
            local: OnceLock::new(),
            local_tags: OnceLock::new(),
            flight_group: OnceLock::new(),
            redis_client,
            redis_conn: tokio::sync::OnceCell::new(),
            max_local_capacity: 1000,
        }) }
    }

    pub fn with_capacity(redis_client: Option<redis::Client>, capacity: usize) -> Self {
        Self { inner: std::sync::Arc::new(HybridCacheInner {
            local: OnceLock::new(),
            local_tags: OnceLock::new(),
            flight_group: OnceLock::new(),
            redis_client,
            redis_conn: tokio::sync::OnceCell::new(),
            max_local_capacity: capacity,
        }) }
    }

    async fn get_redis_conn(&self) -> Option<redis::aio::MultiplexedConnection> {
        if let Some(client) = &self.inner.redis_client {
            let conn = self.inner.redis_conn.get_or_try_init(|| async {
                client.get_multiplexed_tokio_connection().await
            }).await.ok()?;
            Some(conn.clone())
        } else {
            None
        }
    }

    fn get_local(&self) -> &DashMap<String, CacheValue<T>> {
        self.inner.local.get_or_init(|| DashMap::new())
    }

    fn get_local_tags(&self) -> &DashMap<String, DashSet<String>> {
        self.inner.local_tags.get_or_init(|| DashMap::new())
    }

    fn get_flight_group(&self) -> &DashMap<String, tokio::sync::watch::Sender<Option<T>>> {
        self.inner.flight_group.get_or_init(|| DashMap::new())
    }

    pub async fn get(&self, key: &str) -> Option<T> {
        self.get_with_swr(key).await.map(|(v, _)| v)
    }

    /// Gets the value from the cache or fetches it using the provided future.
    /// Ensures that only one fetch happens concurrently for a given key.
    pub async fn get_or_fetch_with_tags_swr<F, Fut>(&self, key: &str, tags: Vec<String>, ttl: Duration, fetch: F) -> Option<T>
    where
        F: FnOnce() -> Fut + Send + 'static,
        Fut: std::future::Future<Output = Option<T>> + Send + 'static,
    {
        let res = self.get_with_swr(key).await;
        if let Some((v, is_stale)) = res.clone() {
            if !is_stale {
                return Some(v);
            }
        }

        let flight_group = self.get_flight_group();

        let mut rx = {
            if let Some(tx) = flight_group.get(key) {
                if let Some((v, true)) = res {
                    return Some(v);
                }
                tx.subscribe()
            } else {
                let (tx, _rx) = tokio::sync::watch::channel(None);
                flight_group.insert(key.to_string(), tx.clone());

                if let Some((v, true)) = res {
                    let cache_clone = self.clone();
                    let key_clone = key.to_string();
                    let tags_clone = tags.clone();
                    tokio::spawn(async move {
                        if let Some(val) = fetch().await {
                            cache_clone.set_with_tags(&key_clone, val.clone(), tags_clone, ttl).await;
                            let _ = tx.send(Some(val));
                        }
                        tokio::time::sleep(Duration::from_millis(50)).await;
                        cache_clone.get_flight_group().remove(&key_clone);
                    });
                    return Some(v);
                } else {
                    // Miss
                    if let Some(val) = fetch().await {
                        self.set_with_tags(key, val.clone(), tags, ttl).await;
                        let _ = tx.send(Some(val.clone()));
                        tokio::time::sleep(Duration::from_millis(50)).await;
                        flight_group.remove(key);
                        return Some(val);
                    }
                    tokio::time::sleep(Duration::from_millis(50)).await;
                    flight_group.remove(key);
                    return None;
                }
            }
        };

        if rx.changed().await.is_ok() {
            rx.borrow().clone()
        } else {
            None
        }
    }

    pub async fn get_or_fetch_with_swr<F, Fut>(&self, key: &str, ttl: Duration, fetch: F) -> Option<T>
    where
        F: FnOnce() -> Fut + Send + 'static,
        Fut: std::future::Future<Output = Option<T>> + Send + 'static,
    {
        let res = self.get_with_swr(key).await;
        if let Some((v, is_stale)) = res.clone() {
            if !is_stale {
                return Some(v);
            }
        }

        let flight_group = self.get_flight_group();

        let mut rx = {
            if let Some(tx) = flight_group.get(key) {
                if let Some((v, true)) = res {
                    return Some(v);
                }
                tx.subscribe()
            } else {
                let (tx, _rx) = tokio::sync::watch::channel(None);
                flight_group.insert(key.to_string(), tx.clone());

                if let Some((v, true)) = res {
                    let cache_clone = self.clone();
                    let key_clone = key.to_string();
                    tokio::spawn(async move {
                        if let Some(val) = fetch().await {
                            cache_clone.set(&key_clone, val.clone(), ttl).await;
                            let _ = tx.send(Some(val));
                        }
                        tokio::time::sleep(Duration::from_millis(50)).await;
                        cache_clone.get_flight_group().remove(&key_clone);
                    });
                    return Some(v);
                } else {
                    // Miss
                    if let Some(val) = fetch().await {
                        self.set(key, val.clone(), ttl).await;
                        let _ = tx.send(Some(val.clone()));
                        tokio::time::sleep(Duration::from_millis(50)).await;
                        flight_group.remove(key);
                        return Some(val);
                    }
                    tokio::time::sleep(Duration::from_millis(50)).await;
                    flight_group.remove(key);
                    return None;
                }
            }
        };

        if rx.changed().await.is_ok() {
            rx.borrow().clone()
        } else {
            None
        }
    }

    pub async fn get_with_swr(&self, key: &str) -> Option<(T, bool)> {
        // Try local cache first, allowing slightly stale reads
        if let Some(entry) = self.get_local().get(key) {
            let now = std::time::Instant::now();
            entry.access_count.fetch_add(1, Ordering::Relaxed);
            if now < entry.expiry {
                return Some((entry.val.clone(), false)); // Fresh
            } else if now < entry.expiry + Duration::from_secs(86400) {
                // Stale but within SWR window (1 day)
                return Some((entry.val.clone(), true));
            }
        }

        // Then Redis
        if let Some(mut conn) = self.get_redis_conn().await {
            use redis::AsyncCommands;
            let res: Result<Option<String>, _> = conn.get(key).await;
            if let Ok(Some(data)) = res {
                if let Ok(item) = serde_json::from_str::<CacheItem<T>>(&data) {
                    // Populate local cache
                    self.set_local(key, item.value.clone(), &item.tags, Duration::from_secs(60));
                    return Some((item.value, false));
                } else if let Ok(val) = serde_json::from_str::<T>(&data) {
                    // Backwards compatibility for items saved before tags
                    self.set_local(key, val.clone(), &[], Duration::from_secs(60));
                    return Some((val, false));
                }
            }
        }
        None
    }

    pub async fn set(&self, key: &str, value: T, ttl: Duration) {
        self.set_with_tags(key, value, vec![], ttl).await;
    }

    pub async fn set_with_tags(&self, key: &str, value: T, tags: Vec<String>, ttl: Duration) {
        // 1. Set local cache
        self.set_local(key, value.clone(), &tags, ttl);

        // 2. Set Redis if available
        if let Some(mut conn) = self.get_redis_conn().await {
            use redis::AsyncCommands;
            let item = CacheItem { value, tags: tags.clone() };
            if let Ok(data) = serde_json::to_string(&item) {
                let _: Result<(), _> = conn.set_ex(key, data, ttl.as_secs() as u64).await;
            }
            for tag in tags {
                let tag_key = format!("tag:{}", tag);
                let _: Result<(), _> = conn.sadd(&tag_key, key).await;
                let _: Result<(), _> = conn.expire(&tag_key, 86400 * 7).await;
            }
        }
    }

    fn set_local(&self, key: &str, value: T, tags: &[String], ttl: Duration) {
        let local = self.get_local();
        let now = std::time::Instant::now();

        if local.len() >= self.inner.max_local_capacity && !local.contains_key(key) {
            let mut removed_keys = Vec::new();
            let mut has_expired = false;

            let mut least_accessed_key = None;
            let mut lowest_access_count = u64::MAX;

            for item in local.iter() {
                if item.expiry <= now {
                    removed_keys.push(item.key().clone());
                    has_expired = true;
                } else {
                    let count = item.access_count.load(Ordering::Relaxed);
                    if count < lowest_access_count {
                        lowest_access_count = count;
                        least_accessed_key = Some(item.key().clone());
                    }
                }
            }


            if has_expired {
                for k in &removed_keys {
                    local.remove(k);
                }
            } else {
                if local.len() >= self.inner.max_local_capacity {
                    if let Some(key_to_remove) = least_accessed_key {
                        local.remove(&key_to_remove);
                        removed_keys.push(key_to_remove);
                    }

                    // LFU decay: halve all access counts
                    for item in local.iter() {
                        let current = item.access_count.load(Ordering::Relaxed);
                        item.access_count.store(current / 2, Ordering::Relaxed);
                    }
                }
            }


            // Clean up tags
            if !removed_keys.is_empty() {
                let tags_map = self.get_local_tags();
                for mut entry in tags_map.iter_mut() {
                    let keys: &mut dashmap::DashSet<String> = entry.value_mut();
                    for k in &removed_keys {
                        keys.remove(k);
                    }
                }
                tags_map.retain(|_, keys| !keys.is_empty());
            }
        }

        local.insert(
            key.to_string(),
            CacheValue {
                val: value,
                expiry: std::time::Instant::now() + ttl,
                access_count: std::sync::atomic::AtomicU64::new(0),
            },
        );

        if !tags.is_empty() {
            let tags_map = self.get_local_tags();
            for tag in tags {
                tags_map
                    .entry(tag.to_string())
                    .or_insert_with(DashSet::new)
                    .insert(key.to_string());
            }
        }
    }

    pub async fn invalidate(&self, key: &str) {
        self.get_local().remove(key);
        let tags_map = self.get_local_tags();
        for mut entry in tags_map.iter_mut() {
            let keys = entry.value_mut();
            keys.remove(key);
        }
        tags_map.retain(|_, keys| !keys.is_empty());

        if let Some(mut conn) = self.get_redis_conn().await {
            use redis::AsyncCommands;
            let _: Result<(), _> = conn.del(key).await;
        }
    }

    pub async fn invalidate_by_tag(&self, tag: &str) {
        let mut keys_to_delete = Vec::new();
        let tags_map = self.get_local_tags();

        if let Some((_, keys)) = tags_map.remove(tag) {
            let local = self.get_local();
            for key in keys.iter() {
                local.remove(key.key());
                keys_to_delete.push(key.key().clone());
            }
        }

        if let Some(mut conn) = self.get_redis_conn().await {
            use redis::AsyncCommands;
            let tag_key = format!("tag:{}", tag);
            let redis_keys: Result<Vec<String>, _> = conn.smembers(&tag_key).await;
            if let Ok(keys) = redis_keys {
                if !keys.is_empty() {
                    let mut pipe = redis::pipe();
                    for key in keys {
                        pipe.del(&key);
                    }
                    pipe.del(&tag_key);
                    let _: Result<(), _> = pipe.query_async(&mut conn).await;
                } else {
                    let _: Result<(), _> = conn.del(&tag_key).await;
                }
            }
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
        tokio::time::sleep(Duration::from_millis(5)).await;
        cache.set("k2", "v2".to_string(), Duration::from_secs(60)).await;
        tokio::time::sleep(Duration::from_millis(5)).await;

        // Access k1 so k2 becomes the LRU
        let _ = cache.get("k1").await;
        tokio::time::sleep(Duration::from_millis(5)).await;

        cache.set("k3", "v3".to_string(), Duration::from_secs(60)).await;

        let local = cache.get_local();
        assert_eq!(local.len(), 2);
        assert!(local.contains_key("k1") || local.contains_key("k2"));
        assert!(local.contains_key("k3"));
    }

    #[tokio::test]
    async fn test_hybrid_cache_tags() {
        let cache = HybridCache::<String>::with_capacity(None, 10);
        cache.set_with_tags("k1", "v1".to_string(), vec!["tag1".to_string()], Duration::from_secs(60)).await;
        cache.set_with_tags("k2", "v2".to_string(), vec!["tag1".to_string(), "tag2".to_string()], Duration::from_secs(60)).await;

        assert_eq!(cache.get("k1").await, Some("v1".to_string()));
        cache.invalidate_by_tag("tag1").await;
        assert_eq!(cache.get("k1").await, None);
        assert_eq!(cache.get("k2").await, None);
    }
}

#[cfg(test)]
mod tests_singleflight {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use tokio::time::Duration;

    #[tokio::test]
    async fn test_hybrid_cache_singleflight() {
        let cache = Arc::new(HybridCache::<String>::with_capacity(None, 10));
        let fetch_count = Arc::new(AtomicUsize::new(0));

        let mut handles = vec![];
        for _ in 0..10 {
            let cache_clone = cache.clone();
            let count_clone = fetch_count.clone();
            handles.push(tokio::spawn(async move {
                cache_clone.get_or_fetch_with_swr("test_key", Duration::from_secs(60), move || async move {
                    count_clone.fetch_add(1, Ordering::SeqCst);
                    tokio::time::sleep(Duration::from_millis(50)).await;
                    Some("test_val".to_string())
                }).await
            }));
        }

        let mut results = vec![];
        for h in handles {
            results.push(h.await.unwrap());
        }

        for res in results {
            assert_eq!(res, Some("test_val".to_string()));
        }

        // Fetch should only have been called once despite 10 concurrent requests
        assert_eq!(fetch_count.load(Ordering::SeqCst), 1);
    }
}
