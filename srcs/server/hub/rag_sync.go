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
	Vector     []byte
	SyncStatus SyncStatus
	LastSyncAt time.Time
}

var (
	RAGRecordsSyncedTotal metric.Int64Counter
	RAGSyncErrorsTotal    metric.Int64Counter
)

func InitRAGSyncMetrics(meter metric.Meter) error {
	var err error
	RAGRecordsSyncedTotal, err = meter.Int64Counter(
		"rag_records_synced_total",
		metric.WithDescription("Total number of RAG records synced to cloud"),
	)
	if err != nil {
		return err
	}

	RAGSyncErrorsTotal, err = meter.Int64Counter(
		"rag_sync_errors_total",
		metric.WithDescription("Total number of errors during RAG sync"),
	)
	if err != nil {
		return err
	}

	return nil
}

type RAGSyncService interface {
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSynced(ctx context.Context, ids []string) error
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}
