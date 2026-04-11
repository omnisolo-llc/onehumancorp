package telemetry

import (
	"context"

	"go.opentelemetry.io/otel/metric"
)

var (
	ragRecordsSyncedTotal metric.Int64Counter
	ragSyncErrorsTotal    metric.Int64Counter
)

func initRagSyncMetrics(meter mockableMeter) error {
	var err error

	ragRecordsSyncedTotal, err = meter.Int64Counter(
		"ohc.rag.records.synced.total",
		metric.WithDescription("Total number of RAG records synced to the cloud"),
	)
	if err != nil {
		return err
	}

	ragSyncErrorsTotal, err = meter.Int64Counter(
		"ohc.rag.sync.errors.total",
		metric.WithDescription("Total number of errors encountered during RAG sync"),
	)
	if err != nil {
		return err
	}

	return nil
}

// RecordRAGRecordsSynced increments the counter for successfully synced RAG records.
func RecordRAGRecordsSynced(ctx context.Context, count int64) {
	if ragRecordsSyncedTotal == nil {
		return
	}
	ragRecordsSyncedTotal.Add(ctx, count)
}

// RecordRAGSyncError increments the counter for RAG sync errors.
func RecordRAGSyncError(ctx context.Context) {
	if ragSyncErrorsTotal == nil {
		return
	}
	ragSyncErrorsTotal.Add(ctx, 1)
}
