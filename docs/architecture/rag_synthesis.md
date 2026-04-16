<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; background: rgba(255, 255, 255, 0.03); color: #fff; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# Hybrid MCP RAG Protocol - Synthesis

## Introduction
This document defines the schema and Go interface requirements for bridging the Local SQLite state with the Cloud PostgreSQL state.

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

## Go Interfaces

```go
package sync

import (
	"context"
	"time"
)

// MissionPayload defines the structure of the task payload
type MissionPayload struct {
	Role    string `json:"role"`
	Task    string `json:"task"`
	Context string `json:"context,omitempty"`
}

// LocalMission represents a row in the local agent_missions table
type LocalMission struct {
	ID             string
	Status         string
	Payload        MissionPayload
	CreatedAt      time.Time
	SyncedToCloud  bool
	CloudMissionID string
	SyncError      string
	LastSyncedAt   time.Time
}

// CloudSynchronizer handles pushing local missions to the cloud and pulling updates
type CloudSynchronizer interface {
	// PushPendingMissions finds tasks marked for escalation and sends them to the cloud
	PushPendingMissions(ctx context.Context) error

	// PullMissionUpdates polls the cloud for updates to previously escalated tasks
	PullMissionUpdates(ctx context.Context) error
}

// LocalRepository defines the interface for interacting with the local SQLite agent_missions table
type LocalRepository interface {
	GetPendingSync(ctx context.Context, limit int) ([]LocalMission, error)
	MarkSynced(ctx context.Context, localID string, cloudID string) error
	MarkSyncError(ctx context.Context, localID string, syncError string) error
	GetActiveEscalations(ctx context.Context) ([]LocalMission, error)
	UpdateLocalStatus(ctx context.Context, localID string, newStatus string) error
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
