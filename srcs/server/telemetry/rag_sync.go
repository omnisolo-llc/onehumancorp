package telemetry

import (
	"go.opentelemetry.io/otel/metric"
)

var (
	RAGRecordsSyncedTotal metric.Int64Counter
	RAGSyncErrorsTotal    metric.Int64Counter
)

func InitRAGSyncMetrics(m metric.Meter) error {
	var err error
	RAGRecordsSyncedTotal, err = m.Int64Counter("rag_records_synced_total", metric.WithDescription("Total number of RAG records synced to cloud"))
	if err != nil {
		return err
	}
	RAGSyncErrorsTotal, err = m.Int64Counter("rag_sync_errors_total", metric.WithDescription("Total number of RAG sync errors"))
	if err != nil {
		return err
	}
	return nil
}
