package telemetry

import (
    "go.opentelemetry.io/otel/metric"
)

var (
    RAGRecordsSyncedTotal metric.Int64Counter
    RAGSyncErrorsTotal    metric.Int64Counter
)

func initRAGSyncMetrics(m mockableMeter) error {
    var err error
    RAGRecordsSyncedTotal, err = m.Int64Counter(
        "rag_records_synced_total",
        metric.WithDescription("Total number of RAG records successfully synced"),
    )
    if err != nil {
        return err
    }

    RAGSyncErrorsTotal, err = m.Int64Counter(
        "rag_sync_errors_total",
        metric.WithDescription("Total number of RAG sync errors"),
    )
    return err
}
