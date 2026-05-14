use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use sha2::{Sha256, Digest};
use hex;

#[derive(Debug, Clone)]
pub struct PromptCache {
    storage: Arc<Mutex<HashMap<String, (String, Instant)>>>,
    ttl: Duration,
}

impl PromptCache {
    pub fn new(ttl: Duration) -> Self {
        Self {
            storage: Arc::new(Mutex::new(HashMap::new())),
            ttl,
        }
    }

    pub fn hash_prompt(&self, system: &str, user: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(system.as_bytes());
        hasher.update(user.as_bytes());
        hex::encode(hasher.finalize())
    }

    pub async fn get(&self, system: &str, user: &str) -> Option<String> {
        let key = self.hash_prompt(system, user);
        let mut storage = self.storage.lock().unwrap();

        if let Some((response, timestamp)) = storage.get(&key) {
            if timestamp.elapsed() < self.ttl {
                return Some(response.clone());
            } else {
                storage.remove(&key);
            }
        }
        None
    }

    pub async fn set(&self, system: &str, user: &str, response: &str) {
        let key = self.hash_prompt(system, user);
        let mut storage = self.storage.lock().unwrap();
        storage.insert(key, (response.to_string(), Instant::now()));
    }

    pub fn clear_expired(&self) {
        let mut storage = self.storage.lock().unwrap();
        let ttl = self.ttl;
        storage.retain(|_, (_, timestamp)| timestamp.elapsed() < ttl);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;

    #[tokio::test]
    async fn test_prompt_cache_get_set() {
        let cache = PromptCache::new(Duration::from_secs(60));
        let sys = "system prompt";
        let user = "user message";
        let resp = "cached response";

        cache.set(sys, user, resp).await;
        assert_eq!(cache.get(sys, user).await, Some(resp.to_string()));
    }

    #[tokio::test]
    async fn test_prompt_cache_expiration() {
        let cache = PromptCache::new(Duration::from_millis(10));
        cache.set("s", "u", "r").await;
        sleep(Duration::from_millis(20));
        assert_eq!(cache.get("s", "u").await, None);
    }
}
