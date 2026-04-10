package telemetry

import (
    "context"
    "go.opentelemetry.io/otel"
    "go.opentelemetry.io/otel/metric"
)

var (
    ragRecordsSyncedTotal metric.Int64Counter
    ragSyncErrorsTotal    metric.Int64Counter
)

func init() {
    meter := otel.Meter("github.com/onehumancorp/mono/ohc")
    ragRecordsSyncedTotal, _ = meter.Int64Counter("rag_records_synced_total", metric.WithDescription("Total number of RAG records synced to the cloud"))
    ragSyncErrorsTotal, _ = meter.Int64Counter("rag_sync_errors_total", metric.WithDescription("Total number of errors encountered while syncing RAG records"))
}

func RecordRAGSyncSuccess(ctx context.Context, count int) {
    if ragRecordsSyncedTotal != nil {
        ragRecordsSyncedTotal.Add(ctx, int64(count))
    }
}

func RecordRAGSyncError(ctx context.Context) {
    if ragSyncErrorsTotal != nil {
        ragSyncErrorsTotal.Add(ctx, 1)
    }
}
