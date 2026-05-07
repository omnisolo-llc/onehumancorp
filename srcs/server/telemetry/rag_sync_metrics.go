package telemetry

import (
	"context"
	"log"

	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/metric"
)

var (
	ragRecordsSyncedTotalCounter metric.Int64Counter
	ragSyncErrorsTotalCounter    metric.Int64Counter
)

func init() {
	var err error
	ragRecordsSyncedTotalCounter, err = meter.Int64Counter(
		"rag_records_synced_total",
		metric.WithDescription("Total number of RAG records synced"),
	)
	if err != nil {
		log.Printf("Failed to create ragRecordsSyncedTotalCounter: %v", err)
	}

	ragSyncErrorsTotalCounter, err = meter.Int64Counter(
		"rag_sync_errors_total",
		metric.WithDescription("Total number of RAG sync errors"),
	)
	if err != nil {
		log.Printf("Failed to create ragSyncErrorsTotalCounter: %v", err)
	}
}

// RecordRAGRecordsSynced increments the counter for RAG records synced.
func RecordRAGRecordsSynced(ctx context.Context, count int, status string) error {
	if ragRecordsSyncedTotalCounter != nil {
		opts := metric.WithAttributes(
			attribute.String("status", status),
			getDeploymentModeAttribute(),
		)
		ragRecordsSyncedTotalCounter.Add(ctx, int64(count), opts)
	}

	bufferMetricHelper(ctx, "rag_records_synced_total", float64(count), map[string]interface{}{
		"status": status,
		"deployment_mode": getDeploymentModeAttribute().Value.AsString(),
	})

	return nil
}

// RecordRAGSyncError increments the counter for RAG sync errors.
func RecordRAGSyncError(ctx context.Context, errorType string) error {
	if ragSyncErrorsTotalCounter != nil {
		opts := metric.WithAttributes(
			attribute.String("error_type", errorType),
			getDeploymentModeAttribute(),
		)
		ragSyncErrorsTotalCounter.Add(ctx, 1, opts)
	}

	bufferMetricHelper(ctx, "rag_sync_errors_total", 1, map[string]interface{}{
		"error_type": errorType,
		"deployment_mode": getDeploymentModeAttribute().Value.AsString(),
	})

	return nil
}
