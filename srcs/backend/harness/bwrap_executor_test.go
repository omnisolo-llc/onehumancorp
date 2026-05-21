package harness

import (
	"context"
	"testing"
)

func TestBwrapExecutor(t *testing.T) {
	telemetry := &MockTelemetry{}
	executor := NewBwrapExecutor(telemetry)
	executor.BwrapBinary = "echo"

	output, err := executor.Execute(context.Background(), "test cmd", []string{"A=B"})
	if err != nil {
		t.Errorf("Expected nil error for mock echo binary, got %v", err)
	}

	if len(output) == 0 {
		t.Errorf("Expected some output from mock echo")
	}

	executor.BwrapBinary = "nonexistent-bwrap-binary"
	_, err = executor.Execute(context.Background(), "echo test", nil)
	if err == nil {
		t.Errorf("Expected error for non-existent binary, got nil")
	}

	if len(telemetry.Violations) == 0 {
		t.Errorf("Expected telemetry violation to be recorded on failure")
	}
}
