package telemetry

import (
	"context"

	"go.opentelemetry.io/otel/metric"
	"go.opentelemetry.io/otel/attribute"
)

var (
	RAGRecordsSyncedTotal metric.Int64Counter
	RAGSyncErrorsTotal    metric.Int64Counter
)

// InitRAGSyncMetrics initializes metrics for RAG Sync. Note: must be explicitly assigned during global init.
func InitRAGSyncMetrics(m mockableMeter) {
	if m == nil {
		return
	}
	RAGRecordsSyncedTotal, _ = m.Int64Counter(
		"ohc.rag_sync.records_synced_total",
		metric.WithDescription("Total number of RAG records successfully synced"),
	)
	RAGSyncErrorsTotal, _ = m.Int64Counter(
		"ohc.rag_sync.errors_total",
		metric.WithDescription("Total number of errors encountered during RAG sync"),
	)
}

func RecordRAGRecordSynced(ctx context.Context, amount int64, deploymentMode string) {
	if RAGRecordsSyncedTotal == nil {
		return
	}
	RAGRecordsSyncedTotal.Add(ctx, amount, metric.WithAttributes(
		attribute.String("deployment_mode", deploymentMode),
	))
}

func RecordRAGSyncError(ctx context.Context, amount int64, deploymentMode string) {
	if RAGSyncErrorsTotal == nil {
		return
	}
	RAGSyncErrorsTotal.Add(ctx, amount, metric.WithAttributes(
		attribute.String("deployment_mode", deploymentMode),
	))
}
