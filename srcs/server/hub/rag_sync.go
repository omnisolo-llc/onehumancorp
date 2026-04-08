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
	meter metric.Meter

	RagRecordsSyncedTotal metric.Int64Counter
	RagSyncErrorsTotal    metric.Int64Counter
)

func init() {
	meter = otel.GetMeterProvider().Meter("ohc-hub-rag-sync")

	var err error
	RagRecordsSyncedTotal, err = meter.Int64Counter(
		"ohc_rag_records_synced_total",
		metric.WithDescription("Total number of RAG records synced"),
	)
	if err != nil {
		panic(err)
	}

	RagSyncErrorsTotal, err = meter.Int64Counter(
		"ohc_rag_sync_errors_total",
		metric.WithDescription("Total number of errors during RAG sync"),
	)
	if err != nil {
		panic(err)
	}
}
