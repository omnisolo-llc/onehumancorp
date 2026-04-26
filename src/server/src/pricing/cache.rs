use std::collections::HashMap;
use std::sync::RwLock;
use std::time::{Duration, Instant};
use crate::pricing::compression;

#[derive(Debug, Clone)]
pub struct CacheEntry {
    pub response: String,
    pub created_at: Instant,
    pub expires_at: Instant,
}

pub struct LocalEmbeddingCache {
    entries: RwLock<HashMap<String, CacheEntry>>,
    ttl: Duration,
}

impl LocalEmbeddingCache {
    pub fn new(ttl: Duration) -> Self {
        LocalEmbeddingCache {
            entries: RwLock::new(HashMap::new()),
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
        let entries = self.entries.read().unwrap();
        
        if let Some(entry) = entries.get(&key) {
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
        let mut entries = self.entries.write().unwrap();
        
        entries.insert(key, CacheEntry {
            response: response.to_string(),
            created_at: now,
            expires_at: now + self.ttl,
        });
    }

    pub fn prune(&self) -> usize {
        let mut entries = self.entries.write().unwrap();
        let now = Instant::now();
        let initial_len = entries.len();
        entries.retain(|_, entry| now <= entry.expires_at);
        initial_len - entries.len()
    }
}

pub struct CompressedEmbeddingCache {
    entries: RwLock<HashMap<String, CacheEntry>>,
    ttl: Duration,
}

impl CompressedEmbeddingCache {
    pub fn new(ttl: Duration) -> Self {
        CompressedEmbeddingCache {
            entries: RwLock::new(HashMap::new()),
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
        let entries = self.entries.read().unwrap();
        
        if let Some(entry) = entries.get(&key) {
            if Instant::now() > entry.expires_at {
                return None;
            }
            match compression::decompress_lossless(&entry.response) {
                Ok(decompressed) => Some(decompressed),
                Err(e) => {
                    eprintln!("Failed to decompress cached response: {}", e);
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
                let mut entries = self.entries.write().unwrap();
                entries.insert(key, CacheEntry {
                    response: compressed,
                    created_at: now,
                    expires_at: now + self.ttl,
                });
            }
            Err(e) => {
                eprintln!("Failed to compress cache response: {}", e);
            }
        }
    }

    pub fn prune(&self) -> usize {
        let mut entries = self.entries.write().unwrap();
        let now = Instant::now();
        let initial_len = entries.len();
        entries.retain(|_, entry| now <= entry.expires_at);
        initial_len - entries.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_local_embedding_cache() {
        let cache = LocalEmbeddingCache::new(Duration::from_secs(1));
        
        cache.set("prompt1", "response1");
        assert_eq!(cache.get("prompt1"), Some("response1".to_string()));
        assert_eq!(cache.get("prompt2"), None);
        
        // Wait for expiration
        thread::sleep(Duration::from_secs(2));
        assert_eq!(cache.get("prompt1"), None);
        
        // Prune
        cache.set("prompt3", "response3");
        assert_eq!(cache.prune(), 1); // Should prune prompt1
    }

    #[test]
    fn test_compressed_embedding_cache() {
        let cache = CompressedEmbeddingCache::new(Duration::from_secs(1));
        
        cache.set("prompt1", "response1");
        assert_eq!(cache.get("prompt1"), Some("response1".to_string()));
        
        // Wait for expiration
        thread::sleep(Duration::from_secs(2));
        assert_eq!(cache.get("prompt1"), None);
    }
}
