use std::sync::Arc;
use crate::db::DB;
use std::time::Duration;

use sqlx::Row;
use serde_json::json;
use tokio::time::timeout;
use crate::orchestration::departments::types::{DepartmentType, ActionRisk};
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;

const DB_OP_TIMEOUT: Duration = Duration::from_secs(2);

pub struct ReviewCampaignsWorker {
    pub db: Arc<DB>,
    pub poll_interval: Duration,
    pub orchestrator: Arc<DepartmentOrchestrator>,
}

impl ReviewCampaignsWorker {
    pub fn new(db: Arc<DB>, orchestrator: Arc<DepartmentOrchestrator>) -> Self {
        Self {
            db,
            poll_interval: Duration::from_secs(10), // Poll every 10s
            orchestrator,
        }
    }

    pub fn start(&self) {
        let db = self.db.clone();
        let interval_duration = self.poll_interval;
        let orchestrator = self.orchestrator.clone();

        tokio::spawn(async move {
            let pool = db.pool.clone();
            loop {
                tokio::time::sleep(interval_duration).await;

                let poll_op = async {
                    match &db.store {
                        crate::db::DbStore::Postgres => {
                            let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
                            let row = sqlx::query(
                                r#"
                                SELECT id, tenant_id, payload FROM ohc_job_queue
                                WHERE status = 'PENDING' AND job_type = 'send_review_request'
                                AND next_retry_at <= CURRENT_TIMESTAMP
                                ORDER BY created_at ASC
                                LIMIT 1 FOR UPDATE SKIP LOCKED
                                "#
                            )
                            .fetch_optional(&mut *tx)
                            .await
                            .map_err(|e| e.to_string())?;

                            if let Some(r) = row {
                                let id: String = r.get("id");
                                let tenant_id: String = r.get("tenant_id");
                                let payload: serde_json::Value = r.try_get("payload").unwrap_or(json!({}));

                                sqlx::query("UPDATE ohc_job_queue SET status = 'PROCESSING', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
                                    .bind(&id)
                                    .execute(&mut *tx).await.map_err(|e| e.to_string())?;

                                tx.commit().await.map_err(|e| e.to_string())?;
                                Ok::<_, String>(Some((id, tenant_id, payload)))
                            } else {
                                tx.rollback().await.map_err(|e| e.to_string())?;
                                Ok::<_, String>(None)
                            }
                        },
                        crate::db::DbStore::Sqlite(sqlite_pool) => {
                             let mut tx = sqlite_pool.begin().await.map_err(|e| e.to_string())?;
                            let row = sqlx::query(
                                r#"
                                SELECT id, tenant_id, payload FROM ohc_job_queue
                                WHERE status = 'PENDING' AND job_type = 'send_review_request'
                                AND next_retry_at <= CURRENT_TIMESTAMP
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
                                let payload_str: String = r.get("payload");
                                let payload: serde_json::Value = serde_json::from_str(&payload_str).unwrap_or(json!({}));

                                sqlx::query("UPDATE ohc_job_queue SET status = 'PROCESSING', updated_at = CURRENT_TIMESTAMP WHERE id = ?")
                                    .bind(&id)
                                    .execute(&mut *tx).await.map_err(|e| e.to_string())?;

                                tx.commit().await.map_err(|e| e.to_string())?;
                                Ok::<_, String>(Some((id, tenant_id, payload)))
                            } else {
                                tx.rollback().await.map_err(|e| e.to_string())?;
                                Ok::<_, String>(None)
                            }
                        }
                    }
                };

                let task = match timeout(DB_OP_TIMEOUT, poll_op).await {
                    Ok(Ok(Some(res))) => res,
                    Ok(Ok(None)) => continue,
                    _ => continue,
                };

                let (job_id, tenant_id, payload) = task;
                let booking_id = payload.get("booking_id").and_then(|c| c.as_str()).unwrap_or("");

                // Construct SMS asking for review
                let customer_id = payload.get("customer_id").and_then(|v| v.as_str()).unwrap_or("");
                let review_link = format!("https://ohc.store/review/{}", booking_id); // Using booking_id instead of order_id here

                let sms_body = format!("Hi there! We hope you loved our service today. We'd really appreciate it if you could leave us a quick review: {}", review_link);

                tracing::info!("ReviewCampaignWorker: Sending review request to customer {} for booking {} (tenant {})", customer_id, booking_id, tenant_id);

                // Let's create an action in the orchestrator
                let action_payload = serde_json::json!({
                    "feature_type": "review_campaign",
                    "booking_id": booking_id,
                    "customer_id": customer_id,
                    "sms_body": sms_body,
                });

                let _ = orchestrator.execute_action(
                    DepartmentType::CustomerSuccess,
                    format!("Send review request to customer for booking {}", booking_id),
                    tenant_id.clone(),
                    ActionRisk::AutoExecute, // Automatically sent per issue description
                    action_payload,
                ).await;

                // Update review campaign status
                match &db.store {
                    crate::db::DbStore::Postgres => {
                        let _ = sqlx::query("UPDATE review_campaigns SET status = 'sent', updated_at = CURRENT_TIMESTAMP WHERE tenant_id = $1 AND booking_id = $2")
                            .bind(&tenant_id)
                            .bind(booking_id)
                            .execute(&db.pool)
                            .await;

                        let _ = sqlx::query("UPDATE ohc_job_queue SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
                            .bind(&job_id)
                            .execute(&db.pool).await;
                    },
                    crate::db::DbStore::Sqlite(sqlite_pool) => {
                        let _ = sqlx::query("UPDATE review_campaigns SET status = 'sent', updated_at = CURRENT_TIMESTAMP WHERE tenant_id = ? AND booking_id = ?")
                            .bind(&tenant_id)
                            .bind(booking_id)
                            .execute(sqlite_pool)
                            .await;

                        let _ = sqlx::query("UPDATE ohc_job_queue SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = ?")
                            .bind(&job_id)
                            .execute(sqlite_pool).await;
                    }
                }
            }
        });
    }
}
