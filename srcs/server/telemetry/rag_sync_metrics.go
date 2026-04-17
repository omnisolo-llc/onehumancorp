package telemetry

import (
	"context"

	"go.opentelemetry.io/otel/attribute"
	"encoding/json"

	"go.opentelemetry.io/otel/metric"
)

var (
	ragRecordsSyncedTotal metric.Int64Counter
	ragSyncErrorsTotal    metric.Int64Counter
)

func initRAGSyncMetrics(m mockableMeter) error {
	var err error
	ragRecordsSyncedTotal, err = m.Int64Counter(
		"rag_records_synced_total",
		metric.WithDescription("Total number of RAG records successfully synced"),
	)
	if err != nil {
		return err
	}

	ragSyncErrorsTotal, err = m.Int64Counter(
		"rag_sync_errors_total",
		metric.WithDescription("Total number of RAG sync errors"),
	)
	return err
}

func RecordRAGRecordsSynced(ctx context.Context, count int64) {
	if BufferMetricFunc != nil {
		payloadMap := map[string]interface{}{
			"count": count,
		}
		payloadMap["env_mode"] = cachedEnvMode
		payloadBytes, _ := json.Marshal(RedactInterfacePII(payloadMap))
		_ = BufferMetricFunc(ctx, "rag_records_synced", string(payloadBytes))
	}
	if ragRecordsSyncedTotal != nil {
		ragRecordsSyncedTotal.Add(ctx, count, metric.WithAttributes(
			attribute.String("env_mode", cachedEnvMode),
		))
	}
}

func RecordRAGSyncError(ctx context.Context, errStr string) {
	if BufferMetricFunc != nil {
		payloadMap := map[string]interface{}{
			"error": errStr,
		}
		payloadMap["env_mode"] = cachedEnvMode
		payloadBytes, _ := json.Marshal(RedactInterfacePII(payloadMap))
		_ = BufferMetricFunc(ctx, "rag_sync_error", string(payloadBytes))
	}
	if ragSyncErrorsTotal != nil {
		ragSyncErrorsTotal.Add(ctx, 1, metric.WithAttributes(
			attribute.String("env_mode", cachedEnvMode),
		))
	}
}
