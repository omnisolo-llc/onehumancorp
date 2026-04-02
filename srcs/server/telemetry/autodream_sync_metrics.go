package telemetry

import (
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

var (
	SyncCompletedCount metric.Int64Counter
	SyncFailedCount    metric.Int64Counter
)

func init() {
	meter := otel.Meter("github.com/onehumancorp/mono/ohc")

	var err error
	SyncCompletedCount, err = meter.Int64Counter(
		"sync_completed_count",
		metric.WithDescription("Total number of successful AutoDream cloud syncs"),
	)
	if err != nil {
		panic(err)
	}

	SyncFailedCount, err = meter.Int64Counter(
		"sync_failed_count",
		metric.WithDescription("Total number of failed AutoDream cloud syncs"),
	)
	if err != nil {
		panic(err)
	}
}
