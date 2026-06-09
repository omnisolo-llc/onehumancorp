use std::sync::Arc;
use std::time::{Instant, Duration};
use dashmap::DashMap;

#[derive(Clone, Debug)]
pub struct CachedResponse {
    pub text: String,
    pub created_at: Instant,
    pub token_count: usize,
    pub ttl: Duration,
}

pub struct PromptCache {
    cache: Arc<DashMap<String, CachedResponse>>,
    default_ttl: Duration,
    max_capacity: usize,
}

impl PromptCache {
    pub fn new(default_ttl: Duration) -> Self {
        Self::with_capacity(default_ttl, 1000)
    }

    pub fn with_capacity(default_ttl: Duration, max_capacity: usize) -> Self {
        PromptCache {
            cache: Arc::new(DashMap::new()),
            default_ttl,
            max_capacity,
        }
    }

    pub fn get(&self, prompt: &str) -> Option<CachedResponse> {
        let entry = self.cache.get(prompt);
        if let Some(entry_ref) = entry {
            if entry_ref.created_at.elapsed() <= entry_ref.ttl {
                // Update access time for LRU-like eviction
                // DashMap doesn't easily support mutable iteration without locking.
                // We'll update created_at as an access time surrogate if we needed strict LRU,
                // but since it has TTL, we just return it. True LRU eviction will sort by created_at.
                return Some(entry_ref.clone());
            }
            drop(entry_ref);
            // Remove expired entry atomically
            self.cache.remove_if(prompt, |_, v| v.created_at.elapsed() > v.ttl);
        }
        None
    }

    pub fn get_with_cost_cents(&self, prompt: &str) -> (Option<CachedResponse>, i64) {
        tracing::info!("💰 Miser telemetry: Prompt cache lookup");
        let res = self.get(prompt);
        let cost = if let Some(ref r) = res {
            tracing::info!("💰 Miser cost optimization: Prompt cache hit saved {} tokens", r.token_count);

            // Fast-path: Avoid taking OS environment locks on every cache hit
            static MODEL: std::sync::OnceLock<String> = std::sync::OnceLock::new();
            static FALLBACK_RATIO: std::sync::OnceLock<f64> = std::sync::OnceLock::new();

            let model = MODEL.get_or_init(|| {
                std::env::var("OHC_LLM_MODEL").unwrap_or_else(|_| "gpt-4o".to_string())
            });

            let ratio = super::calculator::calculate_heuristic_token_efficiency(r.token_count as i64, 0, model);

            if ratio == 0.0 {
                let fallback_ratio = FALLBACK_RATIO.get_or_init(|| {
                    std::env::var("MISER_TOKEN_RATIO")
                        .unwrap_or_else(|_| "0.0001".to_string())
                        .parse::<f64>()
                        .unwrap_or(0.0001)
                });
                (r.token_count as f64 * *fallback_ratio * 100.0).round() as i64
            } else {
                (ratio * 100.0).round() as i64
            }
        } else {
            0
        };
        (res, cost)
    }

    pub fn set(&self, prompt: &str, response: &str, token_count: usize) {
        self.set_with_ttl(prompt, response, token_count, self.default_ttl);
    }

    pub fn set_with_ttl(&self, prompt: &str, response: &str, token_count: usize, ttl: Duration) {
        if self.cache.len() >= self.max_capacity {
            self.evict_oldest();
        }
        self.cache.insert(prompt.to_string(), CachedResponse {
            text: response.to_string(),
            created_at: Instant::now(),
            token_count,
            ttl,
        });
    }

    fn evict_oldest(&self) {
        // Clear expired first to see if that frees enough space
        self.clear_expired();

        let len = self.cache.len();
        if len < self.max_capacity {
            return;
        }

        // We need to evict entries to get back down to 90% of max capacity
        let target_len = (self.max_capacity as f64 * 0.9) as usize;
        let to_remove = len.saturating_sub(target_len);

        if to_remove == 0 {
            return;
        }

        // Sort by created_at to remove the oldest items.
        // DashMap iter() locks the shards, so we collect keys first to minimize lock time.
        let mut entries: Vec<(String, Instant)> = self.cache.iter()
            .map(|kv| (kv.key().clone(), kv.value().created_at))
            .collect();

        entries.sort_unstable_by_key(|(_, time)| *time);

        for (key, _) in entries.into_iter().take(to_remove) {
            self.cache.remove(&key);
        }
    }

    pub fn clear_expired(&self) {
        let now = Instant::now();
        self.cache.retain(|_, entry| now.duration_since(entry.created_at) <= entry.ttl);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_prompt_cache_get_set() {
        let cache = PromptCache::new(Duration::from_secs(10));
        cache.set("What is the capital of France?", "Paris", 1);

        let response = cache.get("What is the capital of France?");
        assert!(response.is_some());
        assert_eq!(response.unwrap().text, "Paris");
    }

    #[test]
    fn test_prompt_cache_expiration() {
        let cache = PromptCache::new(Duration::from_millis(50));
        cache.set("Hello", "World", 1);

        thread::sleep(Duration::from_millis(60));
        assert!(cache.get("Hello").is_none());
    }

    #[test]
    fn test_prompt_cache_clear_expired() {
        let cache = PromptCache::new(Duration::from_millis(50));
        cache.set("Test", "Data", 1);

        thread::sleep(Duration::from_millis(60));
        cache.clear_expired();

        assert!(cache.cache.is_empty());
    }

    #[test]
    fn test_prompt_cache_get_with_cost_cents() {
        unsafe { std::env::set_var("OHC_LLM_MODEL", "gpt-4o"); } // 5.00 per 1M tokens
        let cache = PromptCache::new(Duration::from_secs(10));
        cache.set("What is the capital of France?", "Paris", 1_000_000);

        let (response, cost) = cache.get_with_cost_cents("What is the capital of France?");
        assert!(response.is_some());
        assert_eq!(response.unwrap().text, "Paris");
        // 1,000,000 tokens * 5.00 / 1M = 5.0 dollars = 500 cents
        assert_eq!(cost, 500);
    }

    #[test]
    fn test_prompt_cache_get_missing() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let response = cache.get("What is the capital of France?");
        assert!(response.is_none());
        let (response_with_cost, cost) = cache.get_with_cost_cents("What is the capital of France?");
        assert!(response_with_cost.is_none());
        assert_eq!(cost, 0);
    }

    #[test]
    fn test_prompt_cache_capacity_eviction() {
        let cache = PromptCache::with_capacity(Duration::from_secs(10), 3);

        // Insert 3 items
        cache.set("key1", "val1", 1);
        thread::sleep(Duration::from_millis(10));
        cache.set("key2", "val2", 1);
        thread::sleep(Duration::from_millis(10));
        cache.set("key3", "val3", 1);

        assert_eq!(cache.cache.len(), 3);

        // Insert 4th item, triggering eviction
        // target capacity is 90% of 3 = 2.
        // currently 3 items, len = 3. target_len = 2. to_remove = 3 - 2 = 1.
        thread::sleep(Duration::from_millis(10));
        cache.set("key4", "val4", 1);

        assert_eq!(cache.cache.len(), 3); // 3 items inserted, 1 removed (oldest) + 1 newly inserted
        assert!(cache.get("key1").is_none()); // key1 was oldest, should be gone
        assert!(cache.get("key2").is_some());
        assert!(cache.get("key3").is_some());
        assert!(cache.get("key4").is_some());
    }

    #[test]
    fn test_prompt_cache_set_with_ttl() {
        let cache = PromptCache::new(Duration::from_secs(10));
        cache.set_with_ttl("What is the capital of France?", "Paris", 1, Duration::from_millis(10));

        let response = cache.get("What is the capital of France?");
        assert!(response.is_some());
        assert_eq!(response.unwrap().text, "Paris");

        thread::sleep(Duration::from_millis(20));
        assert!(cache.get("What is the capital of France?").is_none());
    }
}
