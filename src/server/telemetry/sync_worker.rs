use sqlx::{query, SqlitePool, Row};
use chrono::{DateTime, Utc};
use serde_json::Value;

#[derive(Debug)]
pub struct Metric {
    pub id: i32,
    pub metric_name: String,
    pub metric_type: String,
    pub value: f32,
    pub labels_json: String,
    pub timestamp: DateTime<Utc>,
}

pub async fn sync_metrics_to_cloud(db: &SqlitePool, cloud_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let rows = query("SELECT id, metric_name, metric_type, value, labels_json, timestamp FROM local_telemetry_buffer WHERE sync_status = 'pending' LIMIT 100")
        .fetch_all(db)
        .await?;

    if rows.is_empty() {
        return Ok(());
    }

    let mut metrics = Vec::new();
    let mut ids = Vec::new();

    for row in rows {
        let id: i32 = row.get("id");
        let metric_name: String = row.get("metric_name");
        let metric_type: String = row.get("metric_type");
        let value: f32 = row.get("value");
        let labels_json: String = row.get("labels_json");

        let ts_str: String = row.get("timestamp");
        let timestamp = DateTime::parse_from_rfc3339(&ts_str)?.with_timezone(&Utc);

        let labels: Value = serde_json::from_str(&labels_json).unwrap_or(Value::Null);

        metrics.push(serde_json::json!({
            "metric_name": metric_name,
            "metric_type": metric_type,
            "value": value,
            "labels": labels,
            "timestamp": timestamp,
        }));
        ids.push(id);
    }

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .unwrap_or_else(|_| reqwest::Client::new());

    let res = client.post(format!("{}/api/telemetry/sync", cloud_url))
        .json(&metrics)
        .send()
        .await;

    match res {
        Ok(response) => {
            if response.status().is_success() {
                for id in ids {
                    query("UPDATE local_telemetry_buffer SET sync_status = 'synced' WHERE id = $1")
                        .bind(id)
                        .execute(db)
                        .await?;
                }
            } else {
                tracing::error!("Failed to sync metrics, status code: {}", response.status());
            }
        },
        Err(e) => {
            tracing::error!("Error syncing telemetry: {}", e);
        }
    }

    Ok(())
}

pub fn start_telemetry_sync_worker(db: SqlitePool, cloud_url: String, interval: std::time::Duration) {
    tokio::spawn(async move {
        let mut ticker = tokio::time::interval(interval);
        loop {
            ticker.tick().await;
            if let Err(e) = sync_metrics_to_cloud(&db, &cloud_url).await {
                tracing::error!("Error in telemetry sync worker: {}", e);
            }
        }
    });
}
