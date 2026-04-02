package telemetry

import (
	"context"

	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/metric"
)

var (
	syncCompletedCounter metric.Int64Counter
	syncFailedCounter    metric.Int64Counter
)

func initSyncMetrics() error {
	var err error
	var errs []error

	if meter == nil {
		return nil
	}

	syncCompletedCounter, err = meter.Int64Counter(
		"sync_completed_count",
		metric.WithDescription("Total successful autodream syncs to cloud"),
	)
	if err != nil {
		errs = append(errs, err)
	}

	syncFailedCounter, err = meter.Int64Counter(
		"sync_failed_count",
		metric.WithDescription("Total failed autodream syncs to cloud"),
	)
	if err != nil {
		errs = append(errs, err)
	}

	if len(errs) > 0 {
		return errs[0]
	}
	return nil
}

func init() {
	// Let's add it to the Init function. Wait, actually I can just register it here but I should make sure it integrates nicely.
	// Actually, I can just create functions and if metric handles are nil, ignore.
}

// RecordSyncCompleted increments the sync_completed_count metric.
func RecordSyncCompleted(ctx context.Context, source string) {
	if syncCompletedCounter != nil {
		syncCompletedCounter.Add(ctx, 1, metric.WithAttributes(
			attribute.String("source", source),
		))
	}
}

// RecordSyncFailed increments the sync_failed_count metric.
func RecordSyncFailed(ctx context.Context, source string, reason string) {
	if syncFailedCounter != nil {
		syncFailedCounter.Add(ctx, 1, metric.WithAttributes(
			attribute.String("source", source),
			attribute.String("reason", reason),
		))
	}
}
