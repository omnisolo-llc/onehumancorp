use std::time::{Duration, Instant};
use dashmap::DashMap;
use crate::compression;

#[derive(Debug, Clone)]
pub struct CacheEntry {
    pub response: String,
    pub created_at: Instant,
    pub expires_at: Instant,
}

pub struct LocalEmbeddingCache {
    entries: DashMap<String, CacheEntry>,
    ttl: Duration,
}

impl LocalEmbeddingCache {
    pub fn new(ttl: Duration) -> Self {
        LocalEmbeddingCache {
            entries: DashMap::new(),
            ttl,
        }
    }

    fn hash_prompt(&self, prompt: &str) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(prompt.as_bytes());
        hex::encode(hasher.finalize())
    }

    pub fn get(&self, prompt: &str) -> Option<String> {
        let key = self.hash_prompt(prompt);
        if let Some(entry) = self.entries.get(&key) {
            if Instant::now() > entry.expires_at {
                return None;
            }
            Some(entry.response.clone())
        } else {
            None
        }
    }

    pub fn set(&self, prompt: &str, response: &str) {
        let key = self.hash_prompt(prompt);
        let now = Instant::now();
        self.entries.insert(key, CacheEntry {
            response: response.to_string(),
            created_at: now,
            expires_at: now + self.ttl,
        });
    }

    pub fn prune(&self) -> usize {
        let now = Instant::now();
        let expired_keys: Vec<String> = self.entries.iter()
            .filter(|entry| now > entry.value().expires_at)
            .map(|entry| entry.key().clone())
            .collect();

        let pruned = expired_keys.len();
        for key in expired_keys {
            self.entries.remove(&key);
        }

        pruned
    }
}

pub struct CompressedEmbeddingCache {
    entries: DashMap<String, CacheEntry>,
    ttl: Duration,
}

impl CompressedEmbeddingCache {
    pub fn new(ttl: Duration) -> Self {
        CompressedEmbeddingCache {
            entries: DashMap::new(),
            ttl,
        }
    }

    fn hash_prompt(&self, prompt: &str) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(prompt.as_bytes());
        hex::encode(hasher.finalize())
    }

    pub fn get(&self, prompt: &str) -> Option<String> {
        let key = self.hash_prompt(prompt);
        if let Some(entry) = self.entries.get(&key) {
            if Instant::now() > entry.expires_at {
                return None;
            }
            match compression::decompress_lossless(&entry.response) {
                Ok(decompressed) => Some(decompressed),
                Err(e) => {
                    tracing::error!("Failed to decompress cached response: {}", e);
                    None
                }
            }
        } else {
            None
        }
    }

    pub fn set(&self, prompt: &str, response: &str) {
        let key = self.hash_prompt(prompt);
        let now = Instant::now();
        
        match compression::compress_lossless(response) {
            Ok(compressed) => {
                self.entries.insert(key, CacheEntry {
                    response: compressed,
                    created_at: now,
                    expires_at: now + self.ttl,
                });
            }
            Err(e) => {
                tracing::error!("Failed to compress cache response: {}", e);
            }
        }
    }

    pub fn prune(&self) -> usize {
        let now = Instant::now();
        let expired_keys: Vec<String> = self.entries.iter()
            .filter(|entry| now > entry.value().expires_at)
            .map(|entry| entry.key().clone())
            .collect();

        let pruned = expired_keys.len();
        for key in expired_keys {
            self.entries.remove(&key);
        }

        pruned
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_local_embedding_cache() {
        let cache = LocalEmbeddingCache::new(Duration::from_millis(100));
        
        cache.set("prompt1", "response1");
        std::thread::sleep(std::time::Duration::from_millis(10));
        assert_eq!(cache.get("prompt1"), Some("response1".to_string()));
        assert_eq!(cache.get("prompt2"), None);
        
        // Wait for expiration
        thread::sleep(Duration::from_millis(500));
        assert_eq!(cache.get("prompt1"), None);
        
        // Prune
        cache.set("prompt3", "response3");
        assert_eq!(cache.prune(), 1); // Should prune prompt1
    }

    #[test]
    fn test_local_embedding_cache_get() {
        let cache = LocalEmbeddingCache::new(Duration::from_secs(60));

        // Insert a test prompt
        let prompt = "test_prompt_get";
        let expected_response = "test_response_get";
        cache.set(prompt, expected_response);

        // Verify we can retrieve it correctly
        let result = cache.get(prompt);
        assert_eq!(result, Some(expected_response.to_string()));
    }

    #[test]
    fn test_compressed_embedding_cache() {
        let cache = CompressedEmbeddingCache::new(Duration::from_millis(100));
        
        cache.set("prompt1", "response1");
        std::thread::sleep(std::time::Duration::from_millis(10));
        assert_eq!(cache.get("prompt1"), Some("response1".to_string()));
        
        // Wait for expiration
        thread::sleep(Duration::from_millis(500));
        assert_eq!(cache.get("prompt1"), None);
    }
}
