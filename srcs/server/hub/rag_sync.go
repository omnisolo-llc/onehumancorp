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
	Vector     []float32 // Convert to string internally for SQLite compat if needed
	SyncStatus SyncStatus
	LastSyncAt time.Time
}

type RAGSyncService interface {
	// FetchPendingSyncs retrieves records from the local DB that need syncing
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)

	// MarkSynced updates the local DB after a successful sync to the cloud
	MarkSynced(ctx context.Context, ids []string) error

	// ProcessIncomingSync handles data pushed from a standalone client into the cloud DB
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

var (
	meter            = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
	recordsSynced    metric.Int64Counter
	syncErrors       metric.Int64Counter
	syncLatency      metric.Float64Histogram
	recordsPending   metric.Int64UpDownCounter
)

func init() {
	var err error
	recordsSynced, err = meter.Int64Counter("rag_records_synced_total", metric.WithDescription("Total number of RAG records successfully synced"))
	if err != nil {
		panic(err)
	}

	syncErrors, err = meter.Int64Counter("rag_sync_errors_total", metric.WithDescription("Total number of errors encountered during RAG sync"))
	if err != nil {
		panic(err)
	}

	syncLatency, err = meter.Float64Histogram("rag_sync_latency_seconds", metric.WithDescription("Latency of RAG sync operations"))
	if err != nil {
		panic(err)
	}

	recordsPending, err = meter.Int64UpDownCounter("rag_sync_records_pending", metric.WithDescription("Number of RAG records pending sync"))
	if err != nil {
		panic(err)
	}
}

// Ensure observability instruments are accessible to implementations.
func RecordSyncSuccess(ctx context.Context, count int) {
	recordsSynced.Add(ctx, int64(count))
}

func RecordSyncError(ctx context.Context) {
	syncErrors.Add(ctx, 1)
}

func RecordSyncLatency(ctx context.Context, duration time.Duration) {
	syncLatency.Record(ctx, duration.Seconds())
}

func RecordPendingCount(ctx context.Context, count int) {
	recordsPending.Add(ctx, int64(count))
}
