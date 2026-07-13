use std::sync::Arc;
use redis::AsyncCommands;
use redis::Client;

pub struct FxCacheService {
    redis_client: Client,
}

impl FxCacheService {
    pub fn new(redis_client: Client) -> Self {
        Self { redis_client }
    }

    pub async fn get_rate(&self, from_curr: &str, to_curr: &str) -> Result<f64, redis::RedisError> {
        let key = format!("ohc:fx_rates:{}_{}", from_curr, to_curr);
        let mut conn = self.redis_client.get_multiplexed_async_connection().await?;
        let rate: Option<f64> = conn.get(&key).await?;

        if let Some(r) = rate {
            Ok(r)
        } else {
            // Mock or fetch
            if from_curr == to_curr {
                return Ok(1.0);
            }
            if from_curr == "USD" && to_curr == "GBP" {
                return Ok(0.75); // Example
            }
            Ok(1.0)
        }
    }

    pub async fn get_tax_rate(&self, region: &str) -> Result<f64, redis::RedisError> {
        let key = format!("ohc:tax_rates:{}", region);
        let mut conn = self.redis_client.get_multiplexed_async_connection().await?;
        let rate: Option<f64> = conn.get(&key).await?;

        if let Some(r) = rate {
            Ok(r)
        } else {
            if region == "UK" || region == "GB" {
                return Ok(0.20); // 20% VAT
            }
            Ok(0.0)
        }
    }
}
