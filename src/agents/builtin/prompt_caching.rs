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
}

impl PromptCache {
    pub fn new(ttl: Duration) -> Self {
        PromptCache {
            cache: Arc::new(Mutex::new(HashMap::new())),
            ttl,
        }
    }

    pub fn get(&self, prompt: &str) -> Option<CachedResponse> {
        let mut cache = self.cache.lock().unwrap();
        if let Some(entry) = cache.get(prompt) {
            if entry.created_at.elapsed() <= self.ttl {
                return Some(entry.clone());
            }
        }
        // Remove expired entry
        cache.remove(prompt);
        None
    }

    pub fn set(&self, prompt: &str, response: &str, token_count: usize) {
        let mut cache = self.cache.lock().unwrap();
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

        let cache_lock = cache.cache.lock().unwrap();
        assert!(cache_lock.is_empty());
    }
}
