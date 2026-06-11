use std::collections::{HashMap, HashSet};
use std::sync::{OnceLock, RwLock};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::time::Duration;

#[derive(Clone, Serialize, Deserialize)]
struct CacheItem<T> {
    value: T,
    tags: Vec<String>,
}

use std::sync::atomic::{AtomicUsize, Ordering};

static EVICTION_SEED: AtomicUsize = AtomicUsize::new(0);

struct CacheValue<T> {
    val: T,
    expiry: std::time::Instant,
    access_count: std::sync::atomic::AtomicU64,
}

pub struct HybridCache<T> {
    local: OnceLock<RwLock<HashMap<String, CacheValue<T>>>>,
    local_tags: OnceLock<RwLock<HashMap<String, HashSet<String>>>>,
    redis_client: Option<redis::Client>,
    redis_conn: tokio::sync::OnceCell<redis::aio::MultiplexedConnection>,
    max_local_capacity: usize,
}

impl<T> HybridCache<T>
where
    T: Serialize + DeserializeOwned + Clone + Send + Sync + 'static
{
    pub fn new(redis_client: Option<redis::Client>) -> Self {
        Self {
            local: OnceLock::new(),
            local_tags: OnceLock::new(),
            redis_client,
            redis_conn: tokio::sync::OnceCell::new(),
            max_local_capacity: 1000,
        }
    }

    pub fn with_capacity(redis_client: Option<redis::Client>, capacity: usize) -> Self {
        Self {
            local: OnceLock::new(),
            local_tags: OnceLock::new(),
            redis_client,
            redis_conn: tokio::sync::OnceCell::new(),
            max_local_capacity: capacity,
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

    fn get_local(&self) -> &RwLock<HashMap<String, CacheValue<T>>> {
        self.local.get_or_init(|| RwLock::new(HashMap::new()))
    }

    fn get_local_tags(&self) -> &RwLock<HashMap<String, HashSet<String>>> {
        self.local_tags.get_or_init(|| RwLock::new(HashMap::new()))
    }

    pub async fn get(&self, key: &str) -> Option<T> {
        self.get_with_swr(key).await.map(|(v, _)| v)
    }

    pub async fn get_with_swr(&self, key: &str) -> Option<(T, bool)> {
        // Try local cache first, allowing slightly stale reads
        {
            if let Ok(guard) = self.get_local().read() {
                if let Some(entry) = guard.get(key) {
                    let now = std::time::Instant::now();
                    entry.access_count.fetch_add(1, Ordering::Relaxed);
                    if now < entry.expiry {
                        return Some((entry.val.clone(), false)); // Fresh
                    } else if now < entry.expiry + Duration::from_secs(86400) {
                        // Stale but within SWR window (1 day)
                        return Some((entry.val.clone(), true));
                    }
                }
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
        if let Ok(mut guard) = self.get_local().write() {
            let now = std::time::Instant::now();
            if guard.len() >= self.max_local_capacity && !guard.contains_key(key) {
                let mut removed_keys = Vec::new();
                let offset = EVICTION_SEED.fetch_add(7, Ordering::Relaxed) % guard.len();
                let mut sampled_keys = Vec::new();
                let mut has_expired = false;

                // We do a small probabilistic sample instead of an O(N) iteration for eviction.
                for (k, entry) in guard.iter().skip(offset).chain(guard.iter()).take(10) {
                    if entry.expiry <= now {
                        removed_keys.push(k.clone());
                        has_expired = true;
                    } else {
                        sampled_keys.push((k.clone(), entry.access_count.load(Ordering::Relaxed)));
                    }
                }

                if has_expired {
                    for k in &removed_keys {
                        guard.remove(k);
                    }
                } else {
                    if guard.len() >= self.max_local_capacity {
                        sampled_keys.truncate(5); // Only take 5 samples for LFU to avoid excessive work
                        if let Some((least_accessed_key, _)) = sampled_keys.into_iter().min_by_key(|(_, count)| *count) {
                            guard.remove(&least_accessed_key);
                            removed_keys.push(least_accessed_key);
                        }
                    }
                }

                // Clean up tags
                if !removed_keys.is_empty() {
                    if let Ok(mut tag_guard) = self.get_local_tags().write() {
                        for (_, keys) in tag_guard.iter_mut() {
                            for k in &removed_keys {
                                keys.remove(k);
                            }
                        }
                        tag_guard.retain(|_, keys| !keys.is_empty());
                    }
                }
            }
            guard.insert(key.to_string(), CacheValue {
                val: value,
                expiry: std::time::Instant::now() + ttl,
                access_count: std::sync::atomic::AtomicU64::new(0),
            });
        }
        if let Ok(mut tag_guard) = self.get_local_tags().write() {
            for tag in tags {
                tag_guard
                    .entry(tag.clone())
                    .or_insert_with(HashSet::new)
                    .insert(key.to_string());
            }
        }
    }

    pub async fn invalidate(&self, key: &str) {
        if let Ok(mut guard) = self.get_local().write() {
            guard.remove(key);
        }
        if let Ok(mut tag_guard) = self.get_local_tags().write() {
            for (_, keys) in tag_guard.iter_mut() {
                keys.remove(key);
            }
            tag_guard.retain(|_, keys| !keys.is_empty());
        }
        if let Some(mut conn) = self.get_redis_conn().await {
            use redis::AsyncCommands;
            let _: Result<(), _> = conn.del(key).await;
        }
    }

    pub async fn invalidate_by_tag(&self, tag: &str) {
        let mut keys_to_delete = Vec::new();
        if let Ok(mut tag_guard) = self.get_local_tags().write() {
            if let Some(keys) = tag_guard.remove(tag) {
                if let Ok(mut cache_guard) = self.get_local().write() {
                    for key in &keys {
                        cache_guard.remove(key);
                    }
                }
                keys_to_delete.extend(keys.into_iter());
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

        let local = cache.get_local().read().unwrap();
        assert_eq!(local.len(), 2);
        assert!(local.contains_key("k1"));
        assert!(local.contains_key("k3"));
        assert!(!local.contains_key("k2"));
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
