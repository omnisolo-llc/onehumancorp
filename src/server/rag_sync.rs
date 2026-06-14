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
                    context: "test 1".to_string(),
                    vector: vec![0.1, 0.2],
                    sync_status: SyncStatus::Pending,
                    last_sync_at: None,
                },
                RAGSyncRecord {
                    id: "2".to_string(),
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
}
