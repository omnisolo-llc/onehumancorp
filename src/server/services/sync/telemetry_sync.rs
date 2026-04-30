use sqlx::{PgPool, Row, query};
use chrono::{DateTime, Utc};
use tracing::error;
use serde_json::Value;

pub struct TelemetrySyncDaemon {
    pool: PgPool,
    cloud_url: String,
}

impl TelemetrySyncDaemon {
    pub fn new(pool: PgPool, cloud_url: String) -> Self {
        Self { pool, cloud_url }
    }

    pub fn start(self) {
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(60));
            loop {
                interval.tick().await;
                if let Err(e) = self.sync_metrics().await {
                    error!("Failed to sync metrics: {}", e);
                }
            }
        });
    }

    async fn sync_metrics(&self) -> Result<(), Box<dyn std::error::Error>> {
        let rows = query(
            "SELECT id, metric_name, metric_type, value, labels_json, timestamp
             FROM telemetry_buffer WHERE sync_status = 'pending' LIMIT 100"
        )
        .fetch_all(&self.pool)
        .await?;

        if rows.is_empty() {
            return Ok(());
        }

        let mut batch = Vec::new();
        let mut ids = Vec::new();

        for row in rows {
            let id: i32 = row.get("id");
            let metric_name: String = row.get("metric_name");
            let metric_type: String = row.get("metric_type");
            let value: f32 = row.get("value");
            let labels_json: String = row.get("labels_json");
            let timestamp: DateTime<Utc> = row.get("timestamp");

            batch.push(serde_json::json!({
                "metric_name": metric_name,
                "metric_type": metric_type,
                "value": value,
                "labels": serde_json::from_str::<Value>(&labels_json).unwrap_or(Value::Null),
                "timestamp": timestamp,
            }));
            ids.push(id);
        }

        let client = reqwest::Client::new();
        let res = client.post(format!("{}/api/telemetry/sync", self.cloud_url))
            .json(&batch)
            .send()
            .await?;

        if res.status().is_success() {
            for id in ids {
                query("DELETE FROM telemetry_buffer WHERE id = $1")
                    .bind(id)
                    .execute(&self.pool)
                    .await?;
            }
        } else {
            error!("Cloud API returned error: {}", res.status());
        }

        Ok(())
    }
}
