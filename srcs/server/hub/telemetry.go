package hub

import (
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

var (
	meter = otel.Meter("github.com/onehumancorp/ohc/srcs/server/hub")

	RAGRecordsSyncedTotal metric.Int64Counter
	RAGSyncErrorsTotal    metric.Int64Counter
)

func init() {
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
		metric.WithDescription("Total number of errors during RAG sync"),
	)
	if err != nil {
		panic(err)
	}
}
