package telemetry

import (
	"context"

	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/metric"
)

var (
	ragRecordsSyncedTotal metric.Int64Counter
	ragSyncErrorsTotal    metric.Int64Counter
)

func initRAGSyncMetrics(m mockableMeter) []error {
	var errs []error
	var err error

	ragRecordsSyncedTotal, err = m.Int64Counter(
		"rag_records_synced_total",
		metric.WithDescription("Total number of RAG records synced"),
	)
	if err != nil {
		errs = append(errs, err)
	}

	ragSyncErrorsTotal, err = m.Int64Counter(
		"rag_sync_errors_total",
		metric.WithDescription("Total number of RAG sync errors"),
	)
	if err != nil {
		errs = append(errs, err)
	}

	return errs
}

// RecordRAGRecordsSynced increments the counter for successfully synced RAG records.
func RecordRAGRecordsSynced(ctx context.Context, count int64) {
	if ragRecordsSyncedTotal == nil {
		return
	}
	ragRecordsSyncedTotal.Add(ctx, count)
}

// RecordRAGSyncError increments the counter for RAG sync errors.
func RecordRAGSyncError(ctx context.Context, errType string) {
	if ragSyncErrorsTotal == nil {
		return
	}
	ragSyncErrorsTotal.Add(ctx, 1, metric.WithAttributes(
		attribute.String("error_type", errType),
	))
}
