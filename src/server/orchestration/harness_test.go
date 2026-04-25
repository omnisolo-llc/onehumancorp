package orchestration

import (
	"context"
	"strings"
	"testing"

	"github.com/onehumancorp/mono/src/server/agents/harness"
	"github.com/onehumancorp/mono/src/server/telemetry"
	"github.com/prometheus/client_golang/prometheus"
)

func TestDefaultSandboxAdapter_EmitViolation(t *testing.T) {
	prometheus.DefaultRegisterer = prometheus.NewRegistry()
	telemetry.InitTelemetry()

	adapter := &DefaultSandboxAdapter{}
	ctx := context.Background()
	// Should not panic
	adapter.EmitViolation(ctx, "fs_read", "agent-123", "/etc/passwd")
}

func TestHarnessGateway_Execute(t *testing.T) {
	gateway := NewHarnessGateway()
	ctx := context.Background()
	execCtx := harness.ExecutionContext{
		Command: []string{"echo", "hello"},
	}

	// Test Docker backend
	out, err := gateway.Execute(ctx, "docker", execCtx)
	if err != nil {
		t.Fatalf("expected no error for docker backend, got %v", err)
	}
	if string(out) != "executed in docker" {
		t.Fatalf("expected 'executed in docker', got '%s'", string(out))
	}

	// Test unsupported backend
	_, err = gateway.Execute(ctx, "unsupported", execCtx)
	if err == nil {
		t.Fatal("expected error for unsupported backend, got nil")
	}
	if err.Error() != "unsupported backend type" {
		t.Fatalf("expected 'unsupported backend type', got '%v'", err)
	}

	// Test Local backend
	_, err = gateway.Execute(ctx, "local", execCtx)
	if err != nil {
		if !strings.Contains(err.Error(), "executable file not found") && !strings.Contains(err.Error(), "sandbox-exec") {
			t.Fatalf("expected executable file not found error for local backend execution in test env, got %v", err)
		}
	}
}
