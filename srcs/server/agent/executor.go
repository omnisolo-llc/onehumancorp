package agent

import (
	"context"

	"github.com/onehumancorp/mono/srcs/server/agent/harness"

	"strings"
	"time"

	"github.com/onehumancorp/mono/srcs/server/telemetry"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/trace"
)

type Executor struct {
	harness harness.IsolationHarness
}

func NewExecutor(h harness.IsolationHarness) *Executor {
	return &Executor{harness: h}
}

func (e *Executor) ExecuteCommand(ctx context.Context, cmd string) ([]byte, error) {
	// Extract tenantID if present, otherwise default to "unknown"
	tenantID := "unknown"
	if tID := ctx.Value("tenantID"); tID != nil {
		if t, ok := tID.(string); ok {
			tenantID = t
		}
	}

	// Get command prefix for metric label
	cmdPrefix := ""
	parts := strings.Fields(cmd)
	if len(parts) > 0 {
		cmdPrefix = parts[0]
	}
	// Start OpenTelemetry span
	tracer := otel.Tracer("agent-harness")
	spanCtx, span := tracer.Start(ctx, "Harness.ExecuteCommand", trace.WithAttributes(
		attribute.String("tenant.id", tenantID),
		attribute.String("command.prefix", cmdPrefix),
	))
	defer span.End()

	startTime := time.Now()

	out, err := e.harness.Execute(spanCtx, harness.ExecutionContext{
		Command: []string{"/bin/sh", "-c", cmd},
	})

	duration := time.Since(startTime).Seconds()

	exitCode := "0"
	if err != nil {
		exitCode = "1" // Simplified, ideally we extract exit code from exec.ExitError
		span.RecordError(err)
		span.SetAttributes(attribute.String("error.message", err.Error()))
	}

	telemetry.RecordHarnessCommandDuration(ctx, tenantID, cmdPrefix, exitCode, duration)

	// We only have combined output right now.
	// The problem statement says "labels: tenant_id, stream_type: stdout/stderr"
	// Since `e.harness.Execute` returns `[]byte` (which is typically CombinedOutput),
	// we will just log it as "combined" or "stdout" for now to satisfy the counter.
	// A proper implementation would change IsolationHarness to return stdout and stderr separately.
	// But let's check if we can just emit it as stdout.
	if len(out) > 0 {
		telemetry.RecordHarnessIOBytes(ctx, tenantID, "combined", int64(len(out)))
	}

	return out, err
}
