use std::time::Duration;
use sqlx::{SqlitePool, PgPool, Row};
use serde_json::{Value, json};
use tracing::{info, error, warn};
use uuid::Uuid;
use chrono::Utc;

pub struct HybridSyncDaemon {
    sqlite_pool: SqlitePool,
    pg_pool: PgPool,
    cloud_url: String,
    client: reqwest::Client,
}

impl HybridSyncDaemon {
    pub fn new(sqlite_pool: SqlitePool, pg_pool: PgPool, cloud_url: String) -> Self {
        Self {
            sqlite_pool,
            pg_pool,
            cloud_url,
            client: reqwest::Client::new(),
        }
    }

    pub async fn run(&self) {
        loop {
            if let Err(e) = self.sync_step().await {
                error!("Hybrid sync daemon error: {}", e);
            }
            if let Err(e) = self.sync_telemetry_step().await {
                error!("Hybrid sync telemetry error: {}", e);
            }
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    }

    pub async fn sync_telemetry_step(&self) -> Result<(), Box<dyn std::error::Error>> {
        if !::server_config::get().telemetry_enabled {
            return Ok(());
        }

        let rows = sqlx::query("SELECT id, metric_name, metric_type, value, labels_json, timestamp FROM telemetry_buffer WHERE sync_status = 'pending'")
            .fetch_all(&self.sqlite_pool)
            .await?;

        if rows.is_empty() {
            return Ok(());
        }

        let mut tx = match self.pg_pool.begin().await {
            Ok(t) => t,
            Err(e) => {
                warn!("Failed to begin pg transaction for telemetry sync: {}", e);
                return Ok(());
            }
        };

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

            if let Err(e) = res {
                let _ = tx.rollback().await;
                warn!("Failed to insert telemetry to pg: {}", e);
                return Ok(());
            }
        }

        if let Err(e) = tx.commit().await {
            warn!("Failed to commit telemetry to pg: {}", e);
            return Ok(());
        }

        for row in rows {
            let id: i32 = row.get("id");
            let _ = sqlx::query("UPDATE telemetry_buffer SET sync_status = 'SYNCED' WHERE id = ?")
                .bind(id)
                .execute(&self.sqlite_pool)
                .await;
        }

        info!("Successfully synced telemetry batch");

        Ok(())
    }

    pub async fn sync_step(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Find tasks requiring cloud escalation
        let rows = sqlx::query("SELECT memory_id, context FROM swarm_truth_embeddings WHERE escalation_required = 1 AND sync_status = 'PENDING'")
            .fetch_all(&self.sqlite_pool)
            .await?;

        let mut success_count = 0;

        for row in rows {
            let id: String = row.get("memory_id");
            let context: String = row.get("context");

            // Sanitize PII
            let parsed: Value = serde_json::from_str(&context).unwrap_or(json!({ "raw": context }));
            let sanitized = ::server_telemetry::redact_interface_pii(parsed);

            let payload = json!({
                "source": "hybrid_sync",
                "memory_id": id,
                "context": sanitized
            });

            let url = format!("{}/api/sync/missions", self.cloud_url);

            let req_payload = json!({
                "missions": [{
                    "memory_id": id,
                    "payload": payload
                }]
            });

            let system_token = std::env::var("OHC_SYSTEM_TOKEN").unwrap_or_else(|_| "missing_token".to_string());
            match self.client.post(&url)
                .header("Authorization", format!("Bearer {}", system_token))
                .json(&req_payload)
                .send()
                .await
            {
                Ok(resp) if resp.status().is_success() => {
                    // Update SQLite sync status
                    if let Err(e) = sqlx::query("UPDATE swarm_truth_embeddings SET sync_status = 'SYNCED' WHERE memory_id = ?")
                        .bind(&id)
                        .execute(&self.sqlite_pool)
                        .await
                    {
                        warn!("Failed to update SQLite sync status for memory_id: {}. Error: {}", id, e);
                        continue;
                    }

                    info!("Successfully escalated memory_id: {} via cloud gateway", id);
                    success_count += 1;

                    if let Err(e) = ::server_telemetry::record_rag_escalation(&self.pg_pool, "system", "").await {
                        warn!("Failed to record RAG escalation telemetry: {}", e);
                    }
                }
                Ok(resp) => {
                    warn!("Cloud gateway returned error status {} for memory_id: {}, gracefully degrading.", resp.status(), id);
                }
                Err(e) => {
                    warn!("Failed to send HTTP request to escalate memory_id: {}, gracefully degrading. Error: {}", id, e);
                }
            }
        }

        if success_count > 0 {
            if let Err(e) = ::server_telemetry::record_sync_escalation(&self.pg_pool, success_count as f32, ::server_telemetry::get_deployment_mode()).await {
                warn!("Failed to record sync escalation telemetry: {}", e);
            }
        }

        Ok(())
    }
}
