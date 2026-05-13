package telemetry

import (
	"context"
	"log"

	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/metric"
)

var (
	queueLengthGauge metric.Int64UpDownCounter
)

func init() {
	var err error
	// meter is defined in harness_metrics.go within the same package.
	queueLengthGauge, err = meter.Int64UpDownCounter(
		"queue_length",
		metric.WithDescription("Current number of jobs in the queue"),
	)
	if err != nil {
		log.Printf("Failed to create queueLengthGauge: %v", err)
	}
}

// RecordQueueLength sets or updates the current queue length.
func RecordQueueLength(ctx context.Context, length int64, queueType string) error {
	if queueLengthGauge != nil {
		opts := metric.WithAttributes(
			attribute.String("queue_type", queueType),
		)
		queueLengthGauge.Add(ctx, length, opts)
	}
	return nil
}
