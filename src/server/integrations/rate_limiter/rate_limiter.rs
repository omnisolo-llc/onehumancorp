use std::collections::HashMap;
use std::sync::RwLock;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct RateLimitInfo {
    pub limit: u32,
    pub remaining: u32,
    pub reset_in: Duration,
}

#[async_trait::async_trait]
pub trait RateLimiterManager: Send + Sync {
    async fn request_tokens(&self, tenant_id: &str, bucket: &str, amount: u32) -> Result<bool, String>;
    async fn get_rate_limit_status(&self, tenant_id: &str, bucket: &str) -> Result<RateLimitInfo, String>;
}

struct InMemoryBucket {
    tokens: f64,
    capacity: f64,
    last_refill: Instant,
    refill_rate: f64, // tokens per second
}

pub struct InMemoryRateLimiter {
    buckets: RwLock<HashMap<String, InMemoryBucket>>,
    default_capacity: f64,
    default_refill_rate: f64,
}

impl InMemoryRateLimiter {
    pub fn new(capacity: u32, refill_rate: f64) -> Self {
        InMemoryRateLimiter {
            buckets: RwLock::new(HashMap::new()),
            default_capacity: capacity as f64,
            default_refill_rate: refill_rate,
        }
    }

    fn get_key(tenant_id: &str, bucket: &str) -> String {
        format!("{}:{}", tenant_id, bucket)
    }

    fn refill(&self, key: &str) {
        let mut buckets = self.buckets.write().unwrap();
        let bucket = buckets.entry(key.to_string()).or_insert_with(|| InMemoryBucket {
            tokens: self.default_capacity,
            capacity: self.default_capacity,
            last_refill: Instant::now(),
            refill_rate: self.default_refill_rate,
        });

        let now = Instant::now();
        let elapsed = now.duration_since(bucket.last_refill).as_secs_f64();
        let tokens_to_add = elapsed * bucket.refill_rate;

        if tokens_to_add > 0.0 {
            bucket.tokens = (bucket.tokens + tokens_to_add).min(bucket.capacity);
            bucket.last_refill = now;
        }
    }
}

#[async_trait::async_trait]
impl RateLimiterManager for InMemoryRateLimiter {
    async fn request_tokens(&self, tenant_id: &str, bucket: &str, amount: u32) -> Result<bool, String> {
        let key = Self::get_key(tenant_id, bucket);
        self.refill(&key);

        let mut buckets = self.buckets.write().unwrap();
        if let Some(b) = buckets.get_mut(&key) {
            let amount_f64 = amount as f64;
            if b.tokens >= amount_f64 {
                b.tokens -= amount_f64;
                return Ok(true);
            }
            return Ok(false);
        }
        Err("Bucket not found".to_string())
    }

    async fn get_rate_limit_status(&self, tenant_id: &str, bucket: &str) -> Result<RateLimitInfo, String> {
        let key = Self::get_key(tenant_id, bucket);
        self.refill(&key);

        let buckets = self.buckets.read().unwrap();
        if let Some(b) = buckets.get(&key) {
            let tokens_needed_for_full = b.capacity - b.tokens;
            let time_to_full = if tokens_needed_for_full > 0.0 {
                Duration::from_secs_f64(tokens_needed_for_full / b.refill_rate)
            } else {
                Duration::from_secs(0)
            };

            return Ok(RateLimitInfo {
                limit: b.capacity as u32,
                remaining: b.tokens as u32,
                reset_in: time_to_full,
            });
        }

        Ok(RateLimitInfo {
            limit: self.default_capacity as u32,
            remaining: self.default_capacity as u32,
            reset_in: Duration::from_secs(0),
        })
    }
}

pub struct RedisRateLimiter {
    client: redis::Client,
    default_capacity: u32,
    refill_rate: f64,
}

impl RedisRateLimiter {
    pub fn new(redis_url: &str, capacity: u32, refill_rate: f64) -> Result<Self, String> {
        let client = redis::Client::open(redis_url).map_err(|e| e.to_string())?;
        Ok(RedisRateLimiter {
            client,
            default_capacity: capacity,
            refill_rate,
        })
    }

    fn get_key(tenant_id: &str, bucket: &str) -> String {
        format!("rate_limit:token_bucket:{}:{}", tenant_id, bucket)
    }
}

#[async_trait::async_trait]
impl RateLimiterManager for RedisRateLimiter {
    async fn request_tokens(&self, tenant_id: &str, bucket: &str, amount: u32) -> Result<bool, String> {
        let key = Self::get_key(tenant_id, bucket);
        let mut con = self.client.get_multiplexed_async_connection().await.map_err(|e| e.to_string())?;

        // LUA script for token bucket
        let script = redis::Script::new(r"
            local key = KEYS[1]
            local capacity = tonumber(ARGV[1])
            local refill_rate = tonumber(ARGV[2])
            local requested = tonumber(ARGV[3])
            local now = tonumber(ARGV[4])

            local bucket = redis.call('HMGET', key, 'tokens', 'last_refill')
            local tokens = tonumber(bucket[1]) or capacity
            local last_refill = tonumber(bucket[2]) or now

            local elapsed = math.max(0, now - last_refill)
            local tokens_to_add = elapsed * refill_rate
            tokens = math.min(capacity, tokens + tokens_to_add)

            if tokens >= requested then
                tokens = tokens - requested
                redis.call('HMSET', key, 'tokens', tokens, 'last_refill', now)
                local time_to_full = (capacity - tokens) / refill_rate
                redis.call('EXPIRE', key, math.ceil(time_to_full))
                return 1
            else
                redis.call('HMSET', key, 'tokens', tokens, 'last_refill', now)
                local time_to_full = (capacity - tokens) / refill_rate
                redis.call('EXPIRE', key, math.ceil(time_to_full))
                return 0
            end
        ");

        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs_f64();
        let result: i32 = script.key(&key)
            .arg(self.default_capacity)
            .arg(self.refill_rate)
            .arg(amount)
            .arg(now)
            .invoke_async(&mut con).await.map_err(|e| e.to_string())?;

        Ok(result == 1)
    }

    async fn get_rate_limit_status(&self, tenant_id: &str, bucket: &str) -> Result<RateLimitInfo, String> {
        let key = Self::get_key(tenant_id, bucket);
        let mut con = self.client.get_multiplexed_async_connection().await.map_err(|e| e.to_string())?;

        let script = redis::Script::new(r"
            local key = KEYS[1]
            local capacity = tonumber(ARGV[1])
            local refill_rate = tonumber(ARGV[2])
            local now = tonumber(ARGV[3])

            local bucket = redis.call('HMGET', key, 'tokens', 'last_refill')
            local tokens = tonumber(bucket[1]) or capacity
            local last_refill = tonumber(bucket[2]) or now

            local elapsed = math.max(0, now - last_refill)
            local tokens_to_add = elapsed * refill_rate
            tokens = math.min(capacity, tokens + tokens_to_add)

            return tokens
        ");

        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs_f64();
        let tokens: f64 = script.key(&key)
            .arg(self.default_capacity)
            .arg(self.refill_rate)
            .arg(now)
            .invoke_async(&mut con).await.map_err(|e| e.to_string())?;

        let tokens_needed_for_full = self.default_capacity as f64 - tokens;
        let time_to_full = if tokens_needed_for_full > 0.0 {
            Duration::from_secs_f64(tokens_needed_for_full / self.refill_rate)
        } else {
            Duration::from_secs(0)
        };

        Ok(RateLimitInfo {
            limit: self.default_capacity,
            remaining: tokens as u32,
            reset_in: time_to_full,
        })
    }
}

pub fn create_rate_limiter(
    is_cloud: bool,
    redis_url: Option<&str>,
    capacity: u32,
    refill_rate: f64
) -> Box<dyn RateLimiterManager> {
    if is_cloud {
        if let Some(url) = redis_url {
            if let Ok(limiter) = RedisRateLimiter::new(url, capacity, refill_rate) {
                return Box::new(limiter);
            }
        }
    }
    Box::new(InMemoryRateLimiter::new(capacity, refill_rate))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_in_memory_rate_limiter() {
        let limiter = InMemoryRateLimiter::new(10, 5.0); // 10 tokens, 5 tokens/sec

        // Initial state
        let status = limiter.get_rate_limit_status("tenant1", "toolA").await.unwrap();
        assert_eq!(status.limit, 10);
        assert_eq!(status.remaining, 10);

        // Consume some tokens
        assert!(limiter.request_tokens("tenant1", "toolA", 4).await.unwrap());

        let status = limiter.get_rate_limit_status("tenant1", "toolA").await.unwrap();
        assert_eq!(status.limit, 10);
        assert_eq!(status.remaining, 6);

        // Consume more
        assert!(limiter.request_tokens("tenant1", "toolA", 5).await.unwrap());

        // Should fail
        assert!(!limiter.request_tokens("tenant1", "toolA", 2).await.unwrap());

        // Wait for refill
        std::thread::sleep(Duration::from_millis(1100));

        // Should succeed after refill
        assert!(limiter.request_tokens("tenant1", "toolA", 2).await.unwrap());
    }
}
