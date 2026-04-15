package telemetry

import (
	"context"
	"encoding/json"
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

func RecordRAGRecordsSynced(ctx context.Context, count int64) {
	if BufferMetricFunc != nil {
		payloadMap := map[string]interface{}{"count": count}
		redactedMap := RedactInterfacePII(payloadMap)
		payloadBytes, _ := json.Marshal(redactedMap)
		_ = BufferMetricFunc(ctx, "rag_records_synced", string(payloadBytes))
	}
	if RAGRecordsSyncedTotal != nil {
		RAGRecordsSyncedTotal.Add(ctx, count)
	}
}

func RecordRAGSyncError(ctx context.Context, errStr string) {
	if BufferMetricFunc != nil {
		payloadMap := map[string]interface{}{"error": errStr}
		redactedMap := RedactInterfacePII(payloadMap)
		payloadBytes, _ := json.Marshal(redactedMap)
		_ = BufferMetricFunc(ctx, "rag_sync_error", string(payloadBytes))
	}
	if RAGSyncErrorsTotal != nil {
		RAGSyncErrorsTotal.Add(ctx, 1)
	}
}
