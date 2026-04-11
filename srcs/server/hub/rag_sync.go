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

type RAGSyncMetrics struct {
    RecordsSynced metric.Int64Counter
    SyncErrors    metric.Int64Counter
}

func NewRAGSyncMetrics(meter metric.Meter) (*RAGSyncMetrics, error) {
    synced, err := meter.Int64Counter("rag_records_synced_total",
        metric.WithDescription("Total number of RAG records successfully synced"),
    )
    if err != nil {
        return nil, err
    }

    errors, err := meter.Int64Counter("rag_sync_errors_total",
        metric.WithDescription("Total number of errors encountered during RAG sync"),
    )
    if err != nil {
        return nil, err
    }

    return &RAGSyncMetrics{
        RecordsSynced: synced,
        SyncErrors:    errors,
    }, nil
}
