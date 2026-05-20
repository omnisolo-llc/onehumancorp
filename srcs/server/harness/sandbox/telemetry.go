package sandbox

import (
	"context"
	"log"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/metric"
)

var (
	meter                = otel.Meter("harness.sandbox")
	violationsCounter    metric.Int64Counter
	executionsCounter metric.Int64Counter
)

func init() {
	var err error
	violationsCounter, err = meter.Int64Counter("harness.sandbox.violations",
		metric.WithDescription("Total number of sandbox violations"),
	)
	if err != nil {
		log.Printf("Failed to initialize telemetry violationsCounter: %v", err)
	}

	executionsCounter, err = meter.Int64Counter("harness.sandbox.wrapped_executions",
		metric.WithDescription("Total number of wrapped sandbox executions"),
	)
	if err != nil {
		log.Printf("Failed to initialize telemetry executionsCounter: %v", err)
	}
}

func RecordViolation(ctx context.Context, command string, reason string) {
	if violationsCounter != nil {
		violationsCounter.Add(ctx, 1,
			metric.WithAttributes(
				attribute.String("command", command),
				attribute.String("reason", reason),
			),
		)
	}
}

func RecordExecution(ctx context.Context, command string) {
	if executionsCounter != nil {
		executionsCounter.Add(ctx, 1,
			metric.WithAttributes(
				attribute.String("command", command),
			),
		)
	}
}
