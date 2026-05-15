use std::sync::Arc;
use sqlx::{Row, sqlite::SqlitePool};
use crate::db::DB;
use crate::telemetry::redact_interface_pii;
use crate::orchestration::queue::{TaskQueue, Job};
use chrono::Utc;
use serde_json::Value;

pub struct OmniContextSyncDaemon<Q: TaskQueue> {
    db: Arc<DB>,
    cloud_queue: Arc<Q>,
}

impl<Q: TaskQueue> OmniContextSyncDaemon<Q> {
    pub fn new(db: Arc<DB>, cloud_queue: Arc<Q>) -> Self {
        Self { db, cloud_queue }
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.db.is_sqlite() {
            return Ok(());
        }

        let sqlite_pool = match &self.db.store {
            crate::db::DbStore::Sqlite(pool) => pool,
            _ => return Ok(()),
        };

        // 1. Fetch from SQLite
        let rows = sqlx::query(
            "SELECT id, payload, organization_id, status FROM agent_missions WHERE synced_to_cloud = false AND (status = 'CLOUD_ESCALATION' OR status = 'BURSTING') AND (sync_error IS NULL OR last_synced_at < datetime('now', '-5 minutes')) LIMIT 100"
        )
        .fetch_all(sqlite_pool)
        .await?;

        for row in rows {
            let mission_id: String = row.try_get("id")?;
            let payload_str: String = row.try_get("payload").unwrap_or_else(|_| "{}".to_string());
            let org_id: String = row.try_get("organization_id").unwrap_or_else(|_| "".to_string());

            // 2. Sanitize
            let payload: Value = serde_json::from_str(&payload_str).unwrap_or_else(|_| serde_json::json!({}));
            let sanitized_payload = redact_interface_pii(payload);
            let sanitized_payload_str = serde_json::to_string(&sanitized_payload)?;

            // 3. Enqueue to Cloud
            let job = Job {
                id: uuid::Uuid::new_v4().to_string(),
                tenant_id: org_id.clone(),
                parent_task_id: mission_id.clone(),
                agent_role: "cloud_escalation".to_string(),
                payload: sanitized_payload_str,
                status: "QUEUED".to_string(),
                attempts: 0,
                max_attempts: 3,
                run_after: Utc::now(),
                locked_until: None,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };

            match self.cloud_queue.enqueue(job).await {
                Ok(_) => {
                    sqlx::query("UPDATE agent_missions SET synced_to_cloud = true WHERE id = ?")
                        .bind(&mission_id)
                        .execute(sqlite_pool)
                        .await?;
                }
                Err(e) => {
                    tracing::error!("Failed to enqueue cloud escalation task for mission {}: {}", mission_id, e);
                    sqlx::query("UPDATE agent_missions SET sync_error = ?, last_synced_at = CURRENT_TIMESTAMP WHERE id = ?")
                        .bind(&e.to_string())
                        .bind(&mission_id)
                        .execute(sqlite_pool)
                        .await?;
                }
            }
        }

        Ok(())
    }
}
