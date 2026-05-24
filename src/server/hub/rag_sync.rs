use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SyncStatus {
    Pending,
    Synced,
    Error,
}

impl std::fmt::Display for SyncStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SyncStatus::Pending => write!(f, "pending"),
            SyncStatus::Synced => write!(f, "synced"),
            SyncStatus::Error => write!(f, "error"),
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

#[async_trait::async_trait]
pub trait RAGSyncService: Send + Sync {
    async fn fetch_pending_syncs(&self, limit: i32) -> Result<Vec<RAGSyncRecord>, sqlx::Error>;
    async fn mark_synced(&self, ids: Vec<String>) -> Result<(), sqlx::Error>;
    async fn process_incoming_sync(&self, records: Vec<RAGSyncRecord>) -> Result<(), sqlx::Error>;
}
