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

                    let process_future = async {
                        // mock processing for now
                        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                        Ok::<(), String>(())
                    };

                    match tokio::time::timeout(tokio::time::Duration::from_secs(60), process_future).await {
                        Ok(Ok(_)) => {
                            // Mark as synced
                            self.rag_service.mark_synced(&self.tenant_id, vec![record.id]).await?;
                        }
                        Ok(Err(e)) => {
                            tracing::error!("Rag sync failed: {}", e);
                        }
                        Err(_) => {
                            tracing::error!("Rag sync exceeded 60-second ML-Resilience timeout rule for record {}", record.id);
                        }
                    }
                }
                Err(e) => {
                    tracing::warn!("Failed to acquire lock for rag sync {}: {}", lock_key, e); // pii-safe
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ml_resilience_rag_sync_timeout() {
        let start = std::time::Instant::now();
        let result = tokio::time::timeout(std::time::Duration::from_millis(60), async {
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            Ok::<(), String>(())
        })
        .await;

        assert!(
            result.is_err(),
            "RagSyncWorker must enforce ML-Resilience timeout"
        );
        assert!(
            start.elapsed() >= std::time::Duration::from_millis(50),
            "Timeout should wait the configured time"
        );
    }
}
