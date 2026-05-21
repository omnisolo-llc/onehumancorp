<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; background: rgba(255, 255, 255, 0.03);">

# Hybrid MCP RAG Protocol - Synthesis

## Introduction
This document defines the schema and Rust trait requirements for bridging the Local SQLite state with the Cloud PostgreSQL state.

## Schema Requirements
The `agent_missions` table in the SQLite daemon must be augmented with additional columns to facilitate synchronization.

```sql
-- SQLite Schema additions for Standalone Agent
ALTER TABLE agent_missions ADD COLUMN synced_to_cloud BOOLEAN DEFAULT FALSE;
ALTER TABLE agent_missions ADD COLUMN cloud_mission_id TEXT;
ALTER TABLE agent_missions ADD COLUMN sync_error TEXT;
ALTER TABLE agent_missions ADD COLUMN last_synced_at TIMESTAMP;
```

These fields allow the daemon to track which tasks have been escalated, track errors, and receive the ID returned from the Cloud.

## Rust Implementations

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// MissionPayload defines the structure of the task payload
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MissionPayload {
    pub role: String,
    pub task: String,
    pub context: Option<String>,
}

// LocalMission represents a row in the local agent_missions table
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

// CloudSynchronizer handles pushing local missions to the cloud and pulling updates
#[async_trait::async_trait]
pub trait CloudSynchronizer: Send + Sync {
    // PushPendingMissions finds tasks marked for escalation and sends them to the cloud
    async fn push_pending_missions(&self, tenant_id: &str) -> Result<(), String>;

    // PullMissionUpdates polls the cloud for updates to previously escalated tasks
    async fn pull_mission_updates(&self, tenant_id: &str) -> Result<(), String>;
}

// LocalRepository defines the interface for interacting with the local SQLite agent_missions table
#[async_trait::async_trait]
pub trait LocalRepository: Send + Sync {
    async fn get_pending_sync(&self, tenant_id: &str, limit: i32) -> Result<Vec<LocalMission>, String>;
    async fn mark_synced(&self, tenant_id: &str, local_id: &str, cloud_id: &str) -> Result<(), String>;
    async fn mark_sync_error(&self, tenant_id: &str, local_id: &str, sync_error: &str) -> Result<(), String>;
    async fn get_active_escalations(&self, tenant_id: &str) -> Result<Vec<LocalMission>, String>;
    async fn update_local_status(&self, tenant_id: &str, local_id: &str, new_status: &str) -> Result<(), String>;
}
```

## API Contract (Cloud REST Endpoint)
The Local Synchronizer will communicate with the Cloud via REST.

**Endpoint:** `POST /api/v1/missions/escalate`
**Request Payload:**
```json
{
  "local_id": "m-local-uuid",
  "payload": {
    "role": "data_analysis",
    "task": "Compute embeddings for local dump",
    "context": "<sanitized context payload>"
  }
}
```

**Response Payload:**
```json
{
  "cloud_id": "m-cloud-uuid",
  "status": "ACCEPTED"
}
```

**Endpoint:** `GET /api/v1/missions/{cloud_id}/status`
**Response Payload:**
```json
{
  "cloud_id": "m-cloud-uuid",
  "status": "DONE",
  "result": "<computed result from k8s pod>"
}
```
</div>
