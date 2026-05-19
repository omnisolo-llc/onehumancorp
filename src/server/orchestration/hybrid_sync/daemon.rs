use std::time::Duration;
use sqlx::{SqlitePool, PgPool, Row};
use serde_json::{Value, json};
use tracing::{info, error, warn};
use uuid::Uuid;
use chrono::Utc;

pub struct HybridSyncDaemon {
    sqlite_pool: SqlitePool,
    pg_pool: PgPool,
}

impl HybridSyncDaemon {
    pub fn new(sqlite_pool: SqlitePool, pg_pool: PgPool) -> Self {
        Self { sqlite_pool, pg_pool }
    }

    pub async fn run(&self) {
        loop {
            if let Err(e) = self.sync_step().await {
                error!("Hybrid sync daemon error: {}", e);
            }
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    }

    pub async fn sync_step(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Find tasks requiring cloud escalation
        let rows = sqlx::query("SELECT memory_id, context FROM swarm_truth_embeddings WHERE escalation_required = 1 AND sync_status = 'PENDING'")
            .fetch_all(&self.sqlite_pool)
            .await?;

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

            let queue_id = Uuid::new_v4().to_string();
            let now = Utc::now().naive_utc();

            // Enqueue task into SubAgentQueue in PostgreSQL
            // Note: Cloud Native Postgres db usage implies this queue handles `FOR UPDATE SKIP LOCKED`
            // when picking tasks, as required by the prompt. We just need to ensure we insert it properly
            // so the worker can pick it up with `FOR UPDATE SKIP LOCKED`. We'll just insert here.
            let mut tx = match self.pg_pool.begin().await {
                Ok(t) => t,
                Err(e) => {
                    warn!("Failed to begin pg transaction: {}, gracefully degrading (cloud unreachable).", e);
                    continue;
                }
            };

            let res = sqlx::query("INSERT INTO sub_agent_queue (id, tenant_id, parent_task_id, payload, status, scheduled_at, created_at, updated_at) VALUES ($1, 'system', NULL, $2, 'QUEUED', $3, $3, $3)")
                .bind(&queue_id)
                .bind(payload.to_string())
                .bind(now)
                .execute(&mut *tx)
                .await;

            match res {
                Ok(_) => {
                    let commit_res = tx.commit().await;
                    if let Err(e) = commit_res {
                        warn!("Failed to commit pg transaction for memory_id: {}, gracefully degrading. Error: {}", id, e);
                        continue;
                    }

                    // Update SQLite sync status
                    sqlx::query("UPDATE swarm_truth_embeddings SET sync_status = 'SYNCED' WHERE memory_id = ?")
                        .bind(&id)
                        .execute(&self.sqlite_pool)
                        .await?;
                    info!("Successfully escalated memory_id: {} to cloud queue: {}", id, queue_id);
                }
                Err(e) => {
                    let _ = tx.rollback().await;
                    warn!("Failed to escalate memory_id: {}, gracefully degrading (cloud unreachable). Error: {}", id, e);
                }
            }
        }

        Ok(())
    }
}
