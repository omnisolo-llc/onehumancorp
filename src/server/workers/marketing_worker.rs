use std::sync::Arc;
use crate::db::DB;
use std::time::Duration;
use uuid::Uuid;
use sqlx::Row;
use tracing::{info, error};

pub struct MarketingWorker {
    pub db: Arc<DB>,
    pub poll_interval: Duration,
}

impl MarketingWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self {
            db,
            poll_interval: Duration::from_secs(5),
        }
    }

    pub fn start(&self) {
        let db = self.db.clone();
        let interval_duration = self.poll_interval;
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(interval_duration);
            loop {
                interval.tick().await;
                loop {
                    match Self::poll(&db).await {
                        Ok(true) => continue, // keep polling if we found a task
                        Ok(false) => break,   // wait for next tick
                        Err(e) => {
                            error!("Error polling in MarketingWorker: {}", e);
                            break;
                        }
                    }
                }
            }
        });
    }

    pub async fn poll(db: &Arc<DB>) -> Result<bool, String> {
        let poll_op = async {
            let task = match &db.store {
                crate::db::DbStore::Postgres => {
                    let mut tx = db.pool.begin().await.map_err(|e| e.to_string())?;
                    let row = sqlx::query(
                        r#"
                        UPDATE agent_approvals
                        SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP
                        WHERE id = (
                            SELECT id FROM agent_approvals
                            WHERE department = 'Marketing' AND status = 'APPROVED' AND description = 'Trigger Agentic SEO Pre-rendering'
                            ORDER BY created_at ASC
                            LIMIT 1
                            FOR UPDATE SKIP LOCKED
                        )
                        RETURNING id, tenant_id, payload
                        "#
                    )
                    .fetch_optional(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;

                    let res = row.map(|r| (r.get::<String, _>("id"), r.get::<String, _>("tenant_id"), r.get::<serde_json::Value, _>("payload")));
                    tx.commit().await.map_err(|e| e.to_string())?;
                    res
                },
                crate::db::DbStore::Sqlite(sqlite_pool) => {
                    let mut tx = sqlite_pool.begin().await.map_err(|e| e.to_string())?;
                    let row = sqlx::query(
                        r#"
                        SELECT id, tenant_id, payload FROM agent_approvals
                        WHERE department = 'Marketing' AND status = 'APPROVED' AND description = 'Trigger Agentic SEO Pre-rendering'
                        ORDER BY created_at ASC
                        LIMIT 1
                        "#
                    )
                    .fetch_optional(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;

                    if let Some(r) = row {
                        let id: String = r.get("id");
                        let tenant_id: String = r.get("tenant_id");
                        // Note: sqlite payload might be stored as string, but we read as json string and convert if needed, or directly json
                        let payload_str: String = r.get("payload");
                        let payload: serde_json::Value = serde_json::from_str(&payload_str).unwrap_or_else(|_| serde_json::json!({}));

                        sqlx::query("UPDATE agent_approvals SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = ?")
                            .bind(&id)
                            .execute(&mut *tx)
                            .await
                            .map_err(|e| e.to_string())?;

                        tx.commit().await.map_err(|e| e.to_string())?;
                        Some((id, tenant_id, payload))
                    } else {
                        tx.commit().await.map_err(|e| e.to_string())?;
                        None
                    }
                }
            };

            if let Some((_task_id, tenant_id, payload)) = task {
                let tenant_id_uuid = Uuid::parse_str(&tenant_id).unwrap_or_default();
                if let Some(site_id_str) = payload.get("site_id").and_then(|v| v.as_str()) {
                    let site_id_uuid = Uuid::parse_str(site_id_str).unwrap_or_default();
                    info!("Agentic SEO Pre-rendering triggered for tenant {} site {}", tenant_id, site_id_str);
                    let pool_clone = db.pool.clone();
                    tokio::spawn(async move {
                        let _ = crate::builder::jobs::enqueue_publish_site_job(&pool_clone, tenant_id_uuid, site_id_uuid).await;
                    });
                }
                return Ok(true);
            }

            Ok(false)
        };

        match tokio::time::timeout(std::time::Duration::from_secs(10), poll_op).await {
            Ok(res) => res,
            Err(_) => {
                error!("MarketingWorker poll operation timed out");
                Ok(false)
            }
        }
    }
}
