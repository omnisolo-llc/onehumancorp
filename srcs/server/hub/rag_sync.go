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
    ID           string
    Context      string
    Vector       []byte // Mapped to []byte per memory instructions
    SyncStatus   SyncStatus
    LastSyncAt   time.Time
}

type RAGSyncService interface {
    FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
    MarkSynced(ctx context.Context, ids []string) error
    ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

var (
    ragRecordsSyncedTotal metric.Int64Counter
    ragSyncErrorsTotal    metric.Int64Counter
)

func init() {
    meter := otel.Meter("hybrid-rag-sync")
    var err error
    ragRecordsSyncedTotal, err = meter.Int64Counter("rag_records_synced_total", metric.WithDescription("Total RAG records synced to cloud"))
    if err != nil {
        panic(err)
    }
    ragSyncErrorsTotal, err = meter.Int64Counter("rag_sync_errors_total", metric.WithDescription("Total errors during RAG sync"))
    if err != nil {
        panic(err)
    }
}
