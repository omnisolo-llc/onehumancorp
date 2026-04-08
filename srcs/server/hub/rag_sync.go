package hub

import (
	"context"
	"log"
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
	RecordsSyncedCounter metric.Int64Counter
	SyncErrorsCounter    metric.Int64Counter
)

func init() {
	var err error
	RecordsSyncedCounter, err = meter.Int64Counter(
		"rag_records_synced_total",
		metric.WithDescription("Total number of RAG records successfully synced to the cloud"),
	)
	if err != nil {
		log.Printf("Failed to initialize rag_records_synced_total metric: %v", err)
	}

	SyncErrorsCounter, err = meter.Int64Counter(
		"rag_sync_errors_total",
		metric.WithDescription("Total number of RAG record sync errors"),
	)
	if err != nil {
		log.Printf("Failed to initialize rag_sync_errors_total metric: %v", err)
	}
}

// RecordSyncSuccess records a successful sync operation
func RecordSyncSuccess(ctx context.Context, count int) {
	if RecordsSyncedCounter != nil {
		RecordsSyncedCounter.Add(ctx, int64(count))
	}
}

// RecordSyncError records a sync error
func RecordSyncError(ctx context.Context) {
	if SyncErrorsCounter != nil {
		SyncErrorsCounter.Add(ctx, 1)
	}
}
