package telemetry

import (
	"context"

	"go.opentelemetry.io/otel/metric"
)

var (
	RAGRecordsSyncedTotal metric.Int64Counter
	RAGSyncErrorsTotal    metric.Int64Counter
)

func init() {
	if meter != nil {
		var err error
		RAGRecordsSyncedTotal, err = meter.Int64Counter(
			"ohc.hybrid.rag.records_synced_total",
			metric.WithDescription("Total number of RAG records synced to cloud"),
		)
		if err != nil {
			panic(err)
		}

		RAGSyncErrorsTotal, err = meter.Int64Counter(
			"ohc.hybrid.rag.sync_errors_total",
			metric.WithDescription("Total number of RAG sync errors"),
		)
		if err != nil {
			panic(err)
		}
	}
}

// RecordRAGRecordsSynced increments the global counter for successful RAG syncs.
func RecordRAGRecordsSynced(ctx context.Context, count int64) {
	if RAGRecordsSyncedTotal != nil {
		RAGRecordsSyncedTotal.Add(ctx, count)
	}
}

// RecordRAGSyncError increments the global counter for RAG sync errors.
func RecordRAGSyncError(ctx context.Context) {
	if RAGSyncErrorsTotal != nil {
		RAGSyncErrorsTotal.Add(ctx, 1)
	}
}
