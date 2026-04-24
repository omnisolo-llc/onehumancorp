package sync

// SyncStatus represents the current state of synchronization
type SyncStatus string

const (
	SyncStatusIdle       SyncStatus = "IDLE"
	SyncStatusSyncing    SyncStatus = "SYNCING"
	SyncStatusError      SyncStatus = "ERROR"
	SyncStatusOffline    SyncStatus = "OFFLINE"
	SyncStatusUpToDate   SyncStatus = "UP_TO_DATE"
)

// HybridSynchronizer defines the interface for local-to-cloud synchronization.
type HybridSynchronizer interface {
	// StartSync initiates the synchronization process.
	StartSync() error

	// StopSync halts the synchronization process.
	StopSync() error

	// GetSyncStatus returns the current status of the synchronization.
	GetSyncStatus() SyncStatus
}
