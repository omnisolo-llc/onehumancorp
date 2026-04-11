package hub

import (
	"context"
	"time"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

var meter = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")

var (
	RecordsSyncedTotal metric.Int64Counter
	SyncErrorsTotal    metric.Int64Counter
)

func init() {
	var err error
	RecordsSyncedTotal, err = meter.Int64Counter(
		"rag_records_synced_total",
		metric.WithDescription("Total number of RAG records synced to cloud"),
	)
	if err != nil {
		panic(err)
	}

	SyncErrorsTotal, err = meter.Int64Counter(
		"rag_sync_errors_total",
		metric.WithDescription("Total number of RAG sync errors"),
	)
	if err != nil {
		panic(err)
	}
}

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

type DBProvider interface {
    FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
    MarkSynced(ctx context.Context, ids []string) error
    ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type ragSyncServiceImpl struct {
    db DBProvider
}

func NewRAGSyncService(db DBProvider) RAGSyncService {
    return &ragSyncServiceImpl{db: db}
}

func (s *ragSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    records, err := s.db.FetchPendingSyncs(ctx, limit)
    if err != nil {
        SyncErrorsTotal.Add(ctx, 1)
        return nil, err
    }
    return records, nil
}

func (s *ragSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
    err := s.db.MarkSynced(ctx, ids)
    if err != nil {
         SyncErrorsTotal.Add(ctx, 1)
         return err
    }
    RecordsSyncedTotal.Add(ctx, int64(len(ids)))
    return nil
}

func (s *ragSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
     err := s.db.ProcessIncomingSync(ctx, records)
     if err != nil {
          SyncErrorsTotal.Add(ctx, 1)
          return err
     }
     RecordsSyncedTotal.Add(ctx, int64(len(records)))
     return nil
}
