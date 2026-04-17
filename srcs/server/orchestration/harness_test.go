package orchestration

import (
	"context"
	"testing"
)

func TestSandboxAdapterEmitViolation(t *testing.T) {
	// Basic test to ensure it doesn't panic
	adapter := NewSandboxAdapter()
	ctx := context.Background()

	// Should call telemetry without crashing
	adapter.EmitViolation(ctx, "fs_write", "test-agent", "/forbidden/path")
}
