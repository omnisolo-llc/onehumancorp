use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Instant, Duration};

#[derive(Clone, Debug)]
pub struct CachedResponse {
    pub text: String,
    pub created_at: Instant,
    pub token_count: usize,
}

pub struct PromptCache {
    // Key is now "tenant_id:prompt" to ensure isolation
    cache: Arc<Mutex<HashMap<String, CachedResponse>>>,
    ttl: Duration,
}

impl PromptCache {
    pub fn new(ttl: Duration) -> Self {
        PromptCache {
            cache: Arc::new(Mutex::new(HashMap::new())),
            ttl,
        }
    }

    fn build_key(tenant_id: &str, prompt: &str) -> String {
        format!("{}:{}", tenant_id, prompt)
    }

    pub fn get(&self, tenant_id: &str, prompt: &str) -> Option<CachedResponse> {
        let key = Self::build_key(tenant_id, prompt);
        let mut cache = self.cache.lock().unwrap();
        if let Some(entry) = cache.get(&key) {
            if entry.created_at.elapsed() <= self.ttl {
                return Some(entry.clone());
            }
        }
        // Remove expired entry
        cache.remove(&key);
        None
    }

    pub fn get_with_cost_cents(&self, tenant_id: &str, prompt: &str) -> (Option<CachedResponse>, i64) {
        let res = self.get(tenant_id, prompt);
        let cost = if let Some(ref r) = res {
            // very rough estimate of saved cents for cache hit
            (r.token_count as f64 * 0.0001).round() as i64
        } else {
            0
        };
        (res, cost)
    }

    pub fn set(&self, tenant_id: &str, prompt: &str, response: &str, token_count: usize) {
        let key = Self::build_key(tenant_id, prompt);
        let mut cache = self.cache.lock().unwrap();
        cache.insert(key, CachedResponse {
            text: response.to_string(),
            created_at: Instant::now(),
            token_count,
        });
    }

    pub fn clear_expired(&self) {
        let mut cache = self.cache.lock().unwrap();
        let now = Instant::now();
        cache.retain(|_, entry| now.duration_since(entry.created_at) <= self.ttl);
    }
}

pub fn truncate_context(context: &str, max_tokens: usize) -> String {
    // Basic approximation: 1 token ~= 4 characters
    let max_chars = max_tokens * 4;
    if context.len() <= max_chars {
        return context.to_string();
    }

    // Intelligently truncate from the beginning (assuming most recent/relevant context is at the end)
    let truncated = &context[context.len() - max_chars..];
    format!("...{}", truncated)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_prompt_cache_get_set_tenant_isolation() {
        let cache = PromptCache::new(Duration::from_secs(10));
        cache.set("tenant_1", "What is the capital of France?", "Paris", 1);

        let response = cache.get("tenant_1", "What is the capital of France?");
        assert!(response.is_some());
        assert_eq!(response.unwrap().text, "Paris");

        let response2 = cache.get("tenant_2", "What is the capital of France?");
        assert!(response2.is_none());
    }

    #[test]
    fn test_prompt_cache_expiration() {
        let cache = PromptCache::new(Duration::from_millis(50));
        cache.set("t1", "Hello", "World", 1);

        thread::sleep(Duration::from_millis(60));
        assert!(cache.get("t1", "Hello").is_none());
    }

    #[test]
    fn test_prompt_cache_clear_expired() {
        let cache = PromptCache::new(Duration::from_millis(50));
        cache.set("t1", "Test", "Data", 1);

        thread::sleep(Duration::from_millis(60));
        cache.clear_expired();

        let cache_lock = cache.cache.lock().unwrap();
        assert!(cache_lock.is_empty());
    }

    #[test]
    fn test_truncate_context() {
        let context = "This is a very long context string that needs to be truncated because it exceeds the token limit.";
        let truncated = truncate_context(context, 10);
        assert!(truncated.starts_with("..."));
        // 10 tokens * 4 chars = 40 + 3 dots = 43 chars max
        assert!(truncated.len() <= 43);
    }
}
