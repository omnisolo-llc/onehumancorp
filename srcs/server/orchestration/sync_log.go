package orchestration

import (
	"time"
)

// SyncLog represents a record of a successfully synchronized memory.
type SyncLog struct {
	SyncID         string    `json:"sync_id"`
	MemoryID       string    `json:"memory_id"`
	CloudMissionID string    `json:"cloud_mission_id,omitempty"`
	SyncedAt       time.Time `json:"synced_at"`
}
