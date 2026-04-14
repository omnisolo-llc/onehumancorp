package telemetry

import (
	"go.opentelemetry.io/otel/metric"
)

var (
	RAGEscalationCount metric.Int64Counter
)

func initRAGEscalationMetrics(m mockableMeter) error {
	var err error
	RAGEscalationCount, err = m.Int64Counter(
		"rag_escalation_count",
		metric.WithDescription("Total number of RAG queries escalated to cloud"),
	)
	return err
}
