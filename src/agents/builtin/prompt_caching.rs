use std::collections::HashMap;
<<<<<<< HEAD
use std::sync::Arc;
use tokio::sync::RwLock;
=======
use std::sync::{Arc, Mutex};
>>>>>>> 5b473f7d0 (feat: harden platform and real-data e2e)
use std::time::{Instant, Duration};

#[derive(Clone, Debug)]
pub struct CachedResponse {
    pub text: String,
    pub created_at: Instant,
    pub token_count: usize,
}

pub struct PromptCache {
<<<<<<< HEAD
    cache: Arc<RwLock<HashMap<String, CachedResponse>>>,
=======
    cache: Arc<Mutex<HashMap<String, CachedResponse>>>,
>>>>>>> 5b473f7d0 (feat: harden platform and real-data e2e)
    ttl: Duration,
}

impl PromptCache {
    pub fn new(ttl: Duration) -> Self {
        PromptCache {
<<<<<<< HEAD
            cache: Arc::new(RwLock::new(HashMap::new())),
=======
            cache: Arc::new(Mutex::new(HashMap::new())),
>>>>>>> 5b473f7d0 (feat: harden platform and real-data e2e)
            ttl,
        }
    }

<<<<<<< HEAD


    pub async fn get(&self, prompt: &str) -> Option<CachedResponse> {
        {
            let cache = self.cache.read().await;
            if let Some(entry) = cache.get(prompt) {
                if entry.created_at.elapsed() <= self.ttl {
                    return Some(entry.clone());
                }
            } else {
                return None;
            }
        }

        // Remove expired entry. Double check to avoid race condition.
        let mut cache = self.cache.write().await;
        if let Some(entry) = cache.get(prompt) {
            if entry.created_at.elapsed() > self.ttl {
                cache.remove(prompt);
            } else {
                return Some(entry.clone());
            }
        }
        None
    }



    pub async fn set(&self, prompt: &str, response: &str, token_count: usize) {
        let mut cache = self.cache.write().await;
=======
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
>>>>>>> 5b473f7d0 (feat: harden platform and real-data e2e)
        cache.insert(prompt.to_string(), CachedResponse {
            text: response.to_string(),
            created_at: Instant::now(),
            token_count,
        });
    }

<<<<<<< HEAD
    pub async fn clear_expired(&self) {
        let mut cache = self.cache.write().await;
=======
    pub fn clear_expired(&self) {
        let mut cache = self.cache.lock().unwrap();
>>>>>>> 5b473f7d0 (feat: harden platform and real-data e2e)
        let now = Instant::now();
        cache.retain(|_, entry| now.duration_since(entry.created_at) <= self.ttl);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
<<<<<<< HEAD


    #[tokio::test]
    fn test_prompt_cache_get_set() {
        let cache = PromptCache::new(Duration::from_secs(10));
        cache.set("What is the capital of France?", "Paris", 1).await;

        let response = cache.get("What is the capital of France?").await;
=======
    use std::thread;

    #[test]
    fn test_prompt_cache_get_set() {
        let cache = PromptCache::new(Duration::from_secs(10));
        cache.set("What is the capital of France?", "Paris", 1);

        let response = cache.get("What is the capital of France?");
>>>>>>> 5b473f7d0 (feat: harden platform and real-data e2e)
        assert!(response.is_some());
        assert_eq!(response.unwrap().text, "Paris");
    }

<<<<<<< HEAD
    #[tokio::test]
    fn test_prompt_cache_expiration() {
        let cache = PromptCache::new(Duration::from_millis(50));
        cache.set("Hello", "World", 1).await;

        tokio::time::sleep(Duration::from_millis(60)).await;
        assert!(cache.get("Hello").await.is_none());
    }

    #[tokio::test]
    fn test_prompt_cache_clear_expired() {
        let cache = PromptCache::new(Duration::from_millis(50));
        cache.set("Test", "Data", 1).await;

        tokio::time::sleep(Duration::from_millis(60)).await;
        cache.clear_expired().await;

        let cache_lock = cache.cache.read().await;
=======
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
>>>>>>> 5b473f7d0 (feat: harden platform and real-data e2e)
        assert!(cache_lock.is_empty());
    }
}
