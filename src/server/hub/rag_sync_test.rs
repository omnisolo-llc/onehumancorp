use super::rag_sync::{RAGSyncService, RAGSyncRecord, SyncStatus};
use tokio::sync::Mutex;

pub struct MockRAGSyncService {
    pub pending_records: Mutex<Vec<RAGSyncRecord>>,
    pub synced_ids: Mutex<Vec<String>>,
    pub processed: Mutex<Vec<RAGSyncRecord>>,
}

impl MockRAGSyncService {
    pub fn new() -> Self {
        Self {
            pending_records: Mutex::new(Vec::new()),
            synced_ids: Mutex::new(Vec::new()),
            processed: Mutex::new(Vec::new()),
        }
    }
}

#[async_trait::async_trait]
impl RAGSyncService for MockRAGSyncService {
    async fn fetch_pending_syncs(&self, limit: i32) -> Result<Vec<RAGSyncRecord>, sqlx::Error> {
        let records = self.pending_records.lock().await;
        let mut result = Vec::new();
        for i in 0..std::cmp::min(limit as usize, records.len()) {
            result.push(records[i].clone());
        }
        Ok(result)
    }

    async fn mark_synced(&self, ids: Vec<String>) -> Result<(), sqlx::Error> {
        let mut synced = self.synced_ids.lock().await;
        synced.extend(ids);
        Ok(())
    }

    async fn process_incoming_sync(&self, records: Vec<RAGSyncRecord>) -> Result<(), sqlx::Error> {
        let mut processed = self.processed.lock().await;
        processed.extend(records);
        Ok(())
    }
}

#[tokio::test]
async fn test_fetch_pending_syncs() {
    let service = MockRAGSyncService::new();
    service.pending_records.lock().await.push(RAGSyncRecord {
        id: "1".to_string(),
        context: "test".to_string(),
        vector: vec![0.1, 0.2],
        sync_status: SyncStatus::Pending,
        last_sync_at: None,
    });

    let result = service.fetch_pending_syncs(10).await.unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id, "1");
}

#[tokio::test]
async fn test_mark_synced() {
    let service = MockRAGSyncService::new();
    service.mark_synced(vec!["1".to_string()]).await.unwrap();

    let synced = service.synced_ids.lock().await;
    assert_eq!(synced.len(), 1);
    assert_eq!(synced[0], "1");
}

#[tokio::test]
async fn test_process_incoming_sync() {
    let service = MockRAGSyncService::new();
    service.process_incoming_sync(vec![RAGSyncRecord {
        id: "1".to_string(),
        context: "test".to_string(),
        vector: vec![0.1, 0.2],
        sync_status: SyncStatus::Pending,
        last_sync_at: None,
    }]).await.unwrap();

    let processed = service.processed.lock().await;
    assert_eq!(processed.len(), 1);
    assert_eq!(processed[0].id, "1");
}
