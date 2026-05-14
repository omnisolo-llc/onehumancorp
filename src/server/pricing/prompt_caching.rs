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
    cache: Arc<Mutex<HashMap<String, CachedResponse>>>,
    ttl: Duration,
    max_size: usize,
    tokens_saved: Arc<Mutex<usize>>,
}

impl PromptCache {
    pub fn new(ttl: Duration) -> Self {
        PromptCache {
            cache: Arc::new(Mutex::new(HashMap::new())),
            ttl,
            max_size: 10000,
            tokens_saved: Arc::new(Mutex::new(0)),
        }
    }

    pub fn with_max_size(mut self, max_size: usize) -> Self {
        self.max_size = max_size;
        self
    }

    pub fn get(&self, prompt: &str) -> Option<CachedResponse> {
        let mut cache = self.cache.lock().unwrap();
        if let Some(entry) = cache.get(prompt) {
            if entry.created_at.elapsed() <= self.ttl {
                return Some(entry.clone());
            }
        }
        cache.remove(prompt);
        None
    }

    pub fn get_with_cost_cents(&self, prompt: &str) -> (Option<CachedResponse>, i64) {
        let res = self.get(prompt);
        let cost = if let Some(ref r) = res {
            // increment token saved tracking
            let mut ts = self.tokens_saved.lock().unwrap();
            *ts += r.token_count;

            // very rough estimate of saved cents for cache hit
            (r.token_count as f64 * 0.0001).round() as i64
        } else {
            0
        };
        (res, cost)
    }

    pub fn set(&self, prompt: &str, response: &str, token_count: usize) {
        let mut cache = self.cache.lock().unwrap();

        // Eviction policy if cache is full
        if cache.len() >= self.max_size {
            // Find the oldest entry
            if let Some(oldest_key) = cache.iter()
                .min_by_key(|(_, v)| v.created_at)
                .map(|(k, _)| k.clone())
            {
                cache.remove(&oldest_key);
            }
        }

        cache.insert(prompt.to_string(), CachedResponse {
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

    pub fn get_tokens_saved(&self) -> usize {
        let ts = self.tokens_saved.lock().unwrap();
        *ts
    }
}

// Token Truncation Utilities
pub fn intelligent_context_truncation(context: &str, max_tokens_estimate: usize) -> String {
    // Rough estimation: 1 token ~= 4 chars
    let max_chars = max_tokens_estimate * 4;

    if context.len() <= max_chars || max_chars < 30 {
        return context.to_string();
    }

    let half_chars = max_chars / 2;

    // Find safe boundary for start
    let mut start_end = half_chars.saturating_sub(10);
    while start_end > 0 && !context.is_char_boundary(start_end) {
        start_end -= 1;
    }
    let start_part = &context[..start_end];

    // Find safe boundary for end
    let mut end_start = context.len().saturating_sub(half_chars.saturating_sub(10));
    while end_start < context.len() && !context.is_char_boundary(end_start) {
        end_start += 1;
    }

    if end_start < context.len() {
        let end_part = &context[end_start..];
        format!("{}...[TRUNCATED]...{}", start_part, end_part)
    } else {
        // Fallback safe boundary
        let mut fallback_end = max_chars.saturating_sub(15);
        while fallback_end > 0 && !context.is_char_boundary(fallback_end) {
            fallback_end -= 1;
        }
        format!("{}...[TRUNCATED]", &context[..fallback_end])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_prompt_cache_get_set() {
        let cache = PromptCache::new(Duration::from_secs(10));
        cache.set("What is the capital of France?", "Paris", 10);

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

        let cache_lock = cache.cache.lock().unwrap();
        assert!(cache_lock.is_empty());
    }

    #[test]
    fn test_prompt_cache_eviction() {
        let cache = PromptCache::new(Duration::from_secs(10)).with_max_size(2);
        cache.set("1", "A", 1);
        thread::sleep(Duration::from_millis(10));
        cache.set("2", "B", 1);
        thread::sleep(Duration::from_millis(10));
        cache.set("3", "C", 1);

        // 1 should be evicted
        assert!(cache.get("1").is_none());
        assert!(cache.get("2").is_some());
        assert!(cache.get("3").is_some());
    }

    #[test]
    fn test_intelligent_truncation() {
        let context = "This is a very long string that we want to test for truncation. It has many words and should be cut down properly.";
        // About 114 chars. Let's limit to 10 tokens (~40 chars)
        let truncated = intelligent_context_truncation(context, 10);

        assert!(truncated.contains("...[TRUNCATED]..."));
        assert!(truncated.len() <= 50); // 40 chars + room for ...[TRUNCATED]... padding leniency

        // Ensure it contains the beginning and end
        assert!(truncated.starts_with("This is a very "));
        assert!(truncated.ends_with(" cut down properly."));
    }

    #[test]
    fn test_token_saved_tracking() {
        let cache = PromptCache::new(Duration::from_secs(10));
        cache.set("TestPrompt", "Response", 500);

        // Miss
        let (res1, _) = cache.get_with_cost_cents("Unknown");
        assert!(res1.is_none());
        assert_eq!(cache.get_tokens_saved(), 0);

        // Hit
        let (res2, _) = cache.get_with_cost_cents("TestPrompt");
        assert!(res2.is_some());
        assert_eq!(cache.get_tokens_saved(), 500);

        // Another Hit
        let _ = cache.get_with_cost_cents("TestPrompt");
        assert_eq!(cache.get_tokens_saved(), 1000);
    }

    #[test]
    fn test_edge_case_truncation_empty_string() {
        let truncated = intelligent_context_truncation("", 10);
        assert_eq!(truncated, "");
    }

    #[test]
    fn test_edge_case_truncation_exact_size() {
        let context = "12345678"; // 8 chars
        let truncated = intelligent_context_truncation(context, 2); // 2 tokens * 4 = 8 chars
        assert_eq!(truncated, context);
    }

    #[test]
    fn test_edge_case_cache_ttl_zero() {
        let cache = PromptCache::new(Duration::from_secs(0));
        cache.set("Key", "Val", 10);
        thread::sleep(Duration::from_millis(1));
        assert!(cache.get("Key").is_none());
    }

    #[test]
    fn test_eviction_with_same_timestamps() {
        let cache = PromptCache::new(Duration::from_secs(10)).with_max_size(1);
        cache.set("1", "A", 1);
        cache.set("2", "B", 1);
        // Size is 1, so 1 should be evicted when 2 is added
        assert!(cache.get("1").is_none() || cache.get("2").is_none());
        assert_eq!(cache.cache.lock().unwrap().len(), 1);
    }

    #[test]
    fn test_multiple_gets_do_not_reset_ttl() {
        let cache = PromptCache::new(Duration::from_millis(50));
        cache.set("Key", "Val", 10);

        thread::sleep(Duration::from_millis(20));
        assert!(cache.get("Key").is_some());

        thread::sleep(Duration::from_millis(40));
        // Total time is 60ms > 50ms TTL, should be none even though it was accessed
        assert!(cache.get("Key").is_none());
    }
}
