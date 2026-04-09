package telemetry

import (
	"context"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

var (
	ragMeter                = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
	ragRecordsSyncedTotal metric.Int64Counter
	ragSyncErrorsTotal    metric.Int64Counter
)

func init() {
	var err error
	ragRecordsSyncedTotal, err = ragMeter.Int64Counter(
		"rag_records_synced_total",
		metric.WithDescription("Total number of RAG records synced"),
	)
	if err != nil {
		panic(err)
	}

	ragSyncErrorsTotal, err = ragMeter.Int64Counter(
		"rag_sync_errors_total",
		metric.WithDescription("Total number of RAG sync errors"),
	)
	if err != nil {
		panic(err)
	}
}

// RecordRAGSyncSuccess records a successful sync operation.
func RecordRAGSyncSuccess(ctx context.Context, count int64) {
	ragRecordsSyncedTotal.Add(ctx, count)
}

// RecordRAGSyncError records a failed sync operation.
func RecordRAGSyncError(ctx context.Context, count int64) {
	ragSyncErrorsTotal.Add(ctx, count)
}
