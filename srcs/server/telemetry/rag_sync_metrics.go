package telemetry

import (
	"go.opentelemetry.io/otel/metric"
)

import "context"

var (
	RAGRecordsSyncedTotal metric.Int64Counter
	RAGSyncErrorsTotal    metric.Int64Counter
	RAGEscalationCount    metric.Int64Counter
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
	if err != nil {
		return err
	}

	RAGEscalationCount, err = m.Int64Counter(
		"rag_escalation_count",
		metric.WithDescription("Total number of dynamic RAG escalations to the Cloud Swarm"),
	)
	return err
}

// RecordRAGEscalation increments the RAG escalation counter.
func RecordRAGEscalation(ctx context.Context) {
	if RAGEscalationCount != nil {
		RAGEscalationCount.Add(ctx, 1)
	}
}
