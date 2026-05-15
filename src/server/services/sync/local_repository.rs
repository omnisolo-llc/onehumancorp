use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
/// The `MissionPayload` struct acts as a primary component.
///
/// # Overview
/// This struct encapsulates the state necessary for execution.
///
/// # Thread Safety
/// Designed to be shared safely across async tokio tasks.
/// Uses types like `Arc` and `Mutex` to prevent race conditions.
///
/// # Performance
/// Optimized for low-latency operations.
///
/// # Usage Guidelines
/// - Created during initialization.
/// - Avoid holding synchronous locks across await points.
pub struct MissionPayload {
    pub role: String,
    pub task: String,
    pub context: Option<String>,
}

#[derive(Debug, Clone)]
/// The `LocalMission` struct acts as a primary component.
///
/// # Overview
/// This struct encapsulates the state necessary for execution.
///
/// # Thread Safety
/// Designed to be shared safely across async tokio tasks.
/// Uses types like `Arc` and `Mutex` to prevent race conditions.
///
/// # Performance
/// Optimized for low-latency operations.
///
/// # Usage Guidelines
/// - Created during initialization.
/// - Avoid holding synchronous locks across await points.
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
/// Defines the `LocalRepository:` trait.
///
/// # Overview
/// Used for dependency injection and mocking in tests.
pub trait LocalRepository: Send + Sync {
    async fn get_pending_sync(&self, organization_id: &str, limit: i32) -> Result<Vec<LocalMission>, String>;
    async fn mark_synced(&self, organization_id: &str, local_id: &str, cloud_id: &str) -> Result<(), String>;
    async fn mark_sync_error(&self, organization_id: &str, local_id: &str, sync_error: &str) -> Result<(), String>;
    async fn get_active_escalations(&self, organization_id: &str) -> Result<Vec<LocalMission>, String>;
    async fn update_local_status(&self, organization_id: &str, local_id: &str, new_status: &str) -> Result<(), String>;
}
