package hub

import (
	"context"
	"time"
)

// AutoDreamSyncRecord represents a record to be synced between Standalone and Cloud mode.
type AutoDreamSyncRecord struct {
	ID           string
	MemoryData   string
	SyncStatus   string
	LastSyncAt   *time.Time
}

// AutoDreamSyncService manages the synchronization of AutoDream memories.
type AutoDreamSyncService interface {
	// FetchPendingSyncs retrieves records that have a pending sync status.
	FetchPendingSyncs(ctx context.Context, limit int) ([]*AutoDreamSyncRecord, error)

	// ProcessIncomingSync processes a sync record coming from a Standalone instance to the Cloud.
	ProcessIncomingSync(ctx context.Context, record *AutoDreamSyncRecord) error

	// MarkRecordSynced updates the local record's status to synced and updates its last sync time.
	MarkRecordSynced(ctx context.Context, id string) error
}
