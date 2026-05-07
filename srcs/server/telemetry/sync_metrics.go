package telemetry

import (
	"context"
	"log"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/metric"
)

var (
	syncMeter                   = otel.Meter("hybrid_sync")
	syncLatencyHistogram        metric.Float64Histogram
	syncDaemonErrorTotal        metric.Int64Counter
	syncDaemonBatchSizeCounter  metric.Int64Counter
	syncPayloadSizeCounter      metric.Int64Counter
	syncPayloadSizeCount        metric.Int64Counter
	escalationCounter           metric.Int64Counter
)

func init() {
	var err error

	syncLatencyHistogram, err = syncMeter.Float64Histogram(
		"sync_latency_ms",
		metric.WithDescription("Latency of sync operations in milliseconds"),
	)
	if err != nil {
		log.Printf("Failed to create syncLatencyHistogram: %v", err)
	}

	syncDaemonErrorTotal, err = syncMeter.Int64Counter(
		"sync_daemon_error_total",
		metric.WithDescription("Total number of sync daemon errors"),
	)
	if err != nil {
		log.Printf("Failed to create syncDaemonErrorTotal: %v", err)
	}

	syncDaemonBatchSizeCounter, err = syncMeter.Int64Counter(
		"sync_daemon_batch_size",
		metric.WithDescription("Size of batches processed by sync daemon"),
	)
	if err != nil {
		log.Printf("Failed to create syncDaemonBatchSizeCounter: %v", err)
	}

	syncPayloadSizeCounter, err = syncMeter.Int64Counter(
		"sync_payload_size_bytes_sum",
		metric.WithDescription("Total size of synced payloads in bytes"),
	)
	if err != nil {
		log.Printf("Failed to create syncPayloadSizeCounter: %v", err)
	}

	syncPayloadSizeCount, err = syncMeter.Int64Counter(
		"sync_payload_size_bytes_count",
		metric.WithDescription("Number of synced payloads recorded"),
	)
	if err != nil {
		log.Printf("Failed to create syncPayloadSizeCount: %v", err)
	}

	escalationCounter, err = syncMeter.Int64Counter(
		"ohc_sync_escalations_count",
		metric.WithDescription("Total count of escalations to the cloud"),
	)
	if err != nil {
		log.Printf("Failed to create escalationCounter: %v", err)
	}
}

// RecordSyncLatency records the latency of a sync operation.
func RecordSyncLatency(ctx context.Context, durationMs float64, mode string) error {
	if syncLatencyHistogram != nil {
		opts := metric.WithAttributes(
			attribute.String("mode", mode),
		)
		syncLatencyHistogram.Record(ctx, durationMs, opts)
	}
	return nil
}

// RecordSyncDaemonError increments the sync daemon error counter.
func RecordSyncDaemonError(ctx context.Context, mode string, errorType string) error {
	if syncDaemonErrorTotal != nil {
		opts := metric.WithAttributes(
			attribute.String("mode", mode),
			attribute.String("error", errorType),
		)
		syncDaemonErrorTotal.Add(ctx, 1, opts)
	}
	return nil
}

// RecordSyncDaemonBatchSize records the size of a sync batch.
func RecordSyncDaemonBatchSize(ctx context.Context, size int, mode string) error {
	if syncDaemonBatchSizeCounter != nil {
		opts := metric.WithAttributes(
			attribute.String("mode", mode),
		)
		syncDaemonBatchSizeCounter.Add(ctx, int64(size), opts)
	}
	return nil
}

// RecordSyncPayloadSize records the size of a payload.
func RecordSyncPayloadSize(ctx context.Context, size int, mode string) error {
	if syncPayloadSizeCounter != nil && syncPayloadSizeCount != nil {
		opts := metric.WithAttributes(
			attribute.String("mode", mode),
		)
		syncPayloadSizeCounter.Add(ctx, int64(size), opts)
		syncPayloadSizeCount.Add(ctx, 1, opts)
	}
	return nil
}

// RecordSyncEscalation increments the escalation counter.
func RecordSyncEscalation(ctx context.Context, mode string) error {
	if escalationCounter != nil {
		opts := metric.WithAttributes(
			attribute.String("mode", mode),
		)
		escalationCounter.Add(ctx, 1, opts)
	}
	return nil
}
