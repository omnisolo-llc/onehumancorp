use std::sync::{Arc, OnceLock};

pub struct RedisPool {
    client: redis::Client,
}

impl RedisPool {
    pub fn new(url: &str) -> Result<Self, redis::RedisError> {
        let client = redis::Client::open(url)?;
        Ok(Self { client })
    }

    pub fn client(&self) -> &redis::Client {
        &self.client
    }

    pub async fn get_async_connection(
        &self,
    ) -> Result<redis::aio::MultiplexedConnection, redis::RedisError> {
        self.client.get_multiplexed_tokio_connection().await
    }

    pub async fn get_pubsub(
        &self,
    ) -> Result<redis::aio::PubSub, redis::RedisError> {
        self.client.get_async_pubsub().await
    }
}

static REDIS_POOL: OnceLock<Arc<RedisPool>> = OnceLock::new();

pub fn get_redis_pool() -> Option<&'static Arc<RedisPool>> {
    if crate::is_standalone_runtime() {
        return None;
    }
    Some(REDIS_POOL.get_or_init(|| {
        let url = std::env::var("REDIS_URL")
            .unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
        Arc::new(RedisPool::new(&url).expect("Failed to create Redis pool"))
    }))
}

pub fn get_redis_client() -> Option<redis::Client> {
    get_redis_pool().map(|pool| pool.client().clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_redis_pool_returns_same_instance() {
        let pool1 = get_redis_pool();
        let pool2 = get_redis_pool();
        if let (Some(p1), Some(p2)) = (pool1, pool2) {
            assert!(Arc::ptr_eq(p1, p2));
        }
    }

    #[test]
    fn test_returns_none_in_standalone_mode() {
        let _ = std::env::set_var("OHC_STANDALONE_MODE", "true");
        assert!(get_redis_pool().is_none());
    }
}
