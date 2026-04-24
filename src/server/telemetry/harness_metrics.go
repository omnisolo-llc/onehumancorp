package telemetry

import (
	"context"
	"go.opentelemetry.io/otel/metric"
)

var (
	HarnessExecutionDuration    metric.Float64Histogram
	HarnessToolInvocationsTotal metric.Int64Counter
	HarnessViolationsTotal      metric.Int64Counter
)

func initHarnessMetrics(m mockableMeter) error {
	var err error
	HarnessExecutionDuration, err = m.Float64Histogram(
		"harness_execution_duration_seconds",
		metric.WithDescription("Latency of Harness execution in seconds"),
	)
	if err != nil {
		return err
	}

	HarnessToolInvocationsTotal, err = m.Int64Counter(
		"harness_tool_invocations_total",
		metric.WithDescription("Total number of Harness tool invocations"),
	)
	if err != nil {
		return err
	}

	HarnessViolationsTotal, err = m.Int64Counter(
		"harness_violations_total",
		metric.WithDescription("Total number of Harness execution violations"),
	)
	return err
}

func RecordHarnessExecutionDuration(ctx context.Context, duration float64) {
	if HarnessExecutionDuration != nil {
		HarnessExecutionDuration.Record(ctx, duration)
	}
}

func RecordHarnessToolInvocation(ctx context.Context) {
	if HarnessToolInvocationsTotal != nil {
		HarnessToolInvocationsTotal.Add(ctx, 1)
	}
}

func RecordHarnessViolation(ctx context.Context) {
	if HarnessViolationsTotal != nil {
		HarnessViolationsTotal.Add(ctx, 1)
	}
}
