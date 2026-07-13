use sqlx::PgPool;
use std::sync::Arc;
use redis::AsyncCommands;

#[derive(Clone)]
pub struct FxCacheService {
    pool: Arc<PgPool>,
    redis_client: Option<redis::Client>,
}

impl FxCacheService {
    pub fn new(pool: Arc<PgPool>, redis_client: Option<redis::Client>) -> Self {
        Self { pool, redis_client }
    }

    pub async fn apply_fx(
        &self,
        amount_cents: i64,
        from_currency: &str,
        to_currency: &str,
    ) -> Result<i64, String> {
        if from_currency == to_currency {
            return Ok(amount_cents);
        }

        let rate = self.get_fx_rate(from_currency, to_currency).await?;
        let final_amount = (amount_cents as f64 * rate).round() as i64;
        Ok(final_amount)
    }

    pub async fn get_fx_rate(&self, from: &str, to: &str) -> Result<f64, String> {
        let cache_key = format!("ohc:fx_rates:{}:{}", from, to);

        if let Some(client) = &self.redis_client {
            if let Ok(mut con) = client.get_multiplexed_async_connection().await {
                let cached_rate: redis::RedisResult<Option<f64>> = con.get(&cache_key).await;
                if let Ok(Some(rate)) = cached_rate {
                    return Ok(rate);
                }
            }
        }

        let rate: f64 = sqlx::query_scalar("SELECT rate FROM ohc_fx_rates WHERE from_currency = $1 AND to_currency = $2")
            .bind(from)
            .bind(to)
            .fetch_one(&*self.pool)
            .await
            .map_err(|e| format!("FX rate not found for {} to {}: {}", from, to, e))?;

        if let Some(client) = &self.redis_client {
            if let Ok(mut con) = client.get_multiplexed_async_connection().await {
                let _: redis::RedisResult<()> = con.set_ex(&cache_key, rate, 3600).await;
            }
        }

        Ok(rate)
    }
}
