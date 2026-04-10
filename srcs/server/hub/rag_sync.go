package hub

import (
    "context"
    "time"

    "go.opentelemetry.io/otel"
    "go.opentelemetry.io/otel/metric"
)

var (
    meter = otel.Meter("github.com/onehumancorp/ohc/srcs/server/hub")
    ragRecordsSyncedTotal metric.Int64Counter
    ragSyncErrorsTotal    metric.Int64Counter
)

func init() {
    var err error
    ragRecordsSyncedTotal, err = meter.Int64Counter("rag_records_synced_total", metric.WithDescription("Total number of RAG records synced"))
    if err != nil {
        panic(err)
    }
    ragSyncErrorsTotal, err = meter.Int64Counter("rag_sync_errors_total", metric.WithDescription("Total number of RAG sync errors"))
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
    Vector       []byte
    SyncStatus   SyncStatus
    LastSyncAt   time.Time
}

type RAGSyncService interface {
    FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
    MarkSynced(ctx context.Context, ids []string) error
    ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type BasicRAGSyncService struct {}

func NewBasicRAGSyncService() *BasicRAGSyncService {
    return &BasicRAGSyncService{}
}

func (s *BasicRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    return []RAGSyncRecord{}, nil
}

func (s *BasicRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
    if len(ids) > 0 {
        ragRecordsSyncedTotal.Add(ctx, int64(len(ids)))
    }
    return nil
}

func (s *BasicRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    if len(records) > 0 {
        ragRecordsSyncedTotal.Add(ctx, int64(len(records)))
    }
    return nil
}
