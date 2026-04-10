package hub

import (
    "context"
    "time"

    "go.opentelemetry.io/otel"
    "go.opentelemetry.io/otel/metric"
    "log/slog"
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
    Vector       []float32
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
    meter := otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")

    var err error
    ragRecordsSyncedTotal, err = meter.Int64Counter("rag_records_synced_total")
    if err != nil {
        slog.Error("failed to initialize rag_records_synced_total metric", "error", err)
    }

    ragSyncErrorsTotal, err = meter.Int64Counter("rag_sync_errors_total")
    if err != nil {
        slog.Error("failed to initialize rag_sync_errors_total metric", "error", err)
    }
}
