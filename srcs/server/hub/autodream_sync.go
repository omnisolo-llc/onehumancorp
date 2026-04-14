package hub

import (
	"context"
	"time"
)

type AutoDreamSyncRecord struct {
	ID         string
	Context    string
	Vector     []float32
	SyncStatus SyncStatus
	LastSyncAt time.Time
}

type AutoDreamSyncService interface {
	FetchPendingSyncs(ctx context.Context, limit int) ([]AutoDreamSyncRecord, error)
	MarkSynced(ctx context.Context, ids []string) error
	ProcessIncomingSync(ctx context.Context, records []AutoDreamSyncRecord) error
}
