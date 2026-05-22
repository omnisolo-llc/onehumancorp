<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; background: rgba(255, 255, 255, 0.03);">

# Hybrid MCP RAG Protocol - Synthesis

## Introduction
This document defines the schema and Rust service requirements for bridging the local SQLite state with the cloud PostgreSQL state.

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

## Rust Interfaces

```rust
#[derive(serde::Serialize, serde::Deserialize)]
struct MissionPayload {
    role: String,
    task: String,
    context: Option<String>,
}

struct LocalMission {
    id: String,
    status: String,
    payload: MissionPayload,
    created_at: chrono::DateTime<chrono::Utc>,
    synced_to_cloud: bool,
    cloud_mission_id: Option<String>,
    sync_error: Option<String>,
    last_synced_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[async_trait::async_trait]
trait CloudSynchronizer {
    async fn push_pending_missions(&self) -> anyhow::Result<()>;
    async fn pull_mission_updates(&self) -> anyhow::Result<()>;
}

#[async_trait::async_trait]
trait LocalRepository {
    async fn get_pending_sync(&self, limit: i64) -> anyhow::Result<Vec<LocalMission>>;
    async fn mark_synced(&self, local_id: &str, cloud_id: &str) -> anyhow::Result<()>;
    async fn mark_sync_error(&self, local_id: &str, sync_error: &str) -> anyhow::Result<()>;
    async fn get_active_escalations(&self) -> anyhow::Result<Vec<LocalMission>>;
    async fn update_local_status(&self, local_id: &str, new_status: &str) -> anyhow::Result<()>;
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
