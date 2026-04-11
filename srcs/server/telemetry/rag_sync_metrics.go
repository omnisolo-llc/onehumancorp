package telemetry

import (
	"context"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

var (
	RAGRecordsSyncedTotal metric.Int64Counter
	RAGSyncErrorsTotal    metric.Int64Counter
)

func InitRAGSyncMetrics() error {
	meter := otel.Meter("github.com/onehumancorp/mono/ohc")

	var err error
	RAGRecordsSyncedTotal, err = meter.Int64Counter(
		"rag_records_synced_total",
		metric.WithDescription("Total number of RAG records successfully synced to the cloud"),
	)
	if err != nil {
		return err
	}

	RAGSyncErrorsTotal, err = meter.Int64Counter(
		"rag_sync_errors_total",
		metric.WithDescription("Total number of errors encountered during RAG sync"),
	)
	if err != nil {
		return err
	}

	return nil
}

func RecordRAGSyncSuccess(ctx context.Context, count int64) {
	if RAGRecordsSyncedTotal != nil {
		RAGRecordsSyncedTotal.Add(ctx, count)
	}
}

func RecordRAGSyncError(ctx context.Context) {
	if RAGSyncErrorsTotal != nil {
		RAGSyncErrorsTotal.Add(ctx, 1)
	}
}
