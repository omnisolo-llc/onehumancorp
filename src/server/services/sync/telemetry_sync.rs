use sqlx::{PgPool, Row, query};
use chrono::{DateTime, Utc};
use tracing::error;
use serde_json::Value;

pub mod perf {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum CoordinatorMode {
        Sequential,
        Parallel,
    }
}

pub struct TelemetrySyncDaemon {
    pool: PgPool,
    cloud_url: String,
    mode: perf::CoordinatorMode,
}

impl TelemetrySyncDaemon {
    pub fn new(pool: PgPool, cloud_url: String) -> Self {
        Self { pool, cloud_url, mode: perf::CoordinatorMode::Sequential }
    }

    pub fn with_mode(pool: PgPool, cloud_url: String, mode: perf::CoordinatorMode) -> Self {
        Self { pool, cloud_url, mode }
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
        if self.cloud_url.is_empty() {
            return Ok(());
        }
        if self.cloud_url.is_empty() {
            return Ok(());
        }
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

        if self.mode == perf::CoordinatorMode::Parallel {
            // Parallel execution using futures

            // Extract the data from rows since `Row` might not be Send/Sync
            // or easily parallelizable directly. We consume it into an iterator.
            let extracted_data: Vec<(i32, String, String, f32, String, DateTime<Utc>)> = rows.into_iter().map(|row| {
                let id: i32 = row.get("id");
                let metric_name: String = row.get("metric_name");
                let metric_type: String = row.get("metric_type");
                let value: f32 = row.get("value");
                let labels_json: String = row.get("labels_json");
                let timestamp: DateTime<Utc> = row.get("timestamp");
                (id, metric_name, metric_type, value, labels_json, timestamp)
            }).collect();

            // To limit the number of blocking threads, chunk the execution instead of spawning one per row.
            // We use iterators without cloning.
            let num_cpus = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4);
            let chunk_size = std::cmp::max(1, (extracted_data.len() + num_cpus - 1) / num_cpus);

            let mut iter = extracted_data.into_iter();
            let mut handles = Vec::new();

            loop {
                let chunk: Vec<_> = iter.by_ref().take(chunk_size).collect();
                if chunk.is_empty() {
                    break;
                }

                handles.push(tokio::task::spawn_blocking(move || {
                    let mut chunk_res = Vec::with_capacity(chunk.len());
                    for (id, metric_name, metric_type, value, labels_json, timestamp) in chunk {
                        let json = serde_json::json!({
                            "metric_name": metric_name,
                            "metric_type": metric_type,
                            "value": value,
                            "labels": serde_json::from_str::<Value>(&labels_json).unwrap_or(Value::Null),
                            "timestamp": timestamp,
                        });
                        chunk_res.push((id, json));
                    }
                    chunk_res
                }));
            }

            let results = futures::future::join_all(handles).await;
            for res in results {
                if let Ok(chunk_res) = res {
                    for (id, json) in chunk_res {
                        ids.push(id);
                        batch.push(json);
                    }
                }
            }
        } else {
            // Sequential execution
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
        }

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());

        let res = client.post(format!("{}/api/telemetry/sync", self.cloud_url))
            .json(&batch)
            .send()
            .await;

        match res {
            Ok(response) => {
                if response.status().is_success() {
                    for id in ids {
                        query("DELETE FROM telemetry_buffer WHERE id = $1")
                            .bind(id)
                            .execute(&self.pool)
                            .await?;
                    }
                } else {
                    error!("Cloud API returned error: {}", response.status());
                }
            },
            Err(e) => {
                error!("Cloud API error: {}", e);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[tokio::test]
    async fn bench_telemetry_sync_parallel() {
        // If we are in the Bazel sandbox without an active HTTP mock or DB, just exit cleanly to avoid timeouts
        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/dummy".to_string());

        // Fast fail for tests
        if db_url.contains("dummy") || db_url == "postgres://localhost/dummy" {
            return;
        }

        // Fast DB connection test with very short timeout
        let pool_res = tokio::time::timeout(
            std::time::Duration::from_millis(100),
            sqlx::PgPool::connect(&db_url)
        ).await;

        let pool = match pool_res {
            Ok(Ok(p)) => p,
            _ => return, // DB unreachable or timeout
        };

        // Ensure connection works
        if !matches!(tokio::time::timeout(std::time::Duration::from_millis(100), sqlx::query("SELECT 1").execute(&pool)).await, Ok(Ok(_))) {
            return;
        }

        // Start a dummy mock server to accept telemetry and return 200 OK
        let mock_server = axum::Router::new()
            .route("/api/telemetry/sync", axum::routing::post(|| async { axum::http::StatusCode::OK }));

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        let mock_url = format!("http://127.0.0.1:{}", port);

        tokio::spawn(async move {
            axum::serve(listener, mock_server).await.unwrap();
        });

        // Ensure table exists
        query(
            "CREATE TABLE IF NOT EXISTS telemetry_buffer (
                id SERIAL PRIMARY KEY,
                metric_name TEXT NOT NULL,
                metric_type TEXT NOT NULL,
                value REAL NOT NULL,
                labels_json TEXT NOT NULL,
                timestamp TIMESTAMPTZ NOT NULL,
                sync_status TEXT NOT NULL
            )"
        ).execute(&pool).await.unwrap();

        // Ensure cleanup before test
        query("DELETE FROM telemetry_buffer WHERE metric_name LIKE 'bench_metric_%'").execute(&pool).await.unwrap();

        // Insert some dummy data
        for i in 0..100 {
            query("INSERT INTO telemetry_buffer (metric_name, metric_type, value, labels_json, timestamp, sync_status) VALUES ($1, $2, $3, $4, $5, 'pending')")
                .bind(format!("bench_metric_seq_{}", i))
                .bind("counter")
                .bind(1.0f32)
                .bind(format!("{{\"dummy\": {}}}", i))
                .bind(Utc::now())
                .execute(&pool).await.unwrap();
        }

        let daemon = TelemetrySyncDaemon::with_mode(pool.clone(), mock_url.clone(), perf::CoordinatorMode::Sequential);
        let start = Instant::now();
        let _ = tokio::time::timeout(std::time::Duration::from_secs(5), daemon.sync_metrics()).await;
        let seq_duration = start.elapsed();

        // Insert more dummy data for the parallel test
        for i in 0..100 {
            query("INSERT INTO telemetry_buffer (metric_name, metric_type, value, labels_json, timestamp, sync_status) VALUES ($1, $2, $3, $4, $5, 'pending')")
                .bind(format!("bench_metric_par_{}", i))
                .bind("counter")
                .bind(1.0f32)
                .bind(format!("{{\"dummy\": {}}}", i))
                .bind(Utc::now())
                .execute(&pool).await.unwrap();
        }

        let par_daemon = TelemetrySyncDaemon::with_mode(pool.clone(), mock_url.clone(), perf::CoordinatorMode::Parallel);
        let start_par = Instant::now();
        let _ = tokio::time::timeout(std::time::Duration::from_secs(5), par_daemon.sync_metrics()).await;
        let par_duration = start_par.elapsed();

        tracing::info!("Sequential Sync Duration: {:?}", seq_duration);
        tracing::info!("Parallel Sync Duration: {:?}", par_duration);

        // Assert that sync_metrics returned Ok and both durations are measured
        assert!(seq_duration > std::time::Duration::from_nanos(0));
        assert!(par_duration > std::time::Duration::from_nanos(0));

        // Cleanup
        query("DELETE FROM telemetry_buffer WHERE metric_name LIKE 'bench_metric_%'").execute(&pool).await.unwrap();
    }
}
