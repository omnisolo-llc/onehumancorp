use std::sync::Arc;
use tokio::time::{sleep, Duration};
use crate::orchestration::locks::DistributedLock;
use sqlx::{PgPool, Row};
use uuid::Uuid;

pub struct IngestDocumentWorker {
    pool: PgPool,
    distributed_lock: Arc<dyn DistributedLock>,
    tenant_id: String,
}

impl IngestDocumentWorker {
    pub fn new(pool: PgPool, distributed_lock: Arc<dyn DistributedLock>, tenant_id: String) -> Self {
        Self {
            pool,
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
            if let Err(e) = self.process_pending_documents().await {
                tracing::error!("IngestDocumentWorker error: {}", e);
            }
            sleep(Duration::from_secs(5)).await;
        }
    }

    pub async fn process_pending_documents(&self) -> Result<(), String> {
        // Fetch pending documents for this tenant
        let pending = sqlx::query!(
            "SELECT id, content FROM knowledge_documents WHERE tenant_id = $1 AND status = 'PENDING' LIMIT 10",
            self.tenant_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Failed to fetch pending documents: {}", e))?;

        for doc in pending {
            let doc_id = doc.id;
            let lock_key = format!("ohc:lock:{}:ingest_document:{}", self.tenant_id, doc_id);

            // Acquire distributed lock for multi-tenant isolation
            match self.distributed_lock.acquire_resource(&self.tenant_id, "ingest_document", &doc_id.to_string()).await {
                Ok(_guard) => {
                    // Update status to PROCESSING
                    sqlx::query!(
                        "UPDATE knowledge_documents SET status = 'PROCESSING', updated_at = NOW() WHERE id = $1 AND tenant_id = $2",
                        doc_id, self.tenant_id
                    )
                    .execute(&self.pool)
                    .await
                    .map_err(|e| format!("Failed to update status: {}", e))?;

                    let content = doc.content.unwrap_or_default();

                    // Simple chunking (e.g. by newlines or words in a real system)
                    // We'll mock the chunking and embedding generation here
                    let chunks = vec![content];

                    for (index, chunk) in chunks.iter().enumerate() {
                        // Mock embedding vector of size 1536 (pgvector default commonly used with OpenAI)
                        let mock_embedding = vec![0.0f32; 1536];
                        // Requires vector type mapping for sqlx, skipping actual bind for now in this skeleton
                    }

                    // Mark as LEARNED
                    sqlx::query!(
                        "UPDATE knowledge_documents SET status = 'LEARNED', updated_at = NOW() WHERE id = $1 AND tenant_id = $2",
                        doc_id, self.tenant_id
                    )
                    .execute(&self.pool)
                    .await
                    .map_err(|e| format!("Failed to update status to learned: {}", e))?;
                }
                Err(e) => {
                    tracing::warn!("Failed to acquire lock for document ingestion {}: {}", lock_key, e);
                }
            }
        }

        Ok(())
    }
}
