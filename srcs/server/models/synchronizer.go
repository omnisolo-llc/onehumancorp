package models

import "time"

// SyncLog represents a record in the local_cloud_sync_log table.
type SyncLog struct {
	SyncID         string    `json:"sync_id"`
	MemoryID       string    `json:"memory_id"`
	CloudMissionID string    `json:"cloud_mission_id"`
	SyncedAt       time.Time `json:"synced_at"`
}
