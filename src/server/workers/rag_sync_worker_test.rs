use super::rag_sync_worker::RagSyncWorker;
use crate::rag_sync::{RAGSyncService, RAGSyncRecord, SyncStatus};
use crate::orchestration::locks::{DistributedLock, StandaloneLock, LockGuard};
use std::sync::Arc;
use tokio::sync::Mutex;
use chrono::Utc;

struct MockRAGSyncService {
    records: Arc<Mutex<Vec<RAGSyncRecord>>>,
}

#[async_trait::async_trait]
impl RAGSyncService for MockRAGSyncService {
    async fn fetch_pending_syncs(&self, tenant_id: &str, limit: i32) -> Result<Vec<RAGSyncRecord>, String> {
        let records = self.records.lock().await;
        let pending: Vec<RAGSyncRecord> = records
            .iter()
            .filter(|r| r.tenant_id == tenant_id && r.sync_status == SyncStatus::Pending)
            .take(limit as usize)
            .cloned()
            .collect();
        Ok(pending)
    }

    async fn mark_synced(&self, tenant_id: &str, ids: Vec<String>) -> Result<(), String> {
        let mut records = self.records.lock().await;
        for record in records.iter_mut() {
            if record.tenant_id == tenant_id && ids.contains(&record.id) {
                record.sync_status = SyncStatus::Synced;
                record.last_sync_at = Some(Utc::now());
            }
        }
        Ok(())
    }

    async fn process_incoming_sync(&self, tenant_id: &str, incoming_records: Vec<RAGSyncRecord>) -> Result<(), String> {
        Ok(())
    }
}

#[tokio::test]
async fn test_rag_sync_worker_isolation() {
    let mock_service = Arc::new(MockRAGSyncService {
        records: Arc::new(Mutex::new(vec![
            RAGSyncRecord {
                id: "doc-1".to_string(),
                tenant_id: "tenant-1".to_string(),
                context: "policy".to_string(),
                vector: vec![0.1],
                sync_status: SyncStatus::Pending,
                last_sync_at: None,
            },
        ])),
    });

    let distributed_lock = Arc::new(StandaloneLock::new());
    let worker = RagSyncWorker::new(mock_service.clone(), distributed_lock.clone(), "tenant-1".to_string());

    // Run one iteration of the loop
    worker.process_pending_syncs().await.unwrap();

    // Check if synced
    let records = mock_service.records.lock().await;
    assert_eq!(records[0].sync_status, SyncStatus::Synced);

    // Verify lock was used
    let lock_clone = distributed_lock.locks.lock().await;
    let key = "ohc:lock:tenant-1:rag_sync:doc-1".to_string();
    assert!(lock_clone.contains_key(&key));
}

#[tokio::test]
async fn test_rag_sync_worker_concurrent_lock() {
    let mock_service = Arc::new(MockRAGSyncService {
        records: Arc::new(Mutex::new(vec![
            RAGSyncRecord {
                id: "doc-1".to_string(),
                tenant_id: "tenant-1".to_string(),
                context: "policy".to_string(),
                vector: vec![0.1],
                sync_status: SyncStatus::Pending,
                last_sync_at: None,
            },
        ])),
    });

    let distributed_lock = Arc::new(StandaloneLock::new());
    let worker = RagSyncWorker::new(mock_service.clone(), distributed_lock.clone(), "tenant-1".to_string());

    // Acquire lock externally to simulate concurrent access
    let _guard = distributed_lock.acquire_resource("tenant-1", "rag_sync", "doc-1").await.unwrap();

    // Spawn the worker logic to run concurrently
    let worker_future = worker.process_pending_syncs();

    // We expect the worker to be blocked, but let's just let it timeout or skip depending on implementation
    // For standalone lock, it blocks. We can use tokio::select to timeout
    let result = tokio::time::timeout(std::time::Duration::from_millis(100), worker_future).await;
    assert!(result.is_err()); // Timeout means it was blocked waiting for lock
}
