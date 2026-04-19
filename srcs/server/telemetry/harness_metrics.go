package telemetry

import (
	"context"

	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/metric"
)

var (
	HarnessExecutionDuration metric.Float64Histogram
	HarnessToolInvocations   metric.Int64Counter
	HarnessViolations        metric.Int64Counter
)

func initHarnessMetrics(m mockableMeter) []error {
	var errs []error
	var err error

	HarnessExecutionDuration, err = m.Float64Histogram(
		"ohc_harness_execution_duration_seconds",
		metric.WithDescription("Duration of Agent Harness execution"),
		metric.WithUnit("s"),
	)
	if err != nil {
		errs = append(errs, err)
	}

	HarnessToolInvocations, err = m.Int64Counter(
		"ohc_harness_tool_invocations_total",
		metric.WithDescription("Total tool invocations in the harness"),
	)
	if err != nil {
		errs = append(errs, err)
	}

	HarnessViolations, err = m.Int64Counter(
		"ohc_harness_violations_total",
		metric.WithDescription("Total violations in the harness"),
	)
	if err != nil {
		errs = append(errs, err)
	}

	return errs
}

// RecordHarnessExecutionDuration records the execution duration in the harness.
func RecordHarnessExecutionDuration(ctx context.Context, duration float64, agentID string) {
	if HarnessExecutionDuration != nil {
		HarnessExecutionDuration.Record(ctx, duration, metric.WithAttributes(
			attribute.String("agent_id", agentID),
		))
	}
}

// RecordHarnessToolInvocation increments the tool invocation counter in the harness.
func RecordHarnessToolInvocation(ctx context.Context, toolName string, agentID string) {
	if HarnessToolInvocations != nil {
		HarnessToolInvocations.Add(ctx, 1, metric.WithAttributes(
			attribute.String("tool_name", toolName),
			attribute.String("agent_id", agentID),
		))
	}
}

// RecordHarnessViolation increments the violation counter in the harness.
func RecordHarnessViolation(ctx context.Context, agentID string) {
	if HarnessViolations != nil {
		HarnessViolations.Add(ctx, 1, metric.WithAttributes(
			attribute.String("agent_id", agentID),
		))
	}
}
