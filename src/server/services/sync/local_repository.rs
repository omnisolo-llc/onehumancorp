use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MissionPayload {
    pub role: String,
    pub task: String,
    pub context: Option<String>,
    pub action_risk: Option<String>,
}

#[derive(Debug, Clone)]
pub struct LocalMission {
    pub id: String,
    pub organization_id: String,
    pub status: String,
    pub payload: MissionPayload,
    pub created_at: DateTime<Utc>,
    pub synced_to_cloud: bool,
    pub cloud_mission_id: Option<String>,
    pub sync_error: Option<String>,
    pub last_synced_at: Option<DateTime<Utc>>,
}

#[async_trait::async_trait]
pub trait LocalRepository: Send + Sync {
    async fn get_pending_sync(&self, organization_id: &str, limit: i32) -> Result<Vec<LocalMission>, String>;
    async fn mark_synced(&self, organization_id: &str, local_id: &str, cloud_id: &str) -> Result<(), String>;
    async fn mark_sync_error(&self, organization_id: &str, local_id: &str, sync_error: &str) -> Result<(), String>;
    async fn get_active_escalations(&self, organization_id: &str) -> Result<Vec<LocalMission>, String>;
    async fn update_local_status(&self, organization_id: &str, local_id: &str, new_status: &str) -> Result<(), String>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncMutation {
    pub id: String,
    pub organization_id: String,
    pub entity_type: String,
    pub entity_id: String,
    pub mutation_type: String,
    pub payload: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub synced_to_cloud: bool,
    pub sync_error: Option<String>,
    pub last_synced_at: Option<DateTime<Utc>>,
    pub version: i64,
}

#[async_trait::async_trait]
pub trait OfflineSyncRepository: Send + Sync {
    async fn enqueue_mutation(&self, mutation: SyncMutation) -> Result<(), String>;
    async fn get_pending_mutations(&self, organization_id: &str, limit: i32) -> Result<Vec<SyncMutation>, String>;
    async fn mark_mutation_synced(&self, organization_id: &str, id: &str) -> Result<(), String>;
    async fn mark_mutation_error(&self, organization_id: &str, id: &str, error: &str) -> Result<(), String>;
}
