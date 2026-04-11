package telemetry

import (
	"context"
	"os"

	"go.opentelemetry.io/otel/metric"
)

var (
	ragRecordsSyncedCounter metric.Int64Counter
	ragSyncErrorsCounter    metric.Int64Counter
)

func initRAGSyncMetrics(m metric.Meter) error {
	var err error

	ragRecordsSyncedCounter, err = m.Int64Counter(
		"rag_records_synced_total",
		metric.WithDescription("Total number of RAG records synced successfully"),
	)
	if err != nil {
		return err
	}

	ragSyncErrorsCounter, err = m.Int64Counter(
		"rag_sync_errors_total",
		metric.WithDescription("Total number of RAG sync errors"),
	)
	if err != nil {
		return err
	}

	return nil
}

// RecordRAGSyncSuccess increments the successful RAG sync counter
func RecordRAGSyncSuccess(ctx context.Context, count int64) {
	if os.Getenv("OHC_STANDALONE") == "true" && os.Getenv("OHC_TELEMETRY_ENABLED") != "true" {
		return
	}
	if ragRecordsSyncedCounter != nil {
		ragRecordsSyncedCounter.Add(ctx, count)
	}
}

// RecordRAGSyncError increments the RAG sync error counter
func RecordRAGSyncError(ctx context.Context) {
	if os.Getenv("OHC_STANDALONE") == "true" && os.Getenv("OHC_TELEMETRY_ENABLED") != "true" {
		return
	}
	if ragSyncErrorsCounter != nil {
		ragSyncErrorsCounter.Add(ctx, 1)
	}
}
