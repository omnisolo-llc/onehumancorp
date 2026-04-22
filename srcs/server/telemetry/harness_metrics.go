package telemetry

import (
	"context"

	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/metric"
)

var (
	// HarnessExecutionDurationSeconds tracks the execution duration of sub-agents in the harness.
	HarnessExecutionDurationSeconds metric.Float64Histogram
	// HarnessToolInvocationsTotal tracks specific tool invocations within the sandbox.
	HarnessToolInvocationsTotal metric.Int64Counter
	// HarnessViolationsTotal tracks sandbox violations in the harness execution context.
	HarnessViolationsTotal metric.Int64Counter
)

func initHarnessMetrics(m mockableMeter) error {
	var err error

	HarnessExecutionDurationSeconds, err = m.Float64Histogram(
		"harness_execution_duration_seconds",
		metric.WithDescription("Duration of harness execution in seconds"),
		metric.WithUnit("s"),
	)
	if err != nil {
		return err
	}

	HarnessToolInvocationsTotal, err = m.Int64Counter(
		"harness_tool_invocations_total",
		metric.WithDescription("Total number of tool invocations within the harness"),
	)
	if err != nil {
		return err
	}

	HarnessViolationsTotal, err = m.Int64Counter(
		"harness_violations_total",
		metric.WithDescription("Total number of sandbox violations within the harness execution context"),
	)
	if err != nil {
		return err
	}

	return nil
}

// RecordHarnessExecutionDuration records the execution duration.
func RecordHarnessExecutionDuration(ctx context.Context, duration float64) {
	if HarnessExecutionDurationSeconds != nil {
		HarnessExecutionDurationSeconds.Record(ctx, duration)
	}
}

// RecordHarnessToolInvocation increments the counter for a specific tool invocation.
func RecordHarnessToolInvocation(ctx context.Context, tool string) {
	if HarnessToolInvocationsTotal != nil {
		HarnessToolInvocationsTotal.Add(ctx, 1, metric.WithAttributes(
			attribute.String("tool", tool),
		))
	}
}

// RecordHarnessViolation increments the counter for a specific sandbox violation.
func RecordHarnessViolation(ctx context.Context, violationType string) {
	if HarnessViolationsTotal != nil {
		HarnessViolationsTotal.Add(ctx, 1, metric.WithAttributes(
			attribute.String("type", violationType),
		))
	}
}
