use std::time::SystemTime;
use async_trait::async_trait;

#[derive(Debug, Clone, PartialEq)]
pub enum SyncStatus {
    Pending,
    Synced,
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

#[derive(Debug, Clone)]
pub struct RAGSyncRecord {
    pub id: String,
    pub context: String,
    pub vector: Vec<f32>,
    pub sync_status: SyncStatus,
    pub last_sync_at: Option<SystemTime>,
}

#[async_trait]
pub trait RAGSyncService: Send + Sync {
    async fn fetch_pending_syncs(&self, limit: usize) -> Result<Vec<RAGSyncRecord>, Box<dyn std::error::Error + Send + Sync>>;
    async fn mark_synced(&self, ids: &[String]) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    async fn process_incoming_sync(&self, records: Vec<RAGSyncRecord>) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}
