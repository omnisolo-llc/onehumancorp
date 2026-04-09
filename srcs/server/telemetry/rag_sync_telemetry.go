package telemetry

import (
	"context"

	"go.opentelemetry.io/otel/metric"
)

var (
	ragRecordsSyncedTotal metric.Int64Counter
	ragSyncErrorsTotal    metric.Int64Counter
)

func initRAGSyncMetrics(m mockableMeter) error {
	if m == nil {
		return nil
	}

	var err error

	ragRecordsSyncedTotal, err = m.Int64Counter(
		"rag_records_synced_total",
		metric.WithDescription("Total number of RAG records synced to the cloud"),
	)
	if err != nil {
		return err
	}

	ragSyncErrorsTotal, err = m.Int64Counter(
		"rag_sync_errors_total",
		metric.WithDescription("Total number of errors encountered during RAG sync"),
	)
	if err != nil {
		return err
	}

	return nil
}

// RecordRagRecordSynced increments the counter for a successfully synced RAG record.
func RecordRagRecordSynced(ctx context.Context) {
	if ragRecordsSyncedTotal != nil {
		ragRecordsSyncedTotal.Add(ctx, 1)
	}
}

// RecordRagSyncError increments the counter for a RAG sync error.
func RecordRagSyncError(ctx context.Context) {
	if ragSyncErrorsTotal != nil {
		ragSyncErrorsTotal.Add(ctx, 1)
	}
}
