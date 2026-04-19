package agent

import (
	"fmt"

	"context"
	"strings"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/agent/harness"
)

func TestExecutor_E2E_ShadowAccess(t *testing.T) {
	realHarness := harness.NewIsolationHarness()
	exec := NewExecutor(realHarness)

	out, err := exec.ExecuteCommand(context.Background(), "cat /etc/shadow")

	if err == nil {
		t.Fatalf("expected an error or failure when accessing /etc/shadow, got nil. Output: %s", string(out))
	}

	outStr := string(out)
	errStr := err.Error()

	if !strings.Contains(outStr, "Permission denied") &&
		!strings.Contains(outStr, "No such file") &&
		!strings.Contains(outStr, "not permitted") &&
		!strings.Contains(errStr, "not found") &&
		!strings.Contains(errStr, "exit status") {
		t.Fatalf("Unexpected output when trying to access /etc/shadow: out=%s, err=%s", outStr, errStr)
	}
}

func TestExecutor_Success(t *testing.T) {
	realHarness := harness.NewIsolationHarness()
	exec := NewExecutor(realHarness)

	out, err := exec.ExecuteCommand(context.Background(), "echo 'test'")

	if err != nil {
		if strings.Contains(err.Error(), "not found") {
			t.Skipf("Skipping success test because bwrap/sandbox-exec is not installed: %v", err)
			return
		}
	}

	if err == nil && !strings.Contains(string(out), "test") {
		t.Fatalf("expected output to contain 'test', got: %s", string(out))
	}
}

// A mock harness for testing telemetry and tracing context propagation
type mockHarnessWithTrace struct {
	lastSpanCtx context.Context
}

func (m *mockHarnessWithTrace) Execute(ctx context.Context, execCtx harness.ExecutionContext) ([]byte, error) {
	m.lastSpanCtx = ctx
	return []byte("mock output"), nil
}

func TestExecutor_TracingAndTelemetry(t *testing.T) {
	mockH := &mockHarnessWithTrace{}
	exec := NewExecutor(mockH)

	ctx := context.WithValue(context.Background(), "tenantID", "test-tenant-123")

	out, err := exec.ExecuteCommand(ctx, "echo test")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if string(out) != "mock output" {
		t.Fatalf("unexpected output: %s", string(out))
	}

	if mockH.lastSpanCtx == nil {
		t.Fatal("expected span context to be passed to harness")
	}
}

type errorMockHarness struct{}

func (m *errorMockHarness) Execute(ctx context.Context, execCtx harness.ExecutionContext) ([]byte, error) {
	return []byte("error output"), fmt.Errorf("mock error")
}

func TestExecutor_ErrorTracingAndTelemetry(t *testing.T) {
	mockH := &errorMockHarness{}
	exec := NewExecutor(mockH)

	out, err := exec.ExecuteCommand(context.Background(), "false")

	if err == nil {
		t.Fatal("expected error, got nil")
	}

	if string(out) != "error output" {
		t.Fatalf("unexpected output: %s", string(out))
	}
}
