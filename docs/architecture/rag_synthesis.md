# Hybrid MCP RAG Protocol - Phase 2 (Synthesis)

## 1. Overview
This document specifies the technical design for the Local-to-Cloud Context Synchronizer, allowing the Standalone Agent to delegate complex tasks to the Cloud via the `agent_missions` table.

## 2. DB Schema Definition (SQLite)
To support task escalation, we need to extend the local `agent_missions` SQLite table. This tracks the synchronization state of tasks delegated to the cloud.

```sql
-- Migration: Add cloud sync columns to agent_missions
ALTER TABLE agent_missions ADD COLUMN cloud_sync_status VARCHAR(50) DEFAULT 'LOCAL_ONLY';
ALTER TABLE agent_missions ADD COLUMN cloud_mission_id TEXT UNIQUE;
ALTER TABLE agent_missions ADD COLUMN sync_error_details TEXT;
ALTER TABLE agent_missions ADD COLUMN last_sync_attempt_at TIMESTAMP;
```

## 3. Go Interfaces for Synchronizer
The local daemon requires interfaces to manage the sync lifecycle between SQLite and the Cloud REST API.

```go
package sync

import (
	"context"
	"time"
)

// MissionContext defines the payload structure for injecting tasks.
type MissionContext struct {
	Role         string            `json:"role"`
	Task         string            `json:"task"`
	ContextData  string            `json:"context_data,omitempty"`
	Dependencies map[string]string `json:"dependencies,omitempty"`
}

// LocalAgentMission maps to a row in the agent_missions table.
type LocalAgentMission struct {
	ID                  string
	Status              string
	Payload             MissionContext
	CreatedAt           time.Time
	CloudSyncStatus     string
	CloudMissionID      *string
	SyncErrorDetails    *string
	LastSyncAttemptAt   *time.Time
}

// CloudMissionSynchronizer manages the background synchronization protocol.
type CloudMissionSynchronizer interface {
	// EscalatePendingTasks scans agent_missions for tasks requiring cloud escalation
	// and pushes them to the Cloud REST endpoint.
	EscalatePendingTasks(ctx context.Context) error

	// SyncTaskStatus polls the Cloud for status updates on escalated tasks
	// and updates the local agent_missions table accordingly.
	SyncTaskStatus(ctx context.Context) error
}

// MissionRepository provides CRUD operations for agent_missions.
type MissionRepository interface {
	GetPendingEscalations(ctx context.Context, batchSize int) ([]LocalAgentMission, error)
	UpdateSyncState(ctx context.Context, localID string, cloudID string, status string) error
	RecordSyncError(ctx context.Context, localID string, err string) error
}
```

## 4. API Contract (Cloud REST Endpoint)
The Local Synchronizer pushes tasks to the Cloud using the following REST API contract.

### 4.1 Escalate Mission
**Endpoint:** `POST /api/v2/missions/escalate`
**Request Payload:**
```json
{
  "local_mission_id": "uuid-local-1234",
  "mission_context": {
    "role": "data_analysis",
    "task": "Compute RAG embeddings for offline dataset",
    "context_data": "<base64_encoded_dump>"
  }
}
```

**Response Payload (Success):**
```json
{
  "cloud_mission_id": "uuid-cloud-5678",
  "status": "QUEUED"
}
```

### 4.2 Poll Mission Status
**Endpoint:** `GET /api/v2/missions/{cloud_mission_id}/status`
**Response Payload:**
```json
{
  "cloud_mission_id": "uuid-cloud-5678",
  "status": "COMPLETED",
  "result_artifact_url": "https://storage.onehumancorp.com/results/uuid-cloud-5678.zip",
  "completed_at": "2026-04-13T22:00:00Z"
}
```
