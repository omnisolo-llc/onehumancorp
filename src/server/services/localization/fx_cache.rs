use redis::AsyncCommands;
use sqlx::PgPool;
use std::sync::Arc;

pub struct FxCacheService {
    redis_client: redis::Client,
    db_pool: Arc<PgPool>,
}

impl FxCacheService {
    pub fn new(redis_client: redis::Client, db_pool: Arc<PgPool>) -> Self {
        Self {
            redis_client,
            db_pool,
        }
    }

    pub async fn get_rate(&self, from: &str, to: &str) -> Result<f64, String> {
        if from == to {
            return Ok(1.0);
        }

        let cache_key = format!("ohc:fx_rates:{}_{}", from, to);

        // Try getting from cache first
        let mut conn = self.redis_client.get_multiplexed_tokio_connection().await.map_err(|e| e.to_string())?;
        let cached_rate: redis::RedisResult<String> = conn.get(&cache_key).await;

        if let Ok(rate_str) = cached_rate {
            if let Ok(rate) = rate_str.parse::<f64>() {
                return Ok(rate);
            }
        }

        // Fallback to database
        let row = sqlx::query("SELECT rate FROM ohc_fx_rates WHERE from_currency = $1 AND to_currency = $2")
            .bind(from)
            .bind(to)
            .fetch_optional(&*self.db_pool)
            .await
            .map_err(|e| e.to_string())?;

        match row {
            Some(r) => {
                use sqlx::Row;
                let rate: f64 = r.get("rate");

                // Set cache for future requests (1 hour TTL)
                let _: redis::RedisResult<()> = conn.set_ex(&cache_key, rate.to_string(), 3600).await;

                Ok(rate)
            }
            None => Err(format!("No exchange rate found from {} to {}", from, to)),
        }
    }
}
