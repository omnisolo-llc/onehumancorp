use std::collections::HashMap;
use std::sync::Arc;
use tokio::time::{interval, Duration};
use sqlx::PgPool;
use chrono::{Utc, DateTime};
use serde_json::Value;

pub struct Forecaster {
    pool: PgPool,
    token_usage_samples: std::sync::RwLock<HashMap<String, Vec<(DateTime<Utc>, i64)>>>,
}

impl Forecaster {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            token_usage_samples: std::sync::RwLock::new(HashMap::new()),
        }
    }

    pub fn start(self: Arc<Self>) {
        tokio::spawn(async move {
            // Run every 5 minutes
            let mut interval = interval(Duration::from_secs(300));
            loop {
                interval.tick().await;
                if let Err(e) = self.run_forecast_cycle().await {
                    tracing::error!("Forecaster: Failed to run forecast cycle: {}", e);
                }
            }
        });
    }

    pub async fn run_forecast_cycle(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let now = Utc::now();
        // Look back only at the most recent samples since the last cycle
        let lookback = now - chrono::Duration::minutes(6);

        // 1. Fetch recent token usage from telemetry_buffer
        let rows = sqlx::query("SELECT value, labels_json FROM telemetry_buffer WHERE metric_name = 'ohc_token_usage_total' AND timestamp >= $1")
            .bind(lookback)
            .fetch_all(&self.pool)
            .await?;

        let mut recent_usage = HashMap::new();
        for row in rows {
            use sqlx::Row;
            let val: f32 = row.get("value");
            let labels_json: String = row.get("labels_json");

            if let Ok(parsed) = serde_json::from_str::<Value>(&labels_json) {
                if let Some(org_id) = parsed.get("organization_id").and_then(|v| v.as_str()) {
                    *recent_usage.entry(org_id.to_string()).or_insert(0) += val as i64;
                }
            }
        }

        let mut forecasts = Vec::new();

        {
            let mut samples = self.token_usage_samples.write().unwrap();
            for (org_id, tokens) in recent_usage {
                let org_samples = samples.entry(org_id.clone()).or_insert_with(Vec::new);
                org_samples.push((now, tokens));

                // Keep only last 1 hour of 5-minute samples for moving average (12 samples)
                if org_samples.len() > 12 {
                    org_samples.remove(0);
                }

                // Calculate average tokens per 5 minutes from our window
                let sum_tokens: i64 = org_samples.iter().map(|(_, t)| *t).sum();
                let avg_tokens_per_sample = sum_tokens as f64 / org_samples.len() as f64;

                // 5 minutes = 300 seconds. 24 hours = 86400 seconds.
                // Number of 5-minute samples in 24 hours = 86400 / 300 = 288
                let predicted_24h = avg_tokens_per_sample * 288.0;
                forecasts.push((org_id, predicted_24h as f32));
            }
        }

        // 2. Record forecasts and alerts
        let budget_threshold = 100_000.0;

        for (org_id, forecast) in forecasts {
            let _ = crate::record_token_burn_rate_predicted_24h(&self.pool, &org_id, forecast).await;

            if forecast > budget_threshold {
                let _ = crate::record_token_budget_alert(&self.pool, &org_id, "predicted_24h_exceeded").await;
                tracing::warn!(forecast = %forecast, "Token budget forecast exceeded for tenant");
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;

    async fn setup_test_db() -> PgPool {
        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());
        PgPool::connect(&db_url).await.unwrap()
    }

    #[tokio::test]
    async fn test_forecaster_logic() {
        let pool = match tokio::time::timeout(Duration::from_millis(500), setup_test_db()).await {
            Ok(p) => p,
            _ => return,
        };

        // Ensure table exists
        let _ = sqlx::query(
            "CREATE TABLE IF NOT EXISTS telemetry_buffer (
                id SERIAL PRIMARY KEY,
                metric_name TEXT NOT NULL,
                metric_type TEXT NOT NULL,
                value REAL NOT NULL,
                labels_json TEXT NOT NULL,
                timestamp TIMESTAMPTZ NOT NULL,
                sync_status TEXT NOT NULL
            )"
        ).execute(&pool).await;

        let org_id = "test_org_forecaster_v2";
        let payload = serde_json::json!({ "organization_id": org_id }).to_string();
        let now = Utc::now();

        // Insert some recent usage
        let _ = sqlx::query("INSERT INTO telemetry_buffer (metric_name, metric_type, value, labels_json, timestamp, sync_status) VALUES ($1, $2, $3, $4, $5, 'pending')")
            .bind("ohc_token_usage_total")
            .bind("counter")
            .bind(100.0)
            .bind(&payload)
            .bind(now)
            .execute(&pool).await;

        let forecaster = Forecaster::new(pool.clone());

        // Cycle 1: First sample
        forecaster.run_forecast_cycle().await.unwrap();

        // Check if predicted_24h is recorded
        let row: (f32,) = sqlx::query_as("SELECT value FROM telemetry_buffer WHERE metric_name = 'ohc_token_burn_rate_predicted_24h' ORDER BY timestamp DESC LIMIT 1")
            .fetch_one(&pool).await.unwrap();

        // 100 tokens per 5 mins * 288 (5-min intervals in 24h) = 28800
        assert!((row.0 - 28800.0).abs() < 1.0);
    }
}
