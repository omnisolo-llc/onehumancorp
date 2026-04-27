# Hybrid MCP RAG Protocol - Synthesis Analysis

This report documents the architectural requirements for bridging Local SQLite state with Cloud PostgreSQL state in the One Human Corp (OHC) Hybrid MCP RAG Protocol.

## Schema Requirements

The `agent_missions` table in the SQLite daemon requires the following augmentations for synchronization:

```sql
-- SQLite Schema additions for Standalone Agent
ALTER TABLE agent_missions ADD COLUMN synced_to_cloud BOOLEAN DEFAULT FALSE;
ALTER TABLE agent_missions ADD COLUMN cloud_mission_id TEXT;
ALTER TABLE agent_missions ADD COLUMN sync_error TEXT;
ALTER TABLE agent_missions ADD COLUMN last_synced_at TIMESTAMP;
```

These fields enable tracking of escalated tasks, error logging, and storage of the ID returned from the Cloud.

## Go Interfaces

The following Go interfaces define the contract for synchronization logic:

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

Communication between the Local Synchronizer and the Cloud operates via REST endpoints.

### Escalation Request
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

### Status Polling
**Endpoint:** `GET /api/v1/missions/{cloud_id}/status`

**Response Payload:**
```json
{
  "cloud_id": "m-cloud-uuid",
  "status": "DONE",
  "result": "<computed result from k8s pod>"
}
```
