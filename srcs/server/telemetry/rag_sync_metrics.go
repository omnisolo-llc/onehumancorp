package telemetry

import (
	"context"
	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/metric"
)

var (
	ragRecordsSyncedCounter metric.Int64Counter
	ragSyncErrorsCounter    metric.Int64Counter
)

func initRAGSyncMetrics(m mockableMeter) error {
	var err error
	var errs []error

	ragRecordsSyncedCounter, err = m.Int64Counter(
		"rag_records_synced_total",
		metric.WithDescription("Total number of RAG records successfully synced"),
	)
	if err != nil {
		errs = append(errs, err)
	}

	ragSyncErrorsCounter, err = m.Int64Counter(
		"rag_sync_errors_total",
		metric.WithDescription("Total number of RAG sync errors"),
	)
	if err != nil {
		errs = append(errs, err)
	}

	if len(errs) > 0 {
		return errs[0]
	}
	return nil
}

// RecordRAGSyncSuccess records a successful RAG sync operation.
func RecordRAGSyncSuccess(ctx context.Context, count int, syncType string) {
	attrs := metric.WithAttributes(attribute.String("type", syncType))

	if ragRecordsSyncedCounter != nil {
		ragRecordsSyncedCounter.Add(ctx, int64(count), attrs)
	}
}

// RecordRAGSyncError records a failed RAG sync operation.
func RecordRAGSyncError(ctx context.Context, syncType string) {
	attrs := metric.WithAttributes(attribute.String("type", syncType))

	if ragSyncErrorsCounter != nil {
		ragSyncErrorsCounter.Add(ctx, 1, attrs)
	}
}
