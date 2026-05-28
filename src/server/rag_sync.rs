use std::sync::Arc;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use async_trait::async_trait;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SyncStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "synced")]
    Synced,
    #[serde(rename = "error")]
    Error,
}

impl ToString for SyncStatus {
    fn to_string(&self) -> String {
        match self {
            SyncStatus::Pending => "pending".to_string(),
            SyncStatus::Synced => "synced".to_string(),
            SyncStatus::Error => "error".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RAGSyncRecord {
    pub id: String,
    pub context: String,
    pub vector: Vec<f32>,
    pub sync_status: SyncStatus,
    pub last_sync_at: Option<DateTime<Utc>>,
}

#[async_trait]
pub trait RAGSyncService: Send + Sync {
    // FetchPendingSyncs retrieves records from the local DB that need syncing
    async fn fetch_pending_syncs(&self, limit: i32) -> Result<Vec<RAGSyncRecord>, String>;

    // MarkSynced updates the local DB after a successful sync to the cloud
    async fn mark_synced(&self, ids: Vec<String>) -> Result<(), String>;

    // ProcessIncomingSync handles data pushed from a standalone client into the cloud DB
    async fn process_incoming_sync(&self, records: Vec<RAGSyncRecord>) -> Result<(), String>;
}

pub async fn record_rag_sync_metrics(pool: &sqlx::PgPool, synced_count: usize, error_count: usize, tenant_id: &str) {
    let labels = serde_json::json!({
        "tenant_id": tenant_id
    });
    if synced_count > 0 {
        let _ = ::server_telemetry::buffer_metric(
            pool,
            "rag_records_synced_total",
            "counter",
            synced_count as f32,
            labels.clone()
        ).await;
    }
    if error_count > 0 {
        let _ = ::server_telemetry::buffer_metric(
            pool,
            "rag_sync_errors_total",
            "counter",
            error_count as f32,
            labels
        ).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    struct MockSyncService {
        pending_records: Mutex<Vec<RAGSyncRecord>>,
        synced_ids: Mutex<Vec<String>>,
        processed_records: Mutex<Vec<RAGSyncRecord>>,
    }

    impl MockSyncService {
        fn new() -> Self {
            Self {
                pending_records: Mutex::new(vec![RAGSyncRecord {
                    id: "test1".to_string(),
                    context: "Some AI context".to_string(),
                    vector: vec![0.1, 0.2],
                    sync_status: SyncStatus::Pending,
                    last_sync_at: None,
                }]),
                synced_ids: Mutex::new(vec![]),
                processed_records: Mutex::new(vec![]),
            }
        }
    }

    #[async_trait]
    impl RAGSyncService for MockSyncService {
        async fn fetch_pending_syncs(&self, limit: i32) -> Result<Vec<RAGSyncRecord>, String> {
            let mut pending = self.pending_records.lock().unwrap();
            let mut result = Vec::new();
            for _ in 0..limit.min(pending.len() as i32) {
                if let Some(record) = pending.pop() {
                    result.push(record);
                }
            }
            Ok(result)
        }

        async fn mark_synced(&self, ids: Vec<String>) -> Result<(), String> {
            let mut synced = self.synced_ids.lock().unwrap();
            synced.extend(ids);
            Ok(())
        }

        async fn process_incoming_sync(&self, records: Vec<RAGSyncRecord>) -> Result<(), String> {
            let mut processed = self.processed_records.lock().unwrap();
            processed.extend(records);
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_mock_rag_sync_flow() {
        let service = MockSyncService::new();

        // 1. Fetch pending
        let pending = service.fetch_pending_syncs(10).await.unwrap();
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].id, "test1");

        // 2. Process incoming
        let process_res = service.process_incoming_sync(pending.clone()).await;
        assert!(process_res.is_ok());

        let processed = service.processed_records.lock().unwrap();
        assert_eq!(processed.len(), 1);
        assert_eq!(processed[0].id, "test1");

        // 3. Mark synced
        let mark_res = service.mark_synced(vec!["test1".to_string()]).await;
        assert!(mark_res.is_ok());

        let synced = service.synced_ids.lock().unwrap();
        assert_eq!(synced.len(), 1);
        assert_eq!(synced[0], "test1");
    }
}