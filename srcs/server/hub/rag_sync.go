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
	ID           string
	Context      string
	Vector       []float32 // Convert to string internally for SQLite compat if needed
	SyncStatus   SyncStatus
	LastSyncAt   time.Time
}

type RAGSyncService interface {
	// FetchPendingSyncs retrieves records from the local DB that need syncing
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)

	// MarkSynced updates the local DB after a successful sync to the cloud
	MarkSynced(ctx context.Context, ids []string) error

	// ProcessIncomingSync handles data pushed from a standalone client into the cloud DB
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

// HubTelemetry manages OpenTelemetry metrics for the Hub package
type HubTelemetry struct {
	RecordsSyncedTotal metric.Int64Counter
	SyncErrorsTotal    metric.Int64Counter
}

// NewHubTelemetry creates and registers Hub OpenTelemetry metrics
func NewHubTelemetry(meter metric.Meter) (*HubTelemetry, error) {
	syncedTotal, err := meter.Int64Counter(
		"rag_records_synced_total",
		metric.WithDescription("Total number of RAG records successfully synchronized"),
	)
	if err != nil {
		return nil, err
	}

	errorsTotal, err := meter.Int64Counter(
		"rag_sync_errors_total",
		metric.WithDescription("Total number of RAG synchronization errors"),
	)
	if err != nil {
		return nil, err
	}

	return &HubTelemetry{
		RecordsSyncedTotal: syncedTotal,
		SyncErrorsTotal:    errorsTotal,
	}, nil
}
