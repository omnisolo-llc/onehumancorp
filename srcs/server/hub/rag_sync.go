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
	Vector     []float32
	SyncStatus SyncStatus
	LastSyncAt time.Time
}

type RAGSyncService interface {
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSynced(ctx context.Context, ids []string) error
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type Metrics struct {
	RecordsSyncedTotal metric.Int64Counter
	SyncErrorsTotal    metric.Int64Counter
}

func NewMetrics(meter metric.Meter) (*Metrics, error) {
	syncedTotal, err := meter.Int64Counter("rag_records_synced_total")
	if err != nil {
		return nil, err
	}
	errorsTotal, err := meter.Int64Counter("rag_sync_errors_total")
	if err != nil {
		return nil, err
	}
	return &Metrics{
		RecordsSyncedTotal: syncedTotal,
		SyncErrorsTotal:    errorsTotal,
	}, nil
}
