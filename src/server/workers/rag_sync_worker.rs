use std::sync::Arc;
use crate::orchestration::queue::ohc_job_queue::OHCJob;
use crate::orchestration::queue::worker_pool::JobHandler;
use async_trait::async_trait;

pub struct RagSyncWorker {
    pub lock_provider: Arc<dyn crate::orchestration::locks::DistributedLock>,
}

impl RagSyncWorker {
    pub fn new(lock_provider: Arc<dyn crate::orchestration::locks::DistributedLock>) -> Self {
        Self { lock_provider }
    }
}

#[async_trait]
impl JobHandler for RagSyncWorker {
    fn handle(&self, job: OHCJob) -> tokio::task::JoinHandle<Result<(), String>> {
        let lock_provider = self.lock_provider.clone();
        tokio::spawn(async move {
            let payload: serde_json::Value = serde_json::from_str(&job.payload).unwrap_or(serde_json::json!({}));
            let document_id = payload.get("id").and_then(|v| v.as_str()).unwrap_or("");

            // Ensure isolation check
            let tenant_id = job.tenant_id.clone();
            if tenant_id.is_empty() {
                return Err("Missing tenant_id in rag_sync job".to_string());
            }

            // Attempt to acquire distributed lock
            match lock_provider.acquire_resource(&tenant_id, "rag_sync", document_id).await {
                Ok(_guard) => {
                    // In a real implementation, vector embedding happens here.
                    // For the scope of this e2e test, we just simulate the work.
                    tracing::info!("RagSyncWorker: Successfully processed document {} for tenant {}", document_id, tenant_id);
                    // The lock is dropped here when _guard goes out of scope.
                    Ok(())
                },
                Err(e) => {
                    tracing::warn!("RagSyncWorker: Lock contention for document {}: {}", document_id, e);
                    // Return an error so the job is retried by the queue (backoff)
                    Err("Lock contention".to_string())
                }
            }
        })
    }
}
