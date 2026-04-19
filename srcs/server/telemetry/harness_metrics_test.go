package telemetry

import (
	"context"
	"testing"
)

func TestHarnessMetrics(t *testing.T) {
	cleanup, _ := InitTelemetry()
	defer cleanup()

	ctx := context.Background()

	// Ensure metrics don't panic when used
	RecordHarnessExecutionDuration(ctx, 1.5, "agent-1")
	RecordHarnessToolInvocation(ctx, "ls", "agent-1")
	RecordHarnessViolation(ctx, "agent-1")
}
