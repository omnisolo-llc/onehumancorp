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
    Vector       []byte
    SyncStatus   SyncStatus
    LastSyncAt   time.Time
}

type RAGSyncService interface {
    FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
    MarkSynced(ctx context.Context, ids []string) error
    ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

var (
    RecordsSyncedTotal metric.Int64Counter
    SyncErrorsTotal    metric.Int64Counter
)

func init() {
    meter := otel.Meter("ohc.server.hub")
    RecordsSyncedTotal, _ = meter.Int64Counter(
        "rag_records_synced_total",
        metric.WithDescription("Total number of RAG records successfully synced"),
    )
    SyncErrorsTotal, _ = meter.Int64Counter(
        "rag_sync_errors_total",
        metric.WithDescription("Total number of errors encountered during RAG sync"),
    )
}
