use std::time::{Duration, Instant};
use dashmap::DashMap;
use crate::pricing::compression;
use redis::AsyncCommands;

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
        let hash = self.hash_prompt(prompt);
        if let Some(entry) = self.entries.get(&hash) {
            if entry.expires_at > Instant::now() {
                return Some(entry.response.clone());
            }
        }
        None
    }

    pub fn set(&self, prompt: &str, response: &str) {
        let hash = self.hash_prompt(prompt);
        let now = Instant::now();
        self.entries.insert(hash, CacheEntry {
            response: response.to_string(),
            created_at: now,
            expires_at: now + self.ttl,
        });
    }
}

pub struct RedisEmbeddingCache {
    client: redis::Client,
    ttl: usize,
}

impl RedisEmbeddingCache {
    pub fn new(url: &str, ttl_secs: usize) -> Result<Self, redis::RedisError> {
        let client = redis::Client::open(url)?;
        Ok(RedisEmbeddingCache { client, ttl: ttl_secs })
    }

    fn hash_prompt(&self, prompt: &str) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(prompt.as_bytes());
        hex::encode(hasher.finalize())
    }

    pub async fn get(&self, prompt: &str) -> Option<String> {
        let key = format!("embed:{}", self.hash_prompt(prompt));
        let mut con = self.client.get_multiplexed_async_connection().await.ok()?;
        let compressed: Option<Vec<u8>> = con.get(&key).await.ok();

        if let Some(c_data) = compressed {
            if let Ok(decompressed) = compression::decompress(&c_data) {
                return Some(decompressed);
            }
        }
        None
    }

    pub async fn set(&self, prompt: &str, response: &str) {
        let key = format!("embed:{}", self.hash_prompt(prompt));
        if let Ok(compressed) = compression::compress(response) {
            if let Ok(mut con) = self.client.get_multiplexed_async_connection().await {
                let _: Result<(), _> = con.set_ex(&key, compressed, self.ttl as u64).await;
            }
        }
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
        let hash = self.hash_prompt(prompt);
        if let Some(entry) = self.entries.get(&hash) {
            if entry.expires_at > Instant::now() {
                if let Ok(decompressed) = compression::decompress(entry.response.as_bytes()) {
                    return Some(decompressed);
                }
            }
        }
        None
    }

    pub fn set(&self, prompt: &str, response: &str) {
        let hash = self.hash_prompt(prompt);
        let now = Instant::now();
        if let Ok(compressed) = compression::compress(response) {
            self.entries.insert(hash, CacheEntry {
                response: String::from_utf8_lossy(&compressed).to_string(),
                created_at: now,
                expires_at: now + self.ttl,
            });
        }
    }
}
