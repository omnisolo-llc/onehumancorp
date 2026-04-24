package hub

import (
	"context"
	"time"
)

// AutoDreamSyncRecord represents a memory record pending sync.
type AutoDreamSyncRecord struct {
	ID        string    `json:"id"`
	AgentID   string    `json:"agent_id"`
	Content   string    `json:"content"`
	CreatedAt time.Time `json:"created_at"`
	SyncStatus string   `json:"sync_status"`
	LastSyncAt *time.Time `json:"last_sync_at"`
}

// AutoDreamSyncService defines the interface for synchronizing AutoDream memories.
type AutoDreamSyncService interface {
	// FetchPendingSyncs retrieves records that are ready to be synced.
	FetchPendingSyncs(ctx context.Context, limit int) ([]*AutoDreamSyncRecord, error)
	// ProcessIncomingSync handles an incoming sync payload.
	ProcessIncomingSync(ctx context.Context, payload *AutoDreamSyncRecord) error
	// MarkRecordSynced marks a specific record as successfully synced.
	MarkRecordSynced(ctx context.Context, recordID string) error
}
