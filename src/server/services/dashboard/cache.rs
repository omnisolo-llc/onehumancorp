use std::sync::OnceLock;
use std::sync::Arc;
use std::time::{Instant, Duration};
use dashmap::DashMap;

pub struct CacheEntry<T> {
    pub data: T,
    pub expires_at: Instant,
}

pub struct DistributedCache<T> {
    // In Standalone Mode (or as a fallback), use an in-memory DashMap.
    memory_fallback: DashMap<String, CacheEntry<T>>,
}

impl<T: Clone> DistributedCache<T> {
    pub fn new() -> Self {
        Self {
            memory_fallback: DashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<T> {
        // Implementation note: Cloud Mode would query Redis here.
        // We simulate that by strictly keeping the logic local memory but
        // it serves as the unifying layer for Phase 4.
        if let Some(entry) = self.memory_fallback.get(key) {
            if Instant::now() < entry.value().expires_at {
                return Some(entry.value().data.clone());
            }
        }
        None
    }

    pub fn set(&self, key: &str, value: T, ttl: Duration) {
        // Implementation note: Cloud Mode would SETEX in Redis here.
        self.memory_fallback.insert(
            key.to_string(),
            CacheEntry {
                data: value,
                expires_at: Instant::now() + ttl,
            },
        );
    }
}

pub static PRODUCTS_CACHE: OnceLock<Arc<DistributedCache<Vec<crate::ohc::organization::Product>>>> = OnceLock::new();

pub fn get_products_cache() -> Arc<DistributedCache<Vec<crate::ohc::organization::Product>>> {
    PRODUCTS_CACHE.get_or_init(|| Arc::new(DistributedCache::new())).clone()
}
