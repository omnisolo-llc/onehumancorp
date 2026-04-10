package hub

import (
	"context"
	"log/slog"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

var (
	meter metric.Meter

	RagRecordsSyncedTotal metric.Int64Counter
	RagSyncErrorsTotal    metric.Int64Counter
)

func init() {
	meter = otel.GetMeterProvider().Meter("github.com/onehumancorp/mono/srcs/server/hub")

	var err error
	RagRecordsSyncedTotal, err = meter.Int64Counter("rag_records_synced_total", metric.WithDescription("Total number of RAG records synced"))
	if err != nil {
		slog.Error("failed to initialize rag_records_synced_total counter", "error", err)
	}

	RagSyncErrorsTotal, err = meter.Int64Counter("rag_sync_errors_total", metric.WithDescription("Total number of errors during RAG sync"))
	if err != nil {
		slog.Error("failed to initialize rag_sync_errors_total counter", "error", err)
	}
}

func RecordSyncSuccess(ctx context.Context, count int64) {
	if RagRecordsSyncedTotal != nil {
		RagRecordsSyncedTotal.Add(ctx, count)
	}
}

func RecordSyncError(ctx context.Context, count int64) {
	if RagSyncErrorsTotal != nil {
		RagSyncErrorsTotal.Add(ctx, count)
	}
}
