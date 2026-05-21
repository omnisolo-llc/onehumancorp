use std::collections::HashMap;
use std::sync::Arc;
use tokio::time::{interval, Duration};
use crate::db::DB;

pub struct TokenForecastWorker {
    db: Arc<DB>,
    token_usage_history: std::sync::RwLock<HashMap<String, Vec<i64>>>,
}

impl TokenForecastWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self {
            db,
            token_usage_history: std::sync::RwLock::new(HashMap::new()),
        }
    }

    pub fn start(self: Arc<Self>) {
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(60));
            loop {
                interval.tick().await;
                if let Err(e) = self.calculate_token_burn_rate().await {
                    tracing::error!("TokenForecastWorker: Failed to calculate token burn rate: {}", e);
                }
            }
        });
    }

    pub async fn calculate_token_burn_rate(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let threshold = chrono::Utc::now() - chrono::Duration::hours(24);
        let rows = sqlx::query("SELECT value, labels_json FROM telemetry_buffer WHERE metric_name = 'ohc_token_usage_total' AND timestamp >= $1")
            .bind(threshold)
            .fetch_all(&self.db.pool)
            .await?;

        let mut computed = HashMap::new();
        for row in rows {
            use sqlx::Row;
            let val: f32 = row.get("value");
            let labels_json: String = row.get("labels_json");
            if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&labels_json) {
                if let Some(org_id) = parsed.get("organization_id").and_then(|v| v.as_str()) {
                    *computed.entry(org_id.to_string()).or_insert(0) += val as i64;
                }
            }
        }

        let mut forecasts_to_record = Vec::new();

        {
            let mut history = self.token_usage_history.write().unwrap();
            let mut active_orgs = HashMap::new();

            for (org_id, total_tokens) in computed {
                active_orgs.insert(org_id.clone(), true);
                if total_tokens > 0 {
                    let hist = history.entry(org_id.clone()).or_insert_with(Vec::new);
                    hist.push(total_tokens);

                    if hist.len() > 5 {
                        hist.remove(0);
                    }

                    if hist.len() > 1 {
                        let mut ema_rate = 0.0;
                        let alpha = 0.3;
                        for i in 1..hist.len() {
                            let current_rate = (hist[i] - hist[i - 1]) as f64;
                            if i == 1 {
                                ema_rate = current_rate;
                            } else {
                                ema_rate = alpha * current_rate + (1.0 - alpha) * ema_rate;
                            }
                        }
                        let forecast = hist.last().unwrap() + (ema_rate * 43200.0) as i64;
                        forecasts_to_record.push((org_id.clone(), forecast as f32));
                    }
                } else {
                    history.remove(&org_id);
                }
            }

            history.retain(|org_id, _| active_orgs.contains_key(org_id));
        }

        let budget_threshold = 100_000.0; // Trigger alert if burn rate > 100k

        for (org_id, forecast) in forecasts_to_record {
            let _ = ::server_telemetry::record_token_usage_forecast(&self.db.pool, &org_id, forecast).await;
            if forecast > budget_threshold {
                let _ = ::server_telemetry::record_token_budget_alert(&self.db.pool, &org_id, "forecast_exceeded").await;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_calculate_token_burn_rate_no_usage() {
        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());
        let pool = match tokio::time::timeout(std::time::Duration::from_millis(500), sqlx::PgPool::connect(&db_url)).await {
            Ok(Ok(p)) => p,
            _ => return, // Gracefully exit if DB is not available in sandbox or times out
        };
        let db = Arc::new(DB { pool: pool.clone(), store: crate::db::DbStore::Postgres });
        let worker = TokenForecastWorker::new(db);

        let result = worker.calculate_token_burn_rate().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_calculate_token_burn_rate_with_usage() {
        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());
        let pool = match tokio::time::timeout(std::time::Duration::from_millis(500), sqlx::PgPool::connect(&db_url)).await {
            Ok(Ok(p)) => p,
            _ => return,
        };

        // Insert some mock telemetry data
        let org_id = "test_org_forecast";
        let payload = serde_json::json!({ "organization_id": org_id }).to_string();

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

        let db = Arc::new(DB { pool: pool.clone(), store: crate::db::DbStore::Postgres });
        let worker = TokenForecastWorker::new(db);

        // First execution to populate usage
        let _ = sqlx::query("INSERT INTO telemetry_buffer (metric_name, metric_type, value, labels_json, timestamp, sync_status) VALUES ($1, $2, $3, $4, $5, 'pending')")
            .bind("ohc_token_usage_total")
            .bind("counter")
            .bind(1000.0)
            .bind(&payload)
            .bind(chrono::Utc::now())
            .execute(&pool).await;

        worker.calculate_token_burn_rate().await.unwrap();

        // Second execution to calculate rate (requires >1 history entries)
        let _ = sqlx::query("INSERT INTO telemetry_buffer (metric_name, metric_type, value, labels_json, timestamp, sync_status) VALUES ($1, $2, $3, $4, $5, 'pending')")
            .bind("ohc_token_usage_total")
            .bind("counter")
            .bind(5000.0) // Large increase
            .bind(&payload)
            .bind(chrono::Utc::now())
            .execute(&pool).await;

        worker.calculate_token_burn_rate().await.unwrap();

        // Check if forecast is recorded
        let row: (String,) = sqlx::query_as("SELECT labels_json FROM telemetry_buffer WHERE metric_name = 'ohc_token_burn_rate_forecast' ORDER BY timestamp DESC LIMIT 1")
            .fetch_one(&pool).await.unwrap();

        let parsed: serde_json::Value = serde_json::from_str(&row.0).unwrap();
        // The pii redactor redacts based on specific keys or contents, organization_id is safe though it could be REDACTED
        assert!(parsed.get("organization_id").is_some());
    }

    #[tokio::test]
    async fn test_calculate_token_burn_rate_alert_triggered() {
        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());
        let pool = match tokio::time::timeout(std::time::Duration::from_millis(500), sqlx::PgPool::connect(&db_url)).await {
            Ok(Ok(p)) => p,
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

        let org_id = "alert_org_forecast";
        let payload = serde_json::json!({ "organization_id": org_id }).to_string();

        let db = Arc::new(DB { pool: pool.clone(), store: crate::db::DbStore::Postgres });
        let worker = TokenForecastWorker::new(db);

        // Huge increase to trigger threshold > 100k
        let _ = sqlx::query("INSERT INTO telemetry_buffer (metric_name, metric_type, value, labels_json, timestamp, sync_status) VALUES ($1, $2, $3, $4, $5, 'pending')")
            .bind("ohc_token_usage_total")
            .bind("counter")
            .bind(100.0)
            .bind(&payload)
            .bind(chrono::Utc::now())
            .execute(&pool).await;

        worker.calculate_token_burn_rate().await.unwrap();

        let _ = sqlx::query("INSERT INTO telemetry_buffer (metric_name, metric_type, value, labels_json, timestamp, sync_status) VALUES ($1, $2, $3, $4, $5, 'pending')")
            .bind("ohc_token_usage_total")
            .bind("counter")
            .bind(500000.0)
            .bind(&payload)
            .bind(chrono::Utc::now())
            .execute(&pool).await;

        worker.calculate_token_burn_rate().await.unwrap();

        // Check if budget alert is recorded
        let row: (String,) = sqlx::query_as("SELECT labels_json FROM telemetry_buffer WHERE metric_name = 'ohc_token_budget_alert_total' ORDER BY timestamp DESC LIMIT 1")
            .fetch_one(&pool).await.unwrap();

        let parsed: serde_json::Value = serde_json::from_str(&row.0).unwrap();
        assert!(parsed.get("organization_id").is_some());
        assert_eq!(parsed["alert_type"], "forecast_exceeded");
    }
}
