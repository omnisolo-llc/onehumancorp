use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MissionPayload {
    pub role: String,
    pub task: String,
    pub context: Option<String>,
}

#[derive(Debug, Clone)]
pub struct LocalMission {
    pub id: String,
    pub tenant_id: String,
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
    async fn get_pending_sync(&self, tenant_id: &str, limit: i32) -> Result<Vec<LocalMission>, String>;
    async fn mark_synced(&self, tenant_id: &str, local_id: &str, cloud_id: &str) -> Result<(), String>;
    async fn mark_sync_error(&self, tenant_id: &str, local_id: &str, sync_error: &str) -> Result<(), String>;
    async fn get_active_escalations(&self, tenant_id: &str) -> Result<Vec<LocalMission>, String>;
    async fn update_local_status(&self, tenant_id: &str, local_id: &str, new_status: &str) -> Result<(), String>;
}
