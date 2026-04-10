package hub

import (
	"context"
	"time"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

type SyncStatus string

const (
	SyncStatusPending SyncStatus = "pending"
	SyncStatusSynced  SyncStatus = "synced"
	SyncStatusError   SyncStatus = "error"
)

type RAGSyncRecord struct {
	ID         string
	Context    string
	Vector     []float32
	SyncStatus SyncStatus
	LastSyncAt time.Time
}

type RAGSyncService interface {
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSynced(ctx context.Context, ids []string) error
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

var (
	meter                = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
	RecordsSyncedTotal, _ = meter.Int64Counter("rag_records_synced_total")
	SyncErrorsTotal, _    = meter.Int64Counter("rag_sync_errors_total")
)

// ensure metric import isn't completely useless to compile...
var _ = metric.WithDescription("")
