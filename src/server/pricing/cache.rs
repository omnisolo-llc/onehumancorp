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
    fn test_compressed_embedding_cache() {
        let cache = CompressedEmbeddingCache::new(Duration::from_millis(100));
        
        cache.set("prompt1", "response1");
        assert_eq!(cache.get("prompt1"), Some("response1".to_string()));
        
        // Wait for expiration
        thread::sleep(Duration::from_millis(150));
        assert_eq!(cache.get("prompt1"), None);
    }
}

#[cfg(test)]
mod exhaustive_cache_tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_local_embedding_cache_eviction_edge_case_1() {
        let cache = LocalEmbeddingCache::new(Duration::from_millis(10));
        cache.set("prompt_1", "response_1");
        assert_eq!(cache.get("prompt_1"), Some("response_1".to_string()));
        thread::sleep(Duration::from_millis(60));
        assert_eq!(cache.get("prompt_1"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_compressed_embedding_cache_eviction_edge_case_1() {
        let cache = CompressedEmbeddingCache::new(Duration::from_millis(10));
        let large_payload = "A".repeat(100);
        cache.set("prompt_1", &large_payload);
        assert_eq!(cache.get("prompt_1").unwrap(), large_payload);
        thread::sleep(Duration::from_millis(60));
        assert_eq!(cache.get("prompt_1"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_local_embedding_cache_eviction_edge_case_2() {
        let cache = LocalEmbeddingCache::new(Duration::from_millis(20));
        cache.set("prompt_2", "response_2");
        assert_eq!(cache.get("prompt_2"), Some("response_2".to_string()));
        thread::sleep(Duration::from_millis(70));
        assert_eq!(cache.get("prompt_2"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_compressed_embedding_cache_eviction_edge_case_2() {
        let cache = CompressedEmbeddingCache::new(Duration::from_millis(20));
        let large_payload = "A".repeat(200);
        cache.set("prompt_2", &large_payload);
        assert_eq!(cache.get("prompt_2").unwrap(), large_payload);
        thread::sleep(Duration::from_millis(70));
        assert_eq!(cache.get("prompt_2"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_local_embedding_cache_eviction_edge_case_3() {
        let cache = LocalEmbeddingCache::new(Duration::from_millis(30));
        cache.set("prompt_3", "response_3");
        assert_eq!(cache.get("prompt_3"), Some("response_3".to_string()));
        thread::sleep(Duration::from_millis(80));
        assert_eq!(cache.get("prompt_3"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_compressed_embedding_cache_eviction_edge_case_3() {
        let cache = CompressedEmbeddingCache::new(Duration::from_millis(30));
        let large_payload = "A".repeat(300);
        cache.set("prompt_3", &large_payload);
        assert_eq!(cache.get("prompt_3").unwrap(), large_payload);
        thread::sleep(Duration::from_millis(80));
        assert_eq!(cache.get("prompt_3"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_local_embedding_cache_eviction_edge_case_4() {
        let cache = LocalEmbeddingCache::new(Duration::from_millis(40));
        cache.set("prompt_4", "response_4");
        assert_eq!(cache.get("prompt_4"), Some("response_4".to_string()));
        thread::sleep(Duration::from_millis(90));
        assert_eq!(cache.get("prompt_4"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_compressed_embedding_cache_eviction_edge_case_4() {
        let cache = CompressedEmbeddingCache::new(Duration::from_millis(40));
        let large_payload = "A".repeat(400);
        cache.set("prompt_4", &large_payload);
        assert_eq!(cache.get("prompt_4").unwrap(), large_payload);
        thread::sleep(Duration::from_millis(90));
        assert_eq!(cache.get("prompt_4"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_local_embedding_cache_eviction_edge_case_5() {
        let cache = LocalEmbeddingCache::new(Duration::from_millis(50));
        cache.set("prompt_5", "response_5");
        assert_eq!(cache.get("prompt_5"), Some("response_5".to_string()));
        thread::sleep(Duration::from_millis(100));
        assert_eq!(cache.get("prompt_5"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_compressed_embedding_cache_eviction_edge_case_5() {
        let cache = CompressedEmbeddingCache::new(Duration::from_millis(50));
        let large_payload = "A".repeat(500);
        cache.set("prompt_5", &large_payload);
        assert_eq!(cache.get("prompt_5").unwrap(), large_payload);
        thread::sleep(Duration::from_millis(100));
        assert_eq!(cache.get("prompt_5"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_local_embedding_cache_eviction_edge_case_6() {
        let cache = LocalEmbeddingCache::new(Duration::from_millis(60));
        cache.set("prompt_6", "response_6");
        assert_eq!(cache.get("prompt_6"), Some("response_6".to_string()));
        thread::sleep(Duration::from_millis(110));
        assert_eq!(cache.get("prompt_6"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_compressed_embedding_cache_eviction_edge_case_6() {
        let cache = CompressedEmbeddingCache::new(Duration::from_millis(60));
        let large_payload = "A".repeat(600);
        cache.set("prompt_6", &large_payload);
        assert_eq!(cache.get("prompt_6").unwrap(), large_payload);
        thread::sleep(Duration::from_millis(110));
        assert_eq!(cache.get("prompt_6"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_local_embedding_cache_eviction_edge_case_7() {
        let cache = LocalEmbeddingCache::new(Duration::from_millis(70));
        cache.set("prompt_7", "response_7");
        assert_eq!(cache.get("prompt_7"), Some("response_7".to_string()));
        thread::sleep(Duration::from_millis(120));
        assert_eq!(cache.get("prompt_7"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_compressed_embedding_cache_eviction_edge_case_7() {
        let cache = CompressedEmbeddingCache::new(Duration::from_millis(70));
        let large_payload = "A".repeat(700);
        cache.set("prompt_7", &large_payload);
        assert_eq!(cache.get("prompt_7").unwrap(), large_payload);
        thread::sleep(Duration::from_millis(120));
        assert_eq!(cache.get("prompt_7"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_local_embedding_cache_eviction_edge_case_8() {
        let cache = LocalEmbeddingCache::new(Duration::from_millis(80));
        cache.set("prompt_8", "response_8");
        assert_eq!(cache.get("prompt_8"), Some("response_8".to_string()));
        thread::sleep(Duration::from_millis(130));
        assert_eq!(cache.get("prompt_8"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_compressed_embedding_cache_eviction_edge_case_8() {
        let cache = CompressedEmbeddingCache::new(Duration::from_millis(80));
        let large_payload = "A".repeat(800);
        cache.set("prompt_8", &large_payload);
        assert_eq!(cache.get("prompt_8").unwrap(), large_payload);
        thread::sleep(Duration::from_millis(130));
        assert_eq!(cache.get("prompt_8"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_local_embedding_cache_eviction_edge_case_9() {
        let cache = LocalEmbeddingCache::new(Duration::from_millis(90));
        cache.set("prompt_9", "response_9");
        assert_eq!(cache.get("prompt_9"), Some("response_9".to_string()));
        thread::sleep(Duration::from_millis(140));
        assert_eq!(cache.get("prompt_9"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_compressed_embedding_cache_eviction_edge_case_9() {
        let cache = CompressedEmbeddingCache::new(Duration::from_millis(90));
        let large_payload = "A".repeat(900);
        cache.set("prompt_9", &large_payload);
        assert_eq!(cache.get("prompt_9").unwrap(), large_payload);
        thread::sleep(Duration::from_millis(140));
        assert_eq!(cache.get("prompt_9"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_local_embedding_cache_eviction_edge_case_10() {
        let cache = LocalEmbeddingCache::new(Duration::from_millis(100));
        cache.set("prompt_10", "response_10");
        assert_eq!(cache.get("prompt_10"), Some("response_10".to_string()));
        thread::sleep(Duration::from_millis(150));
        assert_eq!(cache.get("prompt_10"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_compressed_embedding_cache_eviction_edge_case_10() {
        let cache = CompressedEmbeddingCache::new(Duration::from_millis(100));
        let large_payload = "A".repeat(1000);
        cache.set("prompt_10", &large_payload);
        assert_eq!(cache.get("prompt_10").unwrap(), large_payload);
        thread::sleep(Duration::from_millis(150));
        assert_eq!(cache.get("prompt_10"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_local_embedding_cache_eviction_edge_case_11() {
        let cache = LocalEmbeddingCache::new(Duration::from_millis(110));
        cache.set("prompt_11", "response_11");
        assert_eq!(cache.get("prompt_11"), Some("response_11".to_string()));
        thread::sleep(Duration::from_millis(160));
        assert_eq!(cache.get("prompt_11"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_compressed_embedding_cache_eviction_edge_case_11() {
        let cache = CompressedEmbeddingCache::new(Duration::from_millis(110));
        let large_payload = "A".repeat(1100);
        cache.set("prompt_11", &large_payload);
        assert_eq!(cache.get("prompt_11").unwrap(), large_payload);
        thread::sleep(Duration::from_millis(160));
        assert_eq!(cache.get("prompt_11"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_local_embedding_cache_eviction_edge_case_12() {
        let cache = LocalEmbeddingCache::new(Duration::from_millis(120));
        cache.set("prompt_12", "response_12");
        assert_eq!(cache.get("prompt_12"), Some("response_12".to_string()));
        thread::sleep(Duration::from_millis(170));
        assert_eq!(cache.get("prompt_12"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_compressed_embedding_cache_eviction_edge_case_12() {
        let cache = CompressedEmbeddingCache::new(Duration::from_millis(120));
        let large_payload = "A".repeat(1200);
        cache.set("prompt_12", &large_payload);
        assert_eq!(cache.get("prompt_12").unwrap(), large_payload);
        thread::sleep(Duration::from_millis(170));
        assert_eq!(cache.get("prompt_12"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_local_embedding_cache_eviction_edge_case_13() {
        let cache = LocalEmbeddingCache::new(Duration::from_millis(130));
        cache.set("prompt_13", "response_13");
        assert_eq!(cache.get("prompt_13"), Some("response_13".to_string()));
        thread::sleep(Duration::from_millis(180));
        assert_eq!(cache.get("prompt_13"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_compressed_embedding_cache_eviction_edge_case_13() {
        let cache = CompressedEmbeddingCache::new(Duration::from_millis(130));
        let large_payload = "A".repeat(1300);
        cache.set("prompt_13", &large_payload);
        assert_eq!(cache.get("prompt_13").unwrap(), large_payload);
        thread::sleep(Duration::from_millis(180));
        assert_eq!(cache.get("prompt_13"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_local_embedding_cache_eviction_edge_case_14() {
        let cache = LocalEmbeddingCache::new(Duration::from_millis(140));
        cache.set("prompt_14", "response_14");
        assert_eq!(cache.get("prompt_14"), Some("response_14".to_string()));
        thread::sleep(Duration::from_millis(190));
        assert_eq!(cache.get("prompt_14"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_compressed_embedding_cache_eviction_edge_case_14() {
        let cache = CompressedEmbeddingCache::new(Duration::from_millis(140));
        let large_payload = "A".repeat(1400);
        cache.set("prompt_14", &large_payload);
        assert_eq!(cache.get("prompt_14").unwrap(), large_payload);
        thread::sleep(Duration::from_millis(190));
        assert_eq!(cache.get("prompt_14"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_local_embedding_cache_eviction_edge_case_15() {
        let cache = LocalEmbeddingCache::new(Duration::from_millis(150));
        cache.set("prompt_15", "response_15");
        assert_eq!(cache.get("prompt_15"), Some("response_15".to_string()));
        thread::sleep(Duration::from_millis(200));
        assert_eq!(cache.get("prompt_15"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_compressed_embedding_cache_eviction_edge_case_15() {
        let cache = CompressedEmbeddingCache::new(Duration::from_millis(150));
        let large_payload = "A".repeat(1500);
        cache.set("prompt_15", &large_payload);
        assert_eq!(cache.get("prompt_15").unwrap(), large_payload);
        thread::sleep(Duration::from_millis(200));
        assert_eq!(cache.get("prompt_15"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_local_embedding_cache_eviction_edge_case_16() {
        let cache = LocalEmbeddingCache::new(Duration::from_millis(160));
        cache.set("prompt_16", "response_16");
        assert_eq!(cache.get("prompt_16"), Some("response_16".to_string()));
        thread::sleep(Duration::from_millis(210));
        assert_eq!(cache.get("prompt_16"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_compressed_embedding_cache_eviction_edge_case_16() {
        let cache = CompressedEmbeddingCache::new(Duration::from_millis(160));
        let large_payload = "A".repeat(1600);
        cache.set("prompt_16", &large_payload);
        assert_eq!(cache.get("prompt_16").unwrap(), large_payload);
        thread::sleep(Duration::from_millis(210));
        assert_eq!(cache.get("prompt_16"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_local_embedding_cache_eviction_edge_case_17() {
        let cache = LocalEmbeddingCache::new(Duration::from_millis(170));
        cache.set("prompt_17", "response_17");
        assert_eq!(cache.get("prompt_17"), Some("response_17".to_string()));
        thread::sleep(Duration::from_millis(220));
        assert_eq!(cache.get("prompt_17"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_compressed_embedding_cache_eviction_edge_case_17() {
        let cache = CompressedEmbeddingCache::new(Duration::from_millis(170));
        let large_payload = "A".repeat(1700);
        cache.set("prompt_17", &large_payload);
        assert_eq!(cache.get("prompt_17").unwrap(), large_payload);
        thread::sleep(Duration::from_millis(220));
        assert_eq!(cache.get("prompt_17"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_local_embedding_cache_eviction_edge_case_18() {
        let cache = LocalEmbeddingCache::new(Duration::from_millis(180));
        cache.set("prompt_18", "response_18");
        assert_eq!(cache.get("prompt_18"), Some("response_18".to_string()));
        thread::sleep(Duration::from_millis(230));
        assert_eq!(cache.get("prompt_18"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_compressed_embedding_cache_eviction_edge_case_18() {
        let cache = CompressedEmbeddingCache::new(Duration::from_millis(180));
        let large_payload = "A".repeat(1800);
        cache.set("prompt_18", &large_payload);
        assert_eq!(cache.get("prompt_18").unwrap(), large_payload);
        thread::sleep(Duration::from_millis(230));
        assert_eq!(cache.get("prompt_18"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_local_embedding_cache_eviction_edge_case_19() {
        let cache = LocalEmbeddingCache::new(Duration::from_millis(190));
        cache.set("prompt_19", "response_19");
        assert_eq!(cache.get("prompt_19"), Some("response_19".to_string()));
        thread::sleep(Duration::from_millis(240));
        assert_eq!(cache.get("prompt_19"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_compressed_embedding_cache_eviction_edge_case_19() {
        let cache = CompressedEmbeddingCache::new(Duration::from_millis(190));
        let large_payload = "A".repeat(1900);
        cache.set("prompt_19", &large_payload);
        assert_eq!(cache.get("prompt_19").unwrap(), large_payload);
        thread::sleep(Duration::from_millis(240));
        assert_eq!(cache.get("prompt_19"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_local_embedding_cache_eviction_edge_case_20() {
        let cache = LocalEmbeddingCache::new(Duration::from_millis(200));
        cache.set("prompt_20", "response_20");
        assert_eq!(cache.get("prompt_20"), Some("response_20".to_string()));
        thread::sleep(Duration::from_millis(250));
        assert_eq!(cache.get("prompt_20"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_compressed_embedding_cache_eviction_edge_case_20() {
        let cache = CompressedEmbeddingCache::new(Duration::from_millis(200));
        let large_payload = "A".repeat(2000);
        cache.set("prompt_20", &large_payload);
        assert_eq!(cache.get("prompt_20").unwrap(), large_payload);
        thread::sleep(Duration::from_millis(250));
        assert_eq!(cache.get("prompt_20"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_local_embedding_cache_eviction_edge_case_21() {
        let cache = LocalEmbeddingCache::new(Duration::from_millis(210));
        cache.set("prompt_21", "response_21");
        assert_eq!(cache.get("prompt_21"), Some("response_21".to_string()));
        thread::sleep(Duration::from_millis(260));
        assert_eq!(cache.get("prompt_21"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_compressed_embedding_cache_eviction_edge_case_21() {
        let cache = CompressedEmbeddingCache::new(Duration::from_millis(210));
        let large_payload = "A".repeat(2100);
        cache.set("prompt_21", &large_payload);
        assert_eq!(cache.get("prompt_21").unwrap(), large_payload);
        thread::sleep(Duration::from_millis(260));
        assert_eq!(cache.get("prompt_21"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_local_embedding_cache_eviction_edge_case_22() {
        let cache = LocalEmbeddingCache::new(Duration::from_millis(220));
        cache.set("prompt_22", "response_22");
        assert_eq!(cache.get("prompt_22"), Some("response_22".to_string()));
        thread::sleep(Duration::from_millis(270));
        assert_eq!(cache.get("prompt_22"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_compressed_embedding_cache_eviction_edge_case_22() {
        let cache = CompressedEmbeddingCache::new(Duration::from_millis(220));
        let large_payload = "A".repeat(2200);
        cache.set("prompt_22", &large_payload);
        assert_eq!(cache.get("prompt_22").unwrap(), large_payload);
        thread::sleep(Duration::from_millis(270));
        assert_eq!(cache.get("prompt_22"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_local_embedding_cache_eviction_edge_case_23() {
        let cache = LocalEmbeddingCache::new(Duration::from_millis(230));
        cache.set("prompt_23", "response_23");
        assert_eq!(cache.get("prompt_23"), Some("response_23".to_string()));
        thread::sleep(Duration::from_millis(280));
        assert_eq!(cache.get("prompt_23"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_compressed_embedding_cache_eviction_edge_case_23() {
        let cache = CompressedEmbeddingCache::new(Duration::from_millis(230));
        let large_payload = "A".repeat(2300);
        cache.set("prompt_23", &large_payload);
        assert_eq!(cache.get("prompt_23").unwrap(), large_payload);
        thread::sleep(Duration::from_millis(280));
        assert_eq!(cache.get("prompt_23"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_local_embedding_cache_eviction_edge_case_24() {
        let cache = LocalEmbeddingCache::new(Duration::from_millis(240));
        cache.set("prompt_24", "response_24");
        assert_eq!(cache.get("prompt_24"), Some("response_24".to_string()));
        thread::sleep(Duration::from_millis(290));
        assert_eq!(cache.get("prompt_24"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_compressed_embedding_cache_eviction_edge_case_24() {
        let cache = CompressedEmbeddingCache::new(Duration::from_millis(240));
        let large_payload = "A".repeat(2400);
        cache.set("prompt_24", &large_payload);
        assert_eq!(cache.get("prompt_24").unwrap(), large_payload);
        thread::sleep(Duration::from_millis(290));
        assert_eq!(cache.get("prompt_24"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_local_embedding_cache_eviction_edge_case_25() {
        let cache = LocalEmbeddingCache::new(Duration::from_millis(250));
        cache.set("prompt_25", "response_25");
        assert_eq!(cache.get("prompt_25"), Some("response_25".to_string()));
        thread::sleep(Duration::from_millis(300));
        assert_eq!(cache.get("prompt_25"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_compressed_embedding_cache_eviction_edge_case_25() {
        let cache = CompressedEmbeddingCache::new(Duration::from_millis(250));
        let large_payload = "A".repeat(2500);
        cache.set("prompt_25", &large_payload);
        assert_eq!(cache.get("prompt_25").unwrap(), large_payload);
        thread::sleep(Duration::from_millis(300));
        assert_eq!(cache.get("prompt_25"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_local_embedding_cache_eviction_edge_case_26() {
        let cache = LocalEmbeddingCache::new(Duration::from_millis(260));
        cache.set("prompt_26", "response_26");
        assert_eq!(cache.get("prompt_26"), Some("response_26".to_string()));
        thread::sleep(Duration::from_millis(310));
        assert_eq!(cache.get("prompt_26"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_compressed_embedding_cache_eviction_edge_case_26() {
        let cache = CompressedEmbeddingCache::new(Duration::from_millis(260));
        let large_payload = "A".repeat(2600);
        cache.set("prompt_26", &large_payload);
        assert_eq!(cache.get("prompt_26").unwrap(), large_payload);
        thread::sleep(Duration::from_millis(310));
        assert_eq!(cache.get("prompt_26"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_local_embedding_cache_eviction_edge_case_27() {
        let cache = LocalEmbeddingCache::new(Duration::from_millis(270));
        cache.set("prompt_27", "response_27");
        assert_eq!(cache.get("prompt_27"), Some("response_27".to_string()));
        thread::sleep(Duration::from_millis(320));
        assert_eq!(cache.get("prompt_27"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_compressed_embedding_cache_eviction_edge_case_27() {
        let cache = CompressedEmbeddingCache::new(Duration::from_millis(270));
        let large_payload = "A".repeat(2700);
        cache.set("prompt_27", &large_payload);
        assert_eq!(cache.get("prompt_27").unwrap(), large_payload);
        thread::sleep(Duration::from_millis(320));
        assert_eq!(cache.get("prompt_27"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_local_embedding_cache_eviction_edge_case_28() {
        let cache = LocalEmbeddingCache::new(Duration::from_millis(280));
        cache.set("prompt_28", "response_28");
        assert_eq!(cache.get("prompt_28"), Some("response_28".to_string()));
        thread::sleep(Duration::from_millis(330));
        assert_eq!(cache.get("prompt_28"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_compressed_embedding_cache_eviction_edge_case_28() {
        let cache = CompressedEmbeddingCache::new(Duration::from_millis(280));
        let large_payload = "A".repeat(2800);
        cache.set("prompt_28", &large_payload);
        assert_eq!(cache.get("prompt_28").unwrap(), large_payload);
        thread::sleep(Duration::from_millis(330));
        assert_eq!(cache.get("prompt_28"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_local_embedding_cache_eviction_edge_case_29() {
        let cache = LocalEmbeddingCache::new(Duration::from_millis(290));
        cache.set("prompt_29", "response_29");
        assert_eq!(cache.get("prompt_29"), Some("response_29".to_string()));
        thread::sleep(Duration::from_millis(340));
        assert_eq!(cache.get("prompt_29"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_compressed_embedding_cache_eviction_edge_case_29() {
        let cache = CompressedEmbeddingCache::new(Duration::from_millis(290));
        let large_payload = "A".repeat(2900);
        cache.set("prompt_29", &large_payload);
        assert_eq!(cache.get("prompt_29").unwrap(), large_payload);
        thread::sleep(Duration::from_millis(340));
        assert_eq!(cache.get("prompt_29"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_local_embedding_cache_eviction_edge_case_30() {
        let cache = LocalEmbeddingCache::new(Duration::from_millis(300));
        cache.set("prompt_30", "response_30");
        assert_eq!(cache.get("prompt_30"), Some("response_30".to_string()));
        thread::sleep(Duration::from_millis(350));
        assert_eq!(cache.get("prompt_30"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_compressed_embedding_cache_eviction_edge_case_30() {
        let cache = CompressedEmbeddingCache::new(Duration::from_millis(300));
        let large_payload = "A".repeat(3000);
        cache.set("prompt_30", &large_payload);
        assert_eq!(cache.get("prompt_30").unwrap(), large_payload);
        thread::sleep(Duration::from_millis(350));
        assert_eq!(cache.get("prompt_30"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_local_embedding_cache_eviction_edge_case_31() {
        let cache = LocalEmbeddingCache::new(Duration::from_millis(310));
        cache.set("prompt_31", "response_31");
        assert_eq!(cache.get("prompt_31"), Some("response_31".to_string()));
        thread::sleep(Duration::from_millis(360));
        assert_eq!(cache.get("prompt_31"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_compressed_embedding_cache_eviction_edge_case_31() {
        let cache = CompressedEmbeddingCache::new(Duration::from_millis(310));
        let large_payload = "A".repeat(3100);
        cache.set("prompt_31", &large_payload);
        assert_eq!(cache.get("prompt_31").unwrap(), large_payload);
        thread::sleep(Duration::from_millis(360));
        assert_eq!(cache.get("prompt_31"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_local_embedding_cache_eviction_edge_case_32() {
        let cache = LocalEmbeddingCache::new(Duration::from_millis(320));
        cache.set("prompt_32", "response_32");
        assert_eq!(cache.get("prompt_32"), Some("response_32".to_string()));
        thread::sleep(Duration::from_millis(370));
        assert_eq!(cache.get("prompt_32"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_compressed_embedding_cache_eviction_edge_case_32() {
        let cache = CompressedEmbeddingCache::new(Duration::from_millis(320));
        let large_payload = "A".repeat(3200);
        cache.set("prompt_32", &large_payload);
        assert_eq!(cache.get("prompt_32").unwrap(), large_payload);
        thread::sleep(Duration::from_millis(370));
        assert_eq!(cache.get("prompt_32"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_local_embedding_cache_eviction_edge_case_33() {
        let cache = LocalEmbeddingCache::new(Duration::from_millis(330));
        cache.set("prompt_33", "response_33");
        assert_eq!(cache.get("prompt_33"), Some("response_33".to_string()));
        thread::sleep(Duration::from_millis(380));
        assert_eq!(cache.get("prompt_33"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_compressed_embedding_cache_eviction_edge_case_33() {
        let cache = CompressedEmbeddingCache::new(Duration::from_millis(330));
        let large_payload = "A".repeat(3300);
        cache.set("prompt_33", &large_payload);
        assert_eq!(cache.get("prompt_33").unwrap(), large_payload);
        thread::sleep(Duration::from_millis(380));
        assert_eq!(cache.get("prompt_33"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_local_embedding_cache_eviction_edge_case_34() {
        let cache = LocalEmbeddingCache::new(Duration::from_millis(340));
        cache.set("prompt_34", "response_34");
        assert_eq!(cache.get("prompt_34"), Some("response_34".to_string()));
        thread::sleep(Duration::from_millis(390));
        assert_eq!(cache.get("prompt_34"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_compressed_embedding_cache_eviction_edge_case_34() {
        let cache = CompressedEmbeddingCache::new(Duration::from_millis(340));
        let large_payload = "A".repeat(3400);
        cache.set("prompt_34", &large_payload);
        assert_eq!(cache.get("prompt_34").unwrap(), large_payload);
        thread::sleep(Duration::from_millis(390));
        assert_eq!(cache.get("prompt_34"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_local_embedding_cache_eviction_edge_case_35() {
        let cache = LocalEmbeddingCache::new(Duration::from_millis(350));
        cache.set("prompt_35", "response_35");
        assert_eq!(cache.get("prompt_35"), Some("response_35".to_string()));
        thread::sleep(Duration::from_millis(400));
        assert_eq!(cache.get("prompt_35"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_compressed_embedding_cache_eviction_edge_case_35() {
        let cache = CompressedEmbeddingCache::new(Duration::from_millis(350));
        let large_payload = "A".repeat(3500);
        cache.set("prompt_35", &large_payload);
        assert_eq!(cache.get("prompt_35").unwrap(), large_payload);
        thread::sleep(Duration::from_millis(400));
        assert_eq!(cache.get("prompt_35"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_local_embedding_cache_eviction_edge_case_36() {
        let cache = LocalEmbeddingCache::new(Duration::from_millis(360));
        cache.set("prompt_36", "response_36");
        assert_eq!(cache.get("prompt_36"), Some("response_36".to_string()));
        thread::sleep(Duration::from_millis(410));
        assert_eq!(cache.get("prompt_36"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_compressed_embedding_cache_eviction_edge_case_36() {
        let cache = CompressedEmbeddingCache::new(Duration::from_millis(360));
        let large_payload = "A".repeat(3600);
        cache.set("prompt_36", &large_payload);
        assert_eq!(cache.get("prompt_36").unwrap(), large_payload);
        thread::sleep(Duration::from_millis(410));
        assert_eq!(cache.get("prompt_36"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_local_embedding_cache_eviction_edge_case_37() {
        let cache = LocalEmbeddingCache::new(Duration::from_millis(370));
        cache.set("prompt_37", "response_37");
        assert_eq!(cache.get("prompt_37"), Some("response_37".to_string()));
        thread::sleep(Duration::from_millis(420));
        assert_eq!(cache.get("prompt_37"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_compressed_embedding_cache_eviction_edge_case_37() {
        let cache = CompressedEmbeddingCache::new(Duration::from_millis(370));
        let large_payload = "A".repeat(3700);
        cache.set("prompt_37", &large_payload);
        assert_eq!(cache.get("prompt_37").unwrap(), large_payload);
        thread::sleep(Duration::from_millis(420));
        assert_eq!(cache.get("prompt_37"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_local_embedding_cache_eviction_edge_case_38() {
        let cache = LocalEmbeddingCache::new(Duration::from_millis(380));
        cache.set("prompt_38", "response_38");
        assert_eq!(cache.get("prompt_38"), Some("response_38".to_string()));
        thread::sleep(Duration::from_millis(430));
        assert_eq!(cache.get("prompt_38"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_compressed_embedding_cache_eviction_edge_case_38() {
        let cache = CompressedEmbeddingCache::new(Duration::from_millis(380));
        let large_payload = "A".repeat(3800);
        cache.set("prompt_38", &large_payload);
        assert_eq!(cache.get("prompt_38").unwrap(), large_payload);
        thread::sleep(Duration::from_millis(430));
        assert_eq!(cache.get("prompt_38"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_local_embedding_cache_eviction_edge_case_39() {
        let cache = LocalEmbeddingCache::new(Duration::from_millis(390));
        cache.set("prompt_39", "response_39");
        assert_eq!(cache.get("prompt_39"), Some("response_39".to_string()));
        thread::sleep(Duration::from_millis(440));
        assert_eq!(cache.get("prompt_39"), None);
        assert_eq!(cache.prune(), 1);
    }

    #[test]
    fn test_compressed_embedding_cache_eviction_edge_case_39() {
        let cache = CompressedEmbeddingCache::new(Duration::from_millis(390));
        let large_payload = "A".repeat(3900);
        cache.set("prompt_39", &large_payload);
        assert_eq!(cache.get("prompt_39").unwrap(), large_payload);
        thread::sleep(Duration::from_millis(440));
        assert_eq!(cache.get("prompt_39"), None);
        assert_eq!(cache.prune(), 1);
    }
}
