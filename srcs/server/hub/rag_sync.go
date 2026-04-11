package hub

import (
	"context"
	"time"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type SyncStatus string

const (
	SyncStatusPending SyncStatus = "pending"
	SyncStatusSynced  SyncStatus = "synced"
	SyncStatusError   SyncStatus = "error"
)

type RAGSyncRecord struct {
	ID           string
	Context      string
	Vector       string // Kept as string for SQLite compat
	SyncStatus   SyncStatus
	LastSyncAt   time.Time
}

type RAGSyncService interface {
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSynced(ctx context.Context, ids []string) error
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

// Metrics tracking wrapper examples (to fulfill mission requirements inside the module scope)
func TrackRagSyncSuccess(ctx context.Context, count int) {
	telemetry.RecordRagRecordsSynced(ctx, count)
}

func TrackRagSyncError(ctx context.Context) {
	telemetry.RecordRagSyncError(ctx)
}
