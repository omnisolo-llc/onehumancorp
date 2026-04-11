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
	meter                   = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
	ragRecordsSyncedCounter metric.Int64Counter
	ragSyncErrorsCounter    metric.Int64Counter
)

func init() {
	var err error
	ragRecordsSyncedCounter, err = meter.Int64Counter("rag_records_synced_total",
		metric.WithDescription("Total number of RAG records successfully synced to the cloud"),
	)
	if err != nil {
		panic(err)
	}

	ragSyncErrorsCounter, err = meter.Int64Counter("rag_sync_errors_total",
		metric.WithDescription("Total number of errors encountered during RAG sync"),
	)
	if err != nil {
		panic(err)
	}
}

// RecordSyncSuccess increments the counter for successfully synced RAG records
func RecordSyncSuccess(ctx context.Context, count int) {
	ragRecordsSyncedCounter.Add(ctx, int64(count))
}

// RecordSyncError increments the counter for RAG sync errors
func RecordSyncError(ctx context.Context) {
	ragSyncErrorsCounter.Add(ctx, 1)
}

type RAGSyncServiceImpl struct {
	// Database connection or context can be injected here
}

func NewRAGSyncService() *RAGSyncServiceImpl {
	return &RAGSyncServiceImpl{}
}

func (s *RAGSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	// Concrete logic to fetch from SQLite goes here
	return []RAGSyncRecord{}, nil
}

func (s *RAGSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	// Concrete logic to mark local SQLite records as synced goes here
	return nil
}

func (s *RAGSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	// Concrete logic to insert into PostgreSQL cloud DB goes here
	return nil
}
