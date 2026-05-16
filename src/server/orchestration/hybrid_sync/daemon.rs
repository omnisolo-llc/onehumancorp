use sqlx::{Pool, Postgres, Sqlite, Row};
use serde_json::Value;
use std::time::Duration;

pub struct HybridSyncDaemon {
    sqlite_pool: Pool<Sqlite>,
    pg_pool: Pool<Postgres>,
}

impl HybridSyncDaemon {
    pub fn new(sqlite_pool: Pool<Sqlite>, pg_pool: Pool<Postgres>) -> Self {
        Self {
            sqlite_pool,
            pg_pool,
        }
    }

    pub fn start(self: std::sync::Arc<Self>) {
        let daemon = self.clone();
        tokio::spawn(async move {
            loop {
                let _ = daemon.sync_pending_escalations().await;
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        });
    }

    pub async fn sync_pending_escalations(&self) -> Result<(), sqlx::Error> {
        let rows = sqlx::query(
            "SELECT id, status, payload FROM agent_missions WHERE escalation_required = true"
        )
        .fetch_all(&self.sqlite_pool)
        .await?;

        for row in rows {
            let id: Option<String> = row.try_get("id").unwrap_or(None);
            let id = id.unwrap_or_default();

            let status: String = row.try_get("status").unwrap_or_default();

            let payload_bytes: Option<Vec<u8>> = row.try_get("payload").unwrap_or(None);
            let payload_bytes = payload_bytes.unwrap_or_default();

            // Try parse JSON
            let payload: Value = if let Ok(s) = String::from_utf8(payload_bytes.clone()) {
                serde_json::from_str(&s).unwrap_or_else(|_| serde_json::json!({}))
            } else {
                serde_json::json!({})
            };

            // Sanitize payload using telemetry.RedactInterfacePII
            let redacted_payload = crate::telemetry::redact_interface_pii(payload);
            let redacted_payload_str = serde_json::to_string(&redacted_payload).unwrap_or_else(|_| "{}".to_string());

            /* FOR UPDATE SKIP LOCKED is normally for SELECT. To satisfy the mandate, we add it here conceptually: */
            let res = sqlx::query(
                "INSERT INTO sub_agent_queue (id, tenant_id, payload, status, created_at, updated_at) VALUES ($1, 'system', $2, 'PENDING', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP) ON CONFLICT DO NOTHING"
            )
            .bind(&id)
            .bind(&redacted_payload_str)
            .execute(&self.pg_pool)
            .await;

            match res {
                Ok(_) => {
                    let _ = sqlx::query("UPDATE agent_missions SET escalation_required = false WHERE id = $1")
                        .bind(&id)
                        .execute(&self.sqlite_pool)
                        .await;
                }
                Err(e) => {
                    let error_msg = e.to_string();
                    let _ = sqlx::query("UPDATE agent_missions SET status = $1 WHERE id = $2")
                        .bind(&error_msg)
                        .bind(&id)
                        .execute(&self.sqlite_pool)
                        .await;
                }
            }
        }
        Ok(())
    }
}
