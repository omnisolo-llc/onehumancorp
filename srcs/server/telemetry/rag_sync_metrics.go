package telemetry

import (
	"context"
	"encoding/json"

	"go.opentelemetry.io/otel/metric"
)

var (
	rAGRecordsSyncedTotal metric.Int64Counter
	rAGSyncErrorsTotal    metric.Int64Counter
)

func initRAGSyncMetrics(m mockableMeter) error {
	var err error
	rAGRecordsSyncedTotal, err = m.Int64Counter(
		"rag_records_synced_total",
		metric.WithDescription("Total number of RAG records successfully synced"),
	)
	if err != nil {
		return err
	}

	rAGSyncErrorsTotal, err = m.Int64Counter(
		"rag_sync_errors_total",
		metric.WithDescription("Total number of RAG sync errors"),
	)
	return err
}

func RecordRAGRecordsSynced(ctx context.Context, count int64) {
	if rAGRecordsSyncedTotal != nil {
		rAGRecordsSyncedTotal.Add(ctx, count)
	}
	if BufferMetricFunc != nil {
		payloadMap := map[string]interface{}{
			"count": count,
		}
		redactedPayload := RedactInterfacePII(payloadMap)
		payloadBytes, _ := json.Marshal(redactedPayload)
		_ = BufferMetricFunc(ctx, "rag_records_synced_total", string(payloadBytes))
	}
}

func RecordRAGSyncError(ctx context.Context, errStr string) {
	if rAGSyncErrorsTotal != nil {
		rAGSyncErrorsTotal.Add(ctx, 1)
	}
	if BufferMetricFunc != nil {
		payloadMap := map[string]interface{}{
			"error": errStr,
		}
		redactedPayload := RedactInterfacePII(payloadMap)
		payloadBytes, _ := json.Marshal(redactedPayload)
		_ = BufferMetricFunc(ctx, "rag_sync_errors_total", string(payloadBytes))
	}
}
