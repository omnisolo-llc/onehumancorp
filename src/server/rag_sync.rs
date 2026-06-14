use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SyncStatus {
    Pending,
    Synced,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RAGSyncRecord {
    pub id: String,
    pub tenant_id: String,
    pub context: String,
    pub vector: Vec<f32>,
    pub sync_status: SyncStatus,
    pub last_sync_at: Option<DateTime<Utc>>,
}

#[async_trait::async_trait]
pub trait RAGSyncService: Send + Sync {
    /// FetchPendingSyncs retrieves records from the local DB that need syncing
    async fn fetch_pending_syncs(&self, limit: i32) -> Result<Vec<RAGSyncRecord>, String>;

    /// MarkSynced updates the local DB after a successful sync to the cloud
    async fn mark_synced(&self, ids: Vec<String>) -> Result<(), String>;

    /// ProcessIncomingSync handles data pushed from a standalone client into the cloud DB
    async fn process_incoming_sync(&self, records: Vec<RAGSyncRecord>) -> Result<(), String>;
}

use crate::orchestration::locks::DistributedLock;

pub struct RagSyncWorker {
    pub service: std::sync::Arc<dyn RAGSyncService>,
    pub redis_lock: std::sync::Arc<crate::orchestration::locks::RedisLock>,
}

impl RagSyncWorker {
    pub async fn sync_document(&self, tenant_id: &str, context_tenant_id: &str, document_id: &str, records: Vec<RAGSyncRecord>) -> Result<(), String> {
        if tenant_id != context_tenant_id {
            return Err("Tenant isolation violation".to_string());
        }

        let _lock_guard = self.redis_lock.acquire_resource(tenant_id, "rag_sync", document_id).await?;

        let res = self.service.process_incoming_sync(records).await;
        // LockGuard automatically releases the lock when dropped
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use chrono::Utc;

    struct MockRAGSyncService {
        records: Arc<Mutex<Vec<RAGSyncRecord>>>,
    }

    #[async_trait::async_trait]
    impl RAGSyncService for MockRAGSyncService {
        async fn fetch_pending_syncs(&self, limit: i32) -> Result<Vec<RAGSyncRecord>, String> {
            let records = self.records.lock().await;
            let pending: Vec<RAGSyncRecord> = records
                .iter()
                .filter(|r| r.sync_status == SyncStatus::Pending)
                .take(limit as usize)
                .cloned()
                .collect();
            Ok(pending)
        }

        async fn mark_synced(&self, ids: Vec<String>) -> Result<(), String> {
            let mut records = self.records.lock().await;
            for record in records.iter_mut() {
                if ids.contains(&record.id) {
                    record.sync_status = SyncStatus::Synced;
                    record.last_sync_at = Some(Utc::now());
                }
            }
            Ok(())
        }

        async fn process_incoming_sync(&self, incoming_records: Vec<RAGSyncRecord>) -> Result<(), String> {
            let mut records = self.records.lock().await;
            for mut incoming in incoming_records {
                if let Some(existing) = records.iter_mut().find(|r| r.id == incoming.id) {
                    *existing = incoming;
                } else {
                    incoming.sync_status = SyncStatus::Synced; // Assuming they are synced once they reach cloud
                    records.push(incoming);
                }
            }
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_rag_sync_flow() {
        let mock_service = MockRAGSyncService {
            records: Arc::new(Mutex::new(vec![
                RAGSyncRecord {
                    id: "1".to_string(),
                    tenant_id: "test_tenant".to_string(),
                    context: "test 1".to_string(),
                    vector: vec![0.1, 0.2],
                    sync_status: SyncStatus::Pending,
                    last_sync_at: None,
                },
                RAGSyncRecord {
                    id: "2".to_string(),
                    tenant_id: "test_tenant".to_string(),
                    context: "test 2".to_string(),
                    vector: vec![0.3, 0.4],
                    sync_status: SyncStatus::Pending,
                    last_sync_at: None,
                },
            ])),
        };

        let pending = mock_service.fetch_pending_syncs(10).await.unwrap();
        assert_eq!(pending.len(), 2);

        let ids: Vec<String> = pending.iter().map(|r| r.id.clone()).collect();
        mock_service.mark_synced(ids).await.unwrap();

        let still_pending = mock_service.fetch_pending_syncs(10).await.unwrap();
        assert_eq!(still_pending.len(), 0);

        let incoming = vec![RAGSyncRecord {
            id: "3".to_string(),
            tenant_id: "test_tenant".to_string(),
            context: "test 3".to_string(),
            vector: vec![0.5, 0.6],
            sync_status: SyncStatus::Pending,
            last_sync_at: None,
        }];

        mock_service.process_incoming_sync(incoming).await.unwrap();

        let all_records = mock_service.records.lock().await;
        assert_eq!(all_records.len(), 3);
        assert_eq!(all_records[2].id, "3");
        assert_eq!(all_records[2].sync_status, SyncStatus::Synced);
    }

    #[tokio::test]
    async fn test_rag_sync_worker_isolation_rejection() {
        let mock_service = MockRAGSyncService {
            records: Arc::new(Mutex::new(vec![])),
        };

        let worker = RagSyncWorker {
            service: Arc::new(mock_service),
            redis_lock: Arc::new(crate::orchestration::locks::RedisLock::new(redis::Client::open("redis://127.0.0.1/").unwrap())),
        };

        // If redis is not running, we could fail trying to acquire lock.
        // But here we test isolation violation which happens *before* lock acquisition.
        let res = worker.sync_document("tenant_a", "tenant_b", "doc_1", vec![]).await;
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Tenant isolation violation");
    }

    // Add integration test to cover concurrent lock behavior if redis is available
    #[tokio::test]
    async fn test_rag_sync_worker_concurrent_lock() {
        let redis_url = std::env::var("OHC_REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());
        if redis::Client::open(redis_url.clone()).and_then(|c| c.get_connection()).is_err() {
            // Skip test if no redis
            return;
        }

        let mock_service = MockRAGSyncService {
            records: Arc::new(Mutex::new(vec![])),
        };

        let client = redis::Client::open(redis_url.clone()).unwrap();
        let worker = Arc::new(RagSyncWorker {
            service: Arc::new(mock_service),
            redis_lock: Arc::new(crate::orchestration::locks::RedisLock::new(client)),
        });

        let tenant_id = "test_tenant";
        let context_tenant_id = "test_tenant";
        let document_id = "concurrent_doc";

        // Acquire lock manually to simulate concurrent worker holding it
        let client_for_lock = redis::Client::open(redis_url).unwrap();
        let redis_lock_mock = crate::orchestration::locks::RedisLock::new(client_for_lock);
        let guard = redis_lock_mock.acquire_resource(tenant_id, "rag_sync", document_id).await.unwrap();

        // Worker should fail to acquire lock
        let res = worker.sync_document(tenant_id, context_tenant_id, document_id, vec![]).await;
        assert!(res.is_err());

        // Drop guard to release lock
        drop(guard);

        // Worker should now succeed
        let res2 = worker.sync_document(tenant_id, context_tenant_id, document_id, vec![]).await;
        assert!(res2.is_ok());
    }
}
