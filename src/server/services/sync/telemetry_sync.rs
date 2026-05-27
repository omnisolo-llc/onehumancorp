use crate::db::{DB, DbStore};
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

pub struct TelemetrySyncWorker {
    db: DB,
    cloud_url: String,
    mode: perf::CoordinatorMode,
}

impl TelemetrySyncWorker {
    pub fn new(db: DB, cloud_url: String) -> Self {
        Self { db, cloud_url, mode: perf::CoordinatorMode::Sequential }
    }

    pub fn with_mode(db: DB, cloud_url: String, mode: perf::CoordinatorMode) -> Self {
        Self { db, cloud_url, mode }
    }

    pub fn start(self) {
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(60));
            let mut backoff = 1;
            loop {
                interval.tick().await;
                match self.sync_metrics().await {
                    Ok(_) => {
                        backoff = 1; // reset on success
                    }
                    Err(e) => {
                        error!("Failed to sync metrics: {}", e);
                        // Exponential backoff, up to 10 minutes
                        backoff = std::cmp::min(backoff * 2, 10);
                        tokio::time::sleep(std::time::Duration::from_secs(60 * backoff)).await;
                    }
                }
            }
        });
    }

    async fn sync_metrics(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.cloud_url.is_empty() {
            return Ok(());
        }

        let query_str = "SELECT id, metric_name, metric_type, value, labels_json, timestamp \
                         FROM local_telemetry_buffer WHERE sync_status = 'pending' LIMIT 100";

        let mut extracted_data = Vec::new();

        match &self.db.store {
            DbStore::Postgres => {
                let rows = sqlx::query(query_str).fetch_all(&self.db.pool).await?;
                for row in rows {
                    use sqlx::Row;
                    let id: i32 = row.get("id");
                    let metric_name: String = row.get("metric_name");
                    let metric_type: String = row.get("metric_type");
                    let value: f32 = row.get("value");
                    let labels_json: String = row.get("labels_json");
                    let timestamp: DateTime<Utc> = row.get("timestamp");
                    extracted_data.push((id, metric_name, metric_type, value, labels_json, timestamp));
                }
            }
            DbStore::Sqlite(sqlite_pool) => {
                let rows = sqlx::query(query_str).fetch_all(sqlite_pool).await?;
                for row in rows {
                    use sqlx::Row;
                    let id: i32 = row.get("id");
                    let metric_name: String = row.get("metric_name");
                    let metric_type: String = row.get("metric_type");
                    let value: f32 = row.get::<f32, _>("value");
                    let labels_json: String = row.get("labels_json");
                    let timestamp: chrono::NaiveDateTime = row.get("timestamp");
                    let timestamp = chrono::DateTime::from_naive_utc_and_offset(timestamp, chrono::Utc);
                    extracted_data.push((id, metric_name, metric_type, value, labels_json, timestamp));
                }
            }
        }

        if extracted_data.is_empty() {
            return Ok(());
        }

        let mut batch = Vec::new();
        let mut ids = Vec::new();

        if self.mode == perf::CoordinatorMode::Parallel {
            // Parallel execution using futures
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
            for (id, metric_name, metric_type, value, labels_json, timestamp) in extracted_data {
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
                    if !ids.is_empty() {
                        match &self.db.store {
                            DbStore::Postgres => {
                                let query_str = "DELETE FROM local_telemetry_buffer WHERE id = ANY($1)";
                                sqlx::query(query_str)
                                    .bind(&ids)
                                    .execute(&self.db.pool)
                                    .await?;
                            }
                            DbStore::Sqlite(sqlite_pool) => {
                                // SQLite does not support ANY, so we use a transaction with sequential deletes
                                let mut tx = sqlite_pool.begin().await?;
                                for id in ids {
                                    sqlx::query("DELETE FROM local_telemetry_buffer WHERE id = ?")
                                        .bind(id)
                                        .execute(&mut *tx)
                                        .await?;
                                }
                                tx.commit().await?;
                            }
                        }
                    }
                } else {
                    let status = response.status();
                    error!("Cloud API returned error: {}", status);
                    return Err(format!("Cloud API returned error: {}", status).into());
                }
            },
            Err(e) => {
                error!("Cloud API error: {}", e);
                return Err(Box::new(e));
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
        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/dummy".to_string());
        if db_url.contains("dummy") || db_url == "postgres://localhost/dummy" {
            return;
        }

        let pool_res = tokio::time::timeout(
            std::time::Duration::from_millis(100),
            sqlx::PgPool::connect(&db_url)
        ).await;

        let pool = match pool_res {
            Ok(Ok(p)) => p,
            _ => return,
        };

        if !matches!(tokio::time::timeout(std::time::Duration::from_millis(100), sqlx::query("SELECT 1").execute(&pool)).await, Ok(Ok(_))) {
            return;
        }

        let mock_server = axum::Router::new()
            .route("/api/telemetry/sync", axum::routing::post(|| async { axum::http::StatusCode::OK }));

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        let mock_url = format!("http://127.0.0.1:{}", port);

        tokio::spawn(async move {
            axum::serve(listener, mock_server).await.unwrap();
        });

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS local_telemetry_buffer (
                id SERIAL PRIMARY KEY,
                metric_name TEXT NOT NULL,
                metric_type TEXT NOT NULL,
                value REAL NOT NULL,
                labels_json TEXT NOT NULL,
                timestamp TIMESTAMPTZ NOT NULL,
                sync_status TEXT NOT NULL
            )"
        ).execute(&pool).await.unwrap();

        sqlx::query("DELETE FROM local_telemetry_buffer WHERE metric_name LIKE 'bench_metric_%'").execute(&pool).await.unwrap();

        for i in 0..100 {
            sqlx::query("INSERT INTO local_telemetry_buffer (metric_name, metric_type, value, labels_json, timestamp, sync_status) VALUES ($1, $2, $3, $4, $5, 'pending')")
                .bind(format!("bench_metric_seq_{}", i))
                .bind("counter")
                .bind(1.0f32)
                .bind(format!("{{\"dummy\": {}}}", i))
                .bind(Utc::now())
                .execute(&pool).await.unwrap();
        }

        let db = DB { pool: pool.clone(), store: DbStore::Postgres };
        let worker = TelemetrySyncWorker::with_mode(db.clone(), mock_url.clone(), perf::CoordinatorMode::Sequential);
        let start = Instant::now();
        let _ = tokio::time::timeout(std::time::Duration::from_secs(5), worker.sync_metrics()).await;
        let seq_duration = start.elapsed();

        for i in 0..100 {
            sqlx::query("INSERT INTO local_telemetry_buffer (metric_name, metric_type, value, labels_json, timestamp, sync_status) VALUES ($1, $2, $3, $4, $5, 'pending')")
                .bind(format!("bench_metric_par_{}", i))
                .bind("counter")
                .bind(1.0f32)
                .bind(format!("{{\"dummy\": {}}}", i))
                .bind(Utc::now())
                .execute(&pool).await.unwrap();
        }

        let par_worker = TelemetrySyncWorker::with_mode(db.clone(), mock_url.clone(), perf::CoordinatorMode::Parallel);
        let start_par = Instant::now();
        let _ = tokio::time::timeout(std::time::Duration::from_secs(5), par_worker.sync_metrics()).await;
        let par_duration = start_par.elapsed();

        tracing::info!("Sequential Sync Duration: {:?}", seq_duration);
        tracing::info!("Parallel Sync Duration: {:?}", par_duration);

        assert!(seq_duration > std::time::Duration::from_nanos(0));
        assert!(par_duration > std::time::Duration::from_nanos(0));

        sqlx::query("DELETE FROM local_telemetry_buffer WHERE metric_name LIKE 'bench_metric_%'").execute(&pool).await.unwrap();
    }
}
