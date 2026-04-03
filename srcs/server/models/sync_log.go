package models

import "time"

// SyncLog tracks memories that have been synchronized to the cloud.
type SyncLog struct {
	SyncID         string
	MemoryID       string
	CloudMissionID string
	SyncedAt       time.Time
}
