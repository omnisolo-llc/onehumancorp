package telemetry

import (
	"context"
	"log"
	"os"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/metric"
)


func getDeploymentModeAttribute() attribute.KeyValue {
	isStandalone := os.Getenv("OHC_STANDALONE") == "true" || os.Getenv("STANDALONE_MODE") == "true"
	mode := "cloud"
	if isStandalone {
		mode = "standalone"
	}
	return attribute.String("deployment_mode", mode)
}
func isTelemetryEnabled() bool {
	// In standalone mode, do not sync telemetry to cloud unless explicitly enabled
	isStandalone := os.Getenv("OHC_STANDALONE") == "true" || os.Getenv("STANDALONE_MODE") == "true"
	if isStandalone {
		return os.Getenv("OHC_TELEMETRY_ENABLED") == "true"
	}
	return true
}

var (
	meter                   = otel.Meter("harness")
	executionDurationHistogram metric.Float64Histogram
	toolInvocationsCounter     metric.Int64Counter
	violationsCounter          metric.Int64Counter
	mcpToolCallsCounter        metric.Int64Counter
	harnessInitLatencyHistogram metric.Float64Histogram
	harnessDbIoLatencyHistogram metric.Float64Histogram
	bubblewrapSpawnTotalCounter metric.Int64Counter
	bubblewrapExecutionLatencyHistogram metric.Float64Histogram
	bubblewrapViolationTotalCounter metric.Int64Counter
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

	mcpToolCallsCounter, err = meter.Int64Counter(
		"ohc_mcp_tool_calls_total",
		metric.WithDescription("Total number of MCP tool calls"),
	)
	if err != nil {
		log.Printf("Failed to create mcpToolCallsCounter: %v", err)
	}

	harnessInitLatencyHistogram, err = meter.Float64Histogram(
		"harness_init_latency_seconds",
		metric.WithDescription("Duration of harness initialization in seconds"),
	)
	if err != nil {
		log.Printf("Failed to create harnessInitLatencyHistogram: %v", err)
	}

	harnessDbIoLatencyHistogram, err = meter.Float64Histogram(
		"harness_db_io_latency_seconds",
		metric.WithDescription("Duration of harness database I/O in seconds"),
	)
	if err != nil {
		log.Printf("Failed to create harnessDbIoLatencyHistogram: %v", err)
	}

	bubblewrapSpawnTotalCounter, err = meter.Int64Counter(
		"bubblewrap_spawn_total",
		metric.WithDescription("Total number of bubblewrap spawns"),
	)
	if err != nil {
		log.Printf("Failed to create bubblewrapSpawnTotalCounter: %v", err)
	}

	bubblewrapExecutionLatencyHistogram, err = meter.Float64Histogram(
		"bubblewrap_execution_latency_seconds",
		metric.WithDescription("Latency of bubblewrap execution in seconds"),
	)
	if err != nil {
		log.Printf("Failed to create bubblewrapExecutionLatencyHistogram: %v", err)
	}

	bubblewrapViolationTotalCounter, err = meter.Int64Counter(
		"bubblewrap_violation_total",
		metric.WithDescription("Total number of bubblewrap sandbox violations"),
	)
	if err != nil {
		log.Printf("Failed to create bubblewrapViolationTotalCounter: %v", err)
	}
}

// RecordHarnessExecutionDuration records the duration of a harness execution.
func RecordHarnessExecutionDuration(ctx context.Context, durationSecs float64) error {
	if !isTelemetryEnabled() {
		return nil
	}
	if executionDurationHistogram != nil {
		opts := metric.WithAttributes(getDeploymentModeAttribute())
		executionDurationHistogram.Record(ctx, durationSecs, opts)
	}
	return nil
}

// RecordMCPToolCall increments the counter for an MCP tool call.
func RecordMCPToolCall(ctx context.Context, toolName string) error {
	if mcpToolCallsCounter != nil {
		opts := metric.WithAttributes(
			attribute.String("tool", toolName),
			getDeploymentModeAttribute(),
		)
		mcpToolCallsCounter.Add(ctx, 1, opts)
	}
	return nil
}

// RecordHarnessToolInvocation increments the counter for a specific tool invocation.
func RecordHarnessToolInvocation(ctx context.Context, toolName string) error {
	if !isTelemetryEnabled() {
		return nil
	}
	if toolInvocationsCounter != nil {
		opts := metric.WithAttributes(
			attribute.String("tool", toolName),
			getDeploymentModeAttribute(),
		)
		toolInvocationsCounter.Add(ctx, 1, opts)
	}
	return nil
}

// RecordHarnessViolation increments the counter for a harness violation (e.g. timeout, memory limit).
func RecordHarnessViolation(ctx context.Context, violationType string) error {
	if !isTelemetryEnabled() {
		return nil
	}
	if violationsCounter != nil {
		opts := metric.WithAttributes(
			attribute.String("violation_type", violationType),
			getDeploymentModeAttribute(),
		)
		violationsCounter.Add(ctx, 1, opts)
	}
	return nil
}

func RecordHarnessInitLatency(ctx context.Context, durationSecs float64) error {
	if !isTelemetryEnabled() {
		return nil
	}
	if harnessInitLatencyHistogram != nil {
		opts := metric.WithAttributes(getDeploymentModeAttribute())
		harnessInitLatencyHistogram.Record(ctx, durationSecs, opts)
	}
	return nil
}

func RecordHarnessDbIOLatency(ctx context.Context, durationSecs float64, operation string) error {
	if !isTelemetryEnabled() {
		return nil
	}
	if harnessDbIoLatencyHistogram != nil {
		opts := metric.WithAttributes(
			attribute.String("operation", operation),
			getDeploymentModeAttribute(),
		)
		harnessDbIoLatencyHistogram.Record(ctx, durationSecs, opts)
	}
	return nil
}

// RecordBubblewrapSpawn increments the counter for a bubblewrap spawn.
func RecordBubblewrapSpawn(ctx context.Context) error {
	if !isTelemetryEnabled() {
		return nil
	}
	if bubblewrapSpawnTotalCounter != nil {
		opts := metric.WithAttributes(getDeploymentModeAttribute())
		bubblewrapSpawnTotalCounter.Add(ctx, 1, opts)
	}
	return nil
}

// RecordBubblewrapExecutionLatency records the latency of a bubblewrap execution.
func RecordBubblewrapExecutionLatency(ctx context.Context, durationSecs float64) error {
	if !isTelemetryEnabled() {
		return nil
	}
	if bubblewrapExecutionLatencyHistogram != nil {
		opts := metric.WithAttributes(getDeploymentModeAttribute())
		bubblewrapExecutionLatencyHistogram.Record(ctx, durationSecs, opts)
	}
	return nil
}

// RecordBubblewrapViolation increments the counter for a bubblewrap sandbox violation.
func RecordBubblewrapViolation(ctx context.Context, violationType string) error {
	if !isTelemetryEnabled() {
		return nil
	}
	if bubblewrapViolationTotalCounter != nil {
		opts := metric.WithAttributes(
			attribute.String("violation_type", violationType),
			getDeploymentModeAttribute(),
		)
		bubblewrapViolationTotalCounter.Add(ctx, 1, opts)
	}
	return nil
}
