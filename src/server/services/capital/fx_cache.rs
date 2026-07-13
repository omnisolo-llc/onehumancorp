use redis::AsyncCommands;
use std::sync::Arc;

pub struct FxCacheService {
    redis: redis::Client,
    pool: Arc<sqlx::PgPool>,
}

impl FxCacheService {
    pub fn new(redis: redis::Client, pool: Arc<sqlx::PgPool>) -> Self {
        Self { redis, pool }
    }

    pub async fn get_rate(&self, from: &str, to: &str) -> Result<f64, String> {
        if from == to {
            return Ok(1.0);
        }

        let key = format!("ohc:fx_rates:{}_{}", from, to);
        let mut conn = self.redis.get_async_connection().await.map_err(|e| e.to_string())?;

        let cached: Option<f64> = conn.get(&key).await.map_err(|e| e.to_string())?;
        if let Some(rate) = cached {
            return Ok(rate);
        }

        // Fallback to db
        let row = sqlx::query("SELECT rate FROM ohc_fx_rates WHERE from_currency = $1 AND to_currency = $2")
            .bind(from)
            .bind(to)
            .fetch_optional(&*self.pool)
            .await
            .map_err(|e| e.to_string())?;

        match row {
            Some(r) => {
                use sqlx::Row;
                let rate: f64 = r.get("rate");
                let _: () = conn.set_ex(&key, rate, 3600).await.map_err(|e| e.to_string())?;
                Ok(rate)
            }
            None => Err(format!("No exchange rate found from {} to {}", from, to)),
        }
    }
}
