use super::rag_sync::{RAGSyncRecord, RAGSyncService, SyncStatus};
use std::time::SystemTime;
use async_trait::async_trait;
use std::sync::RwLock;

pub struct MockRAGSyncService {
    pub records: RwLock<Vec<RAGSyncRecord>>,
}

#[async_trait]
impl RAGSyncService for MockRAGSyncService {
    async fn fetch_pending_syncs(&self, limit: usize) -> Result<Vec<RAGSyncRecord>, Box<dyn std::error::Error + Send + Sync>> {
        let records = self.records.read().unwrap();
        let mut result = Vec::new();
        for r in records.iter() {
            if r.sync_status == SyncStatus::Pending {
                result.push(r.clone());
            }
        }
        if limit > 0 && result.len() > limit {
            result.truncate(limit);
        }
        Ok(result)
    }

    async fn mark_synced(&self, ids: &[String]) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut records = self.records.write().unwrap();
        let mut count = 0;
        for r in records.iter_mut() {
            if ids.contains(&r.id) {
                r.sync_status = SyncStatus::Synced;
                r.last_sync_at = Some(SystemTime::now());
                count += 1;
            }
        }
        ::server_telemetry::get_rag_records_synced_counter().add(count, &[]);
        Ok(())
    }

    async fn process_incoming_sync(&self, records: Vec<RAGSyncRecord>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut existing_records = self.records.write().unwrap();
        existing_records.extend(records);
        Ok(())
    }
}

#[tokio::test]
async fn test_fetch_pending_syncs() {
    let service = MockRAGSyncService {
        records: RwLock::new(vec![
            RAGSyncRecord { id: "1".to_string(), context: "a".to_string(), vector: vec![], sync_status: SyncStatus::Pending, last_sync_at: None },
            RAGSyncRecord { id: "2".to_string(), context: "b".to_string(), vector: vec![], sync_status: SyncStatus::Synced, last_sync_at: None },
            RAGSyncRecord { id: "3".to_string(), context: "c".to_string(), vector: vec![], sync_status: SyncStatus::Pending, last_sync_at: None },
        ]),
    };

    let records = service.fetch_pending_syncs(10).await.unwrap();
    assert_eq!(records.len(), 2);
}

#[tokio::test]
async fn test_mark_synced() {
    let service = MockRAGSyncService {
        records: RwLock::new(vec![
            RAGSyncRecord { id: "1".to_string(), context: "a".to_string(), vector: vec![], sync_status: SyncStatus::Pending, last_sync_at: None },
            RAGSyncRecord { id: "2".to_string(), context: "b".to_string(), vector: vec![], sync_status: SyncStatus::Pending, last_sync_at: None },
        ]),
    };

    service.mark_synced(&["1".to_string()]).await.unwrap();

    let records = service.records.read().unwrap();
    assert_eq!(records[0].sync_status, SyncStatus::Synced);
    assert!(records[0].last_sync_at.is_some());
    assert_eq!(records[1].sync_status, SyncStatus::Pending);
}

#[tokio::test]
async fn test_process_incoming_sync() {
    let service = MockRAGSyncService {
        records: RwLock::new(vec![]),
    };

    let records = vec![
        RAGSyncRecord { id: "1".to_string(), context: "a".to_string(), vector: vec![], sync_status: SyncStatus::Synced, last_sync_at: None },
    ];

    service.process_incoming_sync(records).await.unwrap();

    let existing_records = service.records.read().unwrap();
    assert_eq!(existing_records.len(), 1);
}
