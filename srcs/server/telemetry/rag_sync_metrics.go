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

func initRAGSyncMetrics(meter metric.Meter) {
	var err error
	RAGRecordsSyncedTotal, err = meter.Int64Counter(
		"rag_records_synced_total",
		metric.WithDescription("Total number of RAG records synced"),
	)
	if err != nil {
		panic(err)
	}

	RAGSyncErrorsTotal, err = meter.Int64Counter(
		"rag_sync_errors_total",
		metric.WithDescription("Total number of errors encountered during RAG sync"),
	)
	if err != nil {
		panic(err)
	}
}

// RecordRAGSyncSuccess increments the synced counter
func RecordRAGSyncSuccess(ctx context.Context, count int) {
	if RAGRecordsSyncedTotal != nil {
		RAGRecordsSyncedTotal.Add(ctx, int64(count))
	}
}

// RecordRAGSyncError increments the error counter
func RecordRAGSyncError(ctx context.Context) {
	if RAGSyncErrorsTotal != nil {
		RAGSyncErrorsTotal.Add(ctx, 1)
	}
}

// Ensure it runs during init
func init() {
	meter := otel.Meter("github.com/onehumancorp/mono/srcs/server/telemetry")
	initRAGSyncMetrics(meter)
}
