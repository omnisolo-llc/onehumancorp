use std::sync::Arc;
use tokio::time::{sleep, Duration};
use crate::rag_sync::RAGSyncService;
use crate::orchestration::locks::DistributedLock;

pub struct RagSyncWorker {
    rag_service: Arc<dyn RAGSyncService>,
    distributed_lock: Arc<dyn DistributedLock>,
    tenant_id: String,
}

impl RagSyncWorker {
    pub fn new(rag_service: Arc<dyn RAGSyncService>, distributed_lock: Arc<dyn DistributedLock>, tenant_id: String) -> Self {
        Self {
            rag_service,
            distributed_lock,
            tenant_id,
        }
    }

    pub fn start(self: Arc<Self>) {
        tokio::spawn(async move {
            self.run_loop().await;
        });
    }

    async fn run_loop(&self) {
        loop {
            if let Err(e) = self.process_pending_syncs().await {
                tracing::error!("RagSyncWorker error: {}", e);
            }
            sleep(Duration::from_secs(5)).await;
        }
    }

    pub async fn process_pending_syncs(&self) -> Result<(), String> {
        let pending = self.rag_service.fetch_pending_syncs(&self.tenant_id, 10).await?;

        for record in pending {
            let lock_key = format!("ohc:lock:{}:rag_sync:{}", self.tenant_id, record.id);

            // Acquire distributed lock for multi-tenant isolation
            match self.distributed_lock.acquire_resource(&self.tenant_id, "rag_sync", &record.id).await {
                Ok(_guard) => {
                    // Process sync (e.g., embedding and storing vector)
                    // ... (Mock processing for now)

                    // Mark as synced
                    self.rag_service.mark_synced(&self.tenant_id, vec![record.id]).await?;
                }
                Err(e) => {
                    tracing::warn!("Failed to acquire lock for rag sync {}: {}", lock_key, e); // pii-safe
                }
            }
        }

        Ok(())
    }
}
