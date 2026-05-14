package telemetry

import (
	"context"
	"testing"
)

func TestRecordHarnessExecutionDuration(t *testing.T) {
	ctx := context.Background()
	err := RecordHarnessExecutionDuration(ctx, 1500.0)
	if err != nil {
		t.Errorf("Expected no error, got %v", err)
	}
}

func TestRecordHarnessToolInvocation(t *testing.T) {
	ctx := context.Background()
	err := RecordHarnessToolInvocation(ctx, "test_tool")
	if err != nil {
		t.Errorf("Expected no error, got %v", err)
	}
}

func TestRecordHarnessViolation(t *testing.T) {
	ctx := context.Background()
	err := RecordHarnessViolation(ctx, "timeout")
	if err != nil {
		t.Errorf("Expected no error, got %v", err)
	}
}
