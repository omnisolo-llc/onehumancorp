package hub

import (
	"context"
	"time"

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

func InitRAGSyncMetrics(meter metric.Meter) (metric.Int64Counter, metric.Int64Counter, error) {
	syncedTotal, err := meter.Int64Counter(
		"rag_records_synced_total",
		metric.WithDescription("Total number of RAG memory records successfully synced to the cloud"),
	)
	if err != nil {
		return nil, nil, err
	}

	errorsTotal, err := meter.Int64Counter(
		"rag_sync_errors_total",
		metric.WithDescription("Total number of errors encountered during RAG memory sync"),
	)
	if err != nil {
		return nil, nil, err
	}

	return syncedTotal, errorsTotal, nil
}
