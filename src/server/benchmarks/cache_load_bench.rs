use std::time::{Instant, Duration};
use crate::utils::cache::HybridCache;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct BigCachePayload {
    id: String,
    data: Vec<u8>,
    nested: std::collections::HashMap<String, String>,
}

pub async fn bench_cache_load() {
    tracing::info!("Benchmarking Cache under heavy load...");

    let redis_client = None; // Start with local only for benchmark purity
    let cache = HybridCache::<BigCachePayload>::with_capacity(redis_client, 5000);

    let iterations = 10000;
    let payload_size_bytes = 1024 * 50; // 50KB

    let mut sample_payload = BigCachePayload {
        id: "sample".to_string(),
        data: vec![0u8; payload_size_bytes],
        nested: std::collections::HashMap::new(),
    };
    for i in 0..100 {
        sample_payload.nested.insert(format!("key_{}", i), format!("val_{}", i));
    }

    let start_write = Instant::now();
    for i in 0..iterations {
        let key = format!("bench_key_{}", i);
        let mut pl = sample_payload.clone();
        pl.id = key.clone();
        cache.set(&key, pl, Duration::from_secs(60)).await;
    }
    let write_duration = start_write.elapsed();
    println!("Cache Write: {} items ({} bytes each) took {:?}", iterations, payload_size_bytes, write_duration);

    let start_read = Instant::now();
    let mut hits = 0;
    for i in 0..iterations {
        let key = format!("bench_key_{}", i);
        if cache.get(&key).await.is_some() {
            hits += 1;
        }
    }
    let read_duration = start_read.elapsed();
    println!("Cache Read: {} iterations, {} hits, took {:?}", iterations, hits, read_duration);

    let start_evict = Instant::now();
    for i in 0..iterations {
        let key = format!("bench_key_{}", i);
        cache.invalidate(&key).await;
    }
    let evict_duration = start_evict.elapsed();
    println!("Cache Evict: {} items took {:?}", iterations, evict_duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_bench_cache_load() {
        bench_cache_load().await;
    }
}
