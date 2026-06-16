use tracing::{info, warn};
use std::time::Duration;
use sqlx::{SqlitePool, PgPool, Row};
use chrono::Utc;

pub struct McpSyncWorker {
    sqlite_pool: SqlitePool,
    pg_pool: PgPool,
}

impl McpSyncWorker {
    pub fn new(sqlite_pool: SqlitePool, pg_pool: PgPool) -> Self {
        Self {
            sqlite_pool,
            pg_pool,
        }
    }

    pub async fn run(&self) {
        info!("Starting McpSyncWorker...");
        loop {
            if let Err(e) = self.sync_metrics().await {
                warn!("McpSyncWorker error: {}", e);
            }
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    }

    pub async fn sync_metrics(&self) -> Result<(), Box<dyn std::error::Error>> {
        let rows = sqlx::query("SELECT id, metric_name, metric_type, value, labels_json, timestamp FROM telemetry_buffer WHERE sync_status = 'pending'")
            .fetch_all(&self.sqlite_pool)
            .await?;

        if rows.is_empty() {
            return Ok(());
        }

        info!("Simulating MCP upload for {} pending metrics...", rows.len());

        let mut tx = self.pg_pool.begin().await?;

        for row in &rows {
            let metric_name: String = row.get("metric_name");
            let metric_type: String = row.get("metric_type");
            let value: f32 = row.get("value");
            let labels_json: String = row.get("labels_json");
            let timestamp: chrono::NaiveDateTime = row.get("timestamp");

            let res = sqlx::query("INSERT INTO telemetry_buffer (metric_name, metric_type, value, labels_json, timestamp, sync_status) VALUES ($1, $2, $3, $4, $5, 'synced')")
                .bind(metric_name)
                .bind(metric_type)
                .bind(value)
                .bind(labels_json)
                .bind(chrono::DateTime::<Utc>::from_naive_utc_and_offset(timestamp, Utc))
                .execute(&mut *tx)
                .await;

            if res.is_err() {
                tx.rollback().await?;
                return Err("Failed to insert telemetry to pg".into());
            }
        }

        tx.commit().await?;

        for row in rows {
            let id: i64 = row.get("id");
            sqlx::query("UPDATE telemetry_buffer SET sync_status = 'synced' WHERE id = ?")
                .bind(id)
                .execute(&self.sqlite_pool)
                .await?;
        }

        Ok(())
    }
}
