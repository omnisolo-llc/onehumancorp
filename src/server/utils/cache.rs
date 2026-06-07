use std::collections::HashSet;
use dashmap::DashMap;
use std::sync::OnceLock;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::time::Duration;

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

pub struct HybridCache<T> {
    local: OnceLock<DashMap<String, CacheValue<T>>>,
    local_tags: OnceLock<DashMap<String, HashSet<String>>>,
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

    fn get_local(&self) -> &DashMap<String, CacheValue<T>> {
        self.local.get_or_init(|| DashMap::new())
    }

    fn get_local_tags(&self) -> &DashMap<String, HashSet<String>> {
        self.local_tags.get_or_init(|| DashMap::new())
    }

    pub async fn get(&self, key: &str) -> Option<T> {
        self.get_with_swr(key).await.map(|(v, _)| v)
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
        let guard = self.get_local();
        let now = std::time::Instant::now();

        if guard.len() >= self.max_local_capacity && !guard.contains_key(key) {
            let mut keys_to_remove = Vec::new();
            for item in guard.iter() {
                if item.value().expiry <= now {
                    keys_to_remove.push(item.key().clone());
                }
            }

            let mut removed_keys = keys_to_remove.clone();
            for k in keys_to_remove {
                guard.remove(&k);
            }

            if guard.len() >= self.max_local_capacity {
                let mut sampled_keys = Vec::new();
                for item in guard.iter() {
                    sampled_keys.push((item.key().clone(), item.value().access_count.load(Ordering::Relaxed)));
                    if sampled_keys.len() >= 5 {
                        break;
                    }
                }

                if let Some((least_accessed_key, _)) = sampled_keys.into_iter().min_by_key(|(_, count)| *count) {
                    guard.remove(&least_accessed_key);
                    removed_keys.push(least_accessed_key);
                }
            }

            if !removed_keys.is_empty() {
                let tag_guard = self.get_local_tags();
                let mut tags_to_remove = Vec::new();
                for mut tag_entry in tag_guard.iter_mut() {
                    for k in &removed_keys {
                        tag_entry.value_mut().remove(k);
                    }
                    if tag_entry.value().is_empty() {
                        tags_to_remove.push(tag_entry.key().clone());
                    }
                }
                for t in tags_to_remove {
                    tag_guard.remove(&t);
                }
            }
        }

        guard.insert(key.to_string(), CacheValue {
            val: value,
            expiry: std::time::Instant::now() + ttl,
            access_count: std::sync::atomic::AtomicU64::new(0),
        });

        let tag_guard = self.get_local_tags();
        for tag in tags {
            let mut entry = tag_guard.entry(tag.clone()).or_insert_with(HashSet::new);
            entry.insert(key.to_string());
        }
    }

    pub async fn invalidate(&self, key: &str) {
        self.get_local().remove(key);
        let tag_guard = self.get_local_tags();
        let mut tags_to_remove = Vec::new();
        for mut tag_entry in tag_guard.iter_mut() {
            tag_entry.value_mut().remove(key);
            if tag_entry.value().is_empty() {
                tags_to_remove.push(tag_entry.key().clone());
            }
        }
        for t in tags_to_remove {
            tag_guard.remove(&t);
        }
        if let Some(mut conn) = self.get_redis_conn().await {
            use redis::AsyncCommands;
            let _: Result<(), _> = conn.del(key).await;
        }
    }

    pub async fn invalidate_by_tag(&self, tag: &str) {
        let mut keys_to_delete = Vec::new();
        if let Some((_, keys)) = self.get_local_tags().remove(tag) {
            let cache_guard = self.get_local();
            for key in &keys {
                cache_guard.remove(key);
            }
            keys_to_delete.extend(keys.into_iter());
        }
        if let Some(mut conn) = self.get_redis_conn().await {
            use redis::AsyncCommands;
            let tag_key = format!("tag:{}", tag);
            let redis_keys: Result<Vec<String>, _> = conn.smembers(&tag_key).await;
            if let Ok(keys) = redis_keys {
                for key in keys {
                    let _: Result<(), _> = conn.del(&key).await;
                }
                let _: Result<(), _> = conn.del(&tag_key).await;
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
