package telemetry

import (
	"context"
	"testing"
)

func TestRecordSandboxViolation(t *testing.T) {
	m := &mockMeter{}
	err := initSandboxViolationMetrics(m)
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}

	ctx := context.Background()
	RecordSandboxViolation(ctx, "fs_read", "agent-1", "/etc/passwd")

	if SandboxViolationTotal == nil {
		t.Fatalf("Expected SandboxViolationTotal to be initialized")
	}
}
