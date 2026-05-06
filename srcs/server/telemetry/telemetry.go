package telemetry

import (
	"context"
	"log"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

var meter = otel.Meter("onehumancorp/telemetry")
var sqliteLockContentionCounter metric.Int64Counter
var sqliteRetryExhaustedCounter metric.Int64Counter
var sqliteThrottledRequestCounter metric.Int64Counter

func init() {
	var err error
	sqliteLockContentionCounter, err = meter.Int64Counter(
		"ohc_sqlite_lock_contention_total",
		metric.WithDescription("Total SQLite lock contentions"),
	)
	if err != nil {
		log.Printf("Failed to initialize sqliteLockContentionCounter: %v", err)
	}

	sqliteRetryExhaustedCounter, err = meter.Int64Counter(
		"sqliteRetryExhaustedCounter",
		metric.WithDescription("Total exhausted retries for SQLite"),
	)
	if err != nil {
		log.Printf("Failed to initialize sqliteRetryExhaustedCounter: %v", err)
	}

	sqliteThrottledRequestCounter, err = meter.Int64Counter(
		"sqliteThrottledRequestCounter",
		metric.WithDescription("Total throttled SQLite requests"),
	)
	if err != nil {
		log.Printf("Failed to initialize sqliteThrottledRequestCounter: %v", err)
	}
}

// RecordSQLiteLockContention logs SQLite lock contention and increments the metric
func RecordSQLiteLockContention(ctx context.Context, operation string) {
	if sqliteLockContentionCounter != nil {
		sqliteLockContentionCounter.Add(ctx, 1)
	}
}

// RecordSQLiteRetryExhausted increments the retry exhausted metric
func RecordSQLiteRetryExhausted(ctx context.Context, operation string) {
	if sqliteRetryExhaustedCounter != nil {
		sqliteRetryExhaustedCounter.Add(ctx, 1)
	}
}

// RecordSQLiteThrottledRequest increments the throttled request metric
func RecordSQLiteThrottledRequest(ctx context.Context, operation string) {
	if sqliteThrottledRequestCounter != nil {
		sqliteThrottledRequestCounter.Add(ctx, 1)
	}
}
