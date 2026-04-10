package hub

import (
    "context"
    "time"

    "go.opentelemetry.io/otel"
    "go.opentelemetry.io/otel/metric"
)

var meter = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")

var (
    ragRecordsSyncedTotal metric.Int64Counter
    ragSyncErrorsTotal    metric.Int64Counter
)

func init() {
    var err error
    ragRecordsSyncedTotal, err = meter.Int64Counter("rag_records_synced_total")
    if err != nil {
        panic(err)
    }
    ragSyncErrorsTotal, err = meter.Int64Counter("rag_sync_errors_total")
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

// ragSyncServiceImpl provides a concrete implementation of RAGSyncService
type ragSyncServiceImpl struct {
    // We would inject database provider here, like provider DBProvider etc.
    // For now we fulfill the interface requirement to make it testable/runnable
}

func NewRAGSyncService() RAGSyncService {
    return &ragSyncServiceImpl{}
}

func (s *ragSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    // Implementation to connect to SQLite/Postgres to fetch pending
    return nil, nil
}

func (s *ragSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
    // Implementation to connect to database to mark as synced
    ragRecordsSyncedTotal.Add(ctx, int64(len(ids)))
    return nil
}

func (s *ragSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    // Implementation to upsert into Postgres DB
    return nil
}
