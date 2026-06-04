use std::time::{Duration, Instant};
use dashmap::DashMap;
use crate::compression;
use sqlx::PgPool;

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

pub struct ExchangeRateCache {
    redis_client: Option<redis::Client>,
    memory_fallback: DashMap<String, (f64, Instant)>,
    ttl: Duration,
}

impl ExchangeRateCache {
    pub fn new(redis_client: Option<redis::Client>, ttl: Duration) -> Self {
        Self {
            redis_client,
            memory_fallback: DashMap::new(),
            ttl,
        }
    }

    fn cache_key(from: &str, to: &str) -> String {
        format!("fx_rate:{}:{}", from.to_uppercase(), to.to_uppercase())
    }

    pub async fn get_rate(&self, from: &str, to: &str, pool: &PgPool) -> Result<f64, String> {
        if from.eq_ignore_ascii_case(to) {
            return Ok(1.0);
        }

        let key = Self::cache_key(from, to);

        // 1. Try Redis
        if let Some(client) = &self.redis_client {
            if let Ok(mut conn) = client.get_multiplexed_tokio_connection().await {
                use redis::AsyncCommands;
                let cached_val: Result<Option<f64>, _> = conn.get(&key).await;
                if let Ok(Some(rate)) = cached_val {
                    return Ok(rate);
                }
            }
        }

        // 2. Try Memory Fallback
        if let Some(entry) = self.memory_fallback.get(&key) {
            if Instant::now() <= entry.1 {
                return Ok(entry.0);
            }
        }

        // 3. Try DB
        let row = sqlx::query("SELECT rate FROM ohc_fx_rates WHERE from_currency = $1 AND to_currency = $2")
            .bind(from.to_uppercase())
            .bind(to.to_uppercase())
            .fetch_optional(pool)
            .await
            .map_err(|e| e.to_string())?;

        if let Some(r) = row {
            use sqlx::Row;
            let rate: f64 = r.get("rate");

            // Cache in Redis
            if let Some(client) = &self.redis_client {
                if let Ok(mut conn) = client.get_multiplexed_tokio_connection().await {
                    use redis::AsyncCommands;
                    let _: Result<(), _> = conn.set_ex(&key, rate, self.ttl.as_secs() as u64).await;
                }
            }

            // Cache in memory
            self.memory_fallback.insert(key, (rate, Instant::now() + self.ttl));

            return Ok(rate);
        }

        Err(format!("Exchange rate not found for {} to {}", from, to))
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
        assert_eq!(cache.get("prompt1"), Some("response1".to_string()));
        assert_eq!(cache.get("prompt2"), None);
        
        // Wait for expiration
        thread::sleep(Duration::from_millis(150));
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
        assert_eq!(cache.get("prompt1"), Some("response1".to_string()));
        
        // Wait for expiration
        thread::sleep(Duration::from_millis(150));
        assert_eq!(cache.get("prompt1"), None);
    }

    #[tokio::test]
    async fn test_exchange_rate_cache_same_currency() {
        // We can pass a dummy PgPool because it shouldn't be hit for identical currencies
        let pool = sqlx::PgPool::connect_lazy("postgres://postgres:postgres@localhost:5432/postgres").unwrap();
        let cache = ExchangeRateCache::new(None, Duration::from_secs(60));

        let rate = cache.get_rate("USD", "USD", &pool).await.unwrap();
        assert_eq!(rate, 1.0);
        let rate2 = cache.get_rate("eur", "EUR", &pool).await.unwrap();
        assert_eq!(rate2, 1.0);
    }

    #[tokio::test]
    async fn test_exchange_rate_cache_memory_fallback() {
        let pool = sqlx::PgPool::connect_lazy("postgres://postgres:postgres@localhost:5432/postgres").unwrap();
        let cache = ExchangeRateCache::new(None, Duration::from_secs(60));

        // Populate local dashmap directly
        let key = "fx_rate:EUR:USD".to_string();
        cache.memory_fallback.insert(key, (1.10, Instant::now() + Duration::from_secs(60)));

        // This should hit the memory fallback and NOT the database
        let rate = cache.get_rate("EUR", "USD", &pool).await.unwrap();
        assert_eq!(rate, 1.10);
    }
}
