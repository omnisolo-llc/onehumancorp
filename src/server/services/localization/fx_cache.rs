use redis::AsyncCommands;
use std::sync::Arc;

pub struct FxCacheService {
    redis_client: Arc<redis::Client>,
}

impl FxCacheService {
    pub fn new(redis_client: Arc<redis::Client>) -> Self {
        Self { redis_client }
    }

    pub async fn get_rate(&self, from: &str, to: &str) -> Result<Option<f64>, redis::RedisError> {
        if from == to {
            return Ok(Some(1.0));
        }
        let mut conn = self.redis_client.get_multiplexed_async_connection().await?;
        let key = format!("ohc:fx_rates:{}_{}", from, to);
        let rate: Option<f64> = conn.get(key).await?;
        Ok(rate)
    }

    pub async fn set_rate(&self, from: &str, to: &str, rate: f64) -> Result<(), redis::RedisError> {
        let mut conn = self.redis_client.get_multiplexed_async_connection().await?;
        let key = format!("ohc:fx_rates:{}_{}", from, to);
        // Cache for 1 hour
        let _: () = conn.set_ex(key, rate, 3600).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_rate_same_currency() {
        let client = Arc::new(redis::Client::open("redis://127.0.0.1/").unwrap());
        let service = FxCacheService::new(client);
        let rate = service.get_rate("USD", "USD").await;
        assert!(rate.is_ok());
        assert_eq!(rate.unwrap(), Some(1.0));
    }

    #[tokio::test]
    async fn test_set_and_get_rate() {
        let client = Arc::new(redis::Client::open("redis://127.0.0.1/").unwrap());
        let service = FxCacheService::new(client);

        let set_res = service.set_rate("USD", "EUR", 0.95).await;

        if set_res.is_ok() {
            let rate = service.get_rate("USD", "EUR").await;
            assert!(rate.is_ok());
            assert_eq!(rate.unwrap(), Some(0.95));
        }
    }
}
