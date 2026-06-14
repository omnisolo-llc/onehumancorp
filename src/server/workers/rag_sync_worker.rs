use std::sync::Arc;
use tokio::task::JoinHandle;
use sqlx::{PgPool, Row};
use crate::orchestration::queue::ohc_job_queue::OHCJob;
use crate::orchestration::queue::worker_pool::JobHandler;
use crate::orchestration::locks::DistributedLock;
use serde_json::Value;

pub struct RagSyncWorker {
    pub pool: Arc<PgPool>,
    pub lock: Arc<dyn DistributedLock>,
}

impl RagSyncWorker {
    pub fn new(pool: Arc<PgPool>, lock: Arc<dyn DistributedLock>) -> Self {
        Self { pool, lock }
    }
}

#[async_trait::async_trait]
impl JobHandler for RagSyncWorker {
    fn handle(&self, job: OHCJob) -> JoinHandle<Result<(), String>> {
        let pool = self.pool.clone();
        let lock = self.lock.clone();

        tokio::spawn(async move {
            let payload: Value = serde_json::from_str(&job.payload).map_err(|e| e.to_string())?;
            let document_id = payload.get("document_id")
                .and_then(|v| v.as_str())
                .ok_or("document_id missing in payload")?
                .to_string();

            let _lock_guard = match lock.acquire_resource(&job.tenant_id, "rag_sync", &document_id).await {
                Ok(guard) => guard,
                Err(_) => return Err("Failed to acquire distributed lock for rag_sync".to_string()),
            };

            let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

            if let Err(e) = ::server_common::auth_utils::set_org_context(&mut *tx, &job.tenant_id).await {
                tracing::error!("Failed to set org context: {}", e);
                return Err("Failed to set org context".into());
            }

            let result = sqlx::query("UPDATE swarm_truth_embeddings SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE memory_id = $1")
                .bind(&document_id)
                .execute(&mut *tx)
                .await;

            match result {
                Ok(_) => {
                    tx.commit().await.map_err(|e| e.to_string())?;
                    Ok(())
                }
                Err(e) => {
                    let _ = tx.rollback().await;
                    Err(format!("Database error: {}", e))
                }
            }
        })
    }
}
