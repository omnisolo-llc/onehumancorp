package telemetry

import (
	"context"
	"log"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/metric"
)

var (
	meter                   = otel.Meter("harness")
	executionDurationHistogram metric.Float64Histogram
	toolInvocationsCounter     metric.Int64Counter
	violationsCounter          metric.Int64Counter
)

func init() {
	var err error
	executionDurationHistogram, err = meter.Float64Histogram(
		"harness_execution_duration_seconds",
		metric.WithDescription("Duration of harness execution in seconds"),
	)
	if err != nil {
		log.Printf("Failed to create executionDurationHistogram: %v", err)
	}

	toolInvocationsCounter, err = meter.Int64Counter(
		"harness_tool_invocations_total",
		metric.WithDescription("Total number of tool invocations"),
	)
	if err != nil {
		log.Printf("Failed to create toolInvocationsCounter: %v", err)
	}

	violationsCounter, err = meter.Int64Counter(
		"harness_violations_total",
		metric.WithDescription("Total number of harness policy violations"),
	)
	if err != nil {
		log.Printf("Failed to create violationsCounter: %v", err)
	}
}

// RecordHarnessExecutionDuration records the duration of a harness execution.
func RecordHarnessExecutionDuration(ctx context.Context, durationSecs float64) error {
	if InterceptMetric("harness_execution_duration_seconds", durationSecs, nil) {
		return nil
	}
	if executionDurationHistogram != nil {
		executionDurationHistogram.Record(ctx, durationSecs)
	}
	return nil
}

// RecordHarnessToolInvocation increments the counter for a specific tool invocation.
func RecordHarnessToolInvocation(ctx context.Context, toolName string) error {
	if InterceptMetric("harness_tool_invocations_total", 1, map[string]string{"tool": toolName}) {
		return nil
	}
	if toolInvocationsCounter != nil {
		opts := metric.WithAttributes(
			attribute.String("tool", toolName),
		)
		toolInvocationsCounter.Add(ctx, 1, opts)
	}
	return nil
}

// RecordHarnessViolation increments the counter for a harness violation (e.g. timeout, memory limit).
func RecordHarnessViolation(ctx context.Context, violationType string) error {
	if InterceptMetric("harness_violations_total", 1, map[string]string{"violation_type": violationType}) {
		return nil
	}
	if violationsCounter != nil {
		opts := metric.WithAttributes(
			attribute.String("violation_type", violationType),
		)
		violationsCounter.Add(ctx, 1, opts)
	}
	return nil
}
