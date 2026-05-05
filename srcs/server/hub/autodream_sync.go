package hub

import (
	"context"
	"time"
)

// AutoDreamSyncRecord represents a record to be synced
type AutoDreamSyncRecord struct {
	ID        string    `json:"id"`
	Content   string    `json:"content"`
	Embedding []byte    `json:"embedding"`
	OrgID     string    `json:"organization_id"`
	AgentID   string    `json:"agent_id"`
	TaskID    string    `json:"task_id"`
	SyncStatus string   `json:"sync_status"`
	LastSyncAt *time.Time `json:"last_sync_at"`
}

// AutoDreamSyncService defines methods for managing sync state
type AutoDreamSyncService interface {
	FetchPendingSyncs(ctx context.Context, limit int) ([]AutoDreamSyncRecord, error)
	ProcessIncomingSyncs(ctx context.Context, records []AutoDreamSyncRecord) error
	MarkRecordsSynced(ctx context.Context, recordIDs []string) error
}
