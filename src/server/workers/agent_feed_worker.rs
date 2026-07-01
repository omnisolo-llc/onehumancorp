use std::sync::Arc;
use tokio::time::Duration;
use crate::db::DB;
use sqlx::Row;

pub struct AgentFeedWorker {
    db: Arc<DB>,
}

impl AgentFeedWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub fn start(self: Arc<Self>) {
        tokio::spawn(async move {
            tracing::info!("Starting AgentFeedWorker for Agent Feed Ingestion...");
            loop {
                match self.poll().await {
                    Ok(true) => {
                        // Processed a job, continue
                        continue;
                    }
                    Ok(false) => {
                        // No jobs, sleep
                        tokio::time::sleep(Duration::from_millis(2000)).await;
                    }
                    Err(e) => {
                        tracing::error!("AgentFeedWorker error: {}", e);
                        tokio::time::sleep(Duration::from_millis(5000)).await;
                    }
                }
            }
        });
    }

    async fn poll(&self) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        let pool = self.db.pool.clone();
        let mut tx = pool.begin().await?;

        let task: Option<(String, String, String)> = match &self.db.store {
            crate::db::DbStore::Postgres => {
                sqlx::query_as(
                    "SELECT id, tenant_id, payload::text FROM ohc_job_queue
                    WHERE status = 'PENDING' AND next_retry_at <= NOW() AND job_type = 'agent_feed_ingest'
                    ORDER BY created_at ASC
                    LIMIT 1 FOR UPDATE SKIP LOCKED"
                )
                .fetch_optional(&mut *tx)
                .await?
            }
            crate::db::DbStore::Sqlite(_) => {
                let row = sqlx::query(
                    "SELECT id, tenant_id, payload FROM ohc_job_queue
                    WHERE status = 'PENDING' AND next_retry_at <= CURRENT_TIMESTAMP AND job_type = 'agent_feed_ingest'
                    ORDER BY created_at ASC
                    LIMIT 1"
                )
                .fetch_optional(&mut *tx)
                .await?;

                if let Some(r) = row {
                    let id: String = r.get("id");
                    let tenant_id: String = r.get("tenant_id");
                    let payload: String = r.get("payload");
                    Some((id, tenant_id, payload))
                } else {
                    None
                }
            }
        };

        let (job_id, tenant_id, payload_str) = match task {
            Some(t) => t,
            None => {
                tx.rollback().await?;
                return Ok(false);
            }
        };

        // Mark as processing
        let _ = match &self.db.store {
            crate::db::DbStore::Postgres => {
                sqlx::query("UPDATE ohc_job_queue SET status = 'PROCESSING', updated_at = NOW() WHERE id = $1")
                    .bind(&job_id)
                    .execute(&mut *tx)
                    .await?
            }
            crate::db::DbStore::Sqlite(_) => {
                sqlx::query("UPDATE ohc_job_queue SET status = 'PROCESSING', updated_at = CURRENT_TIMESTAMP WHERE id = ?")
                    .bind(&job_id)
                    .execute(&mut *tx)
                    .await?
            }
        };

        tx.commit().await?;

        tracing::info!("AgentFeedWorker processing job {}", job_id);

        let parsed: serde_json::Value = serde_json::from_str(&payload_str).unwrap_or(serde_json::json!({}));
        let event_source = parsed.get("event_source").and_then(|v| v.as_str()).unwrap_or("unknown");
        let empty_payload = serde_json::json!({});
        let payload = parsed.get("payload").unwrap_or(&empty_payload);

        let service = crate::services::agent_feed::service::AgentFeedService::new(Arc::new(pool.clone()));
        match service.process_event(&tenant_id, event_source, payload).await {
            Ok(_) => {
                let _ = match &self.db.store {
                    crate::db::DbStore::Postgres => {
                        sqlx::query("UPDATE ohc_job_queue SET status = 'COMPLETED', updated_at = NOW() WHERE id = $1")
                            .bind(&job_id)
                            .execute(&pool)
                            .await
                    }
                    crate::db::DbStore::Sqlite(_) => {
                        sqlx::query("UPDATE ohc_job_queue SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = ?")
                            .bind(&job_id)
                            .execute(&pool)
                            .await
                    }
                };
            }
            Err(e) => {
                tracing::error!("AgentFeedWorker failed to process event: {}", e);
                let _ = match &self.db.store {
                    crate::db::DbStore::Postgres => {
                        sqlx::query("UPDATE ohc_job_queue SET status = 'FAILED', next_retry_at = NOW() + INTERVAL '5 minutes', updated_at = NOW() WHERE id = $1")
                            .bind(&job_id)
                            .execute(&pool)
                            .await
                    }
                    crate::db::DbStore::Sqlite(_) => {
                        sqlx::query("UPDATE ohc_job_queue SET status = 'FAILED', next_retry_at = datetime('now', '+5 minutes'), updated_at = CURRENT_TIMESTAMP WHERE id = ?")
                            .bind(&job_id)
                            .execute(&pool)
                            .await
                    }
                };
            }
        }

        Ok(true)
    }
}
