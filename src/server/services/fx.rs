use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FxRate {
    pub base_currency: String,
    pub target_currency: String,
    pub rate: f64,
}

pub struct FxCacheService {
    redis_client: redis::Client,
}

impl FxCacheService {
    pub fn new(redis_client: redis::Client) -> Self {
        Self { redis_client }
    }

    pub async fn get_rate(&self, base: &str, target: &str) -> Result<FxRate, String> {
        let mut conn = self.redis_client.get_async_connection().await.map_err(|e| e.to_string())?;
        let key = format!("ohc:fx_rates:{}_{}", base, target);

        let rate: Option<f64> = redis::cmd("GET")
            .arg(&key)
            .query_async(&mut conn)
            .await
            .map_err(|e| e.to_string())?;

        if let Some(r) = rate {
            Ok(FxRate {
                base_currency: base.to_string(),
                target_currency: target.to_string(),
                rate: r,
            })
        } else {
            // Fallback default
            Ok(FxRate {
                base_currency: base.to_string(),
                target_currency: target.to_string(),
                rate: 1.0,
            })
        }
    }
}
