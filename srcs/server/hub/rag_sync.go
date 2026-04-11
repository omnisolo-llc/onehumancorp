package hub

import (
	"context"
	"fmt"
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

var (
	ragRecordsSyncedTotal metric.Int64Counter
	ragSyncErrorsTotal    metric.Int64Counter
)

func InitMetrics(m metric.Meter) error {
	var err error
	ragRecordsSyncedTotal, err = m.Int64Counter("rag_records_synced_total", metric.WithDescription("Total number of RAG records successfully synced"))
	if err != nil {
		return fmt.Errorf("failed to initialize rag_records_synced_total: %w", err)
	}

	ragSyncErrorsTotal, err = m.Int64Counter("rag_sync_errors_total", metric.WithDescription("Total number of RAG sync errors"))
	if err != nil {
		return fmt.Errorf("failed to initialize rag_sync_errors_total: %w", err)
	}
	return nil
}

// Ensure init matches what's used if needed or we let the hub or telemetry explicitly init it.
