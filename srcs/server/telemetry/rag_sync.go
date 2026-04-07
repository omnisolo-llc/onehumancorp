package telemetry

import (
	"go.opentelemetry.io/otel/metric"
)

var (
	// RagRecordsSyncedTotal tracks the total number of RAG records synced to the cloud.
	RagRecordsSyncedTotal metric.Int64Counter
	// RagSyncErrorsTotal tracks the total number of RAG sync errors.
	RagSyncErrorsTotal metric.Int64Counter
)

func initRagSyncMetrics(meter metric.Meter) {
	var err error
	RagRecordsSyncedTotal, err = meter.Int64Counter("rag_records_synced_total", metric.WithDescription("Total number of RAG records successfully synced to the cloud"))
	if err != nil {
		panic(err)
	}

	RagSyncErrorsTotal, err = meter.Int64Counter("rag_sync_errors_total", metric.WithDescription("Total number of errors encountered during RAG sync"))
	if err != nil {
		panic(err)
	}
}
