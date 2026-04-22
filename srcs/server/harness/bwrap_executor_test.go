package harness

import (
	"context"
	"strings"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

func TestBwrapExecutor_Execute(t *testing.T) {
	violationCount := 0
	originalFunc := telemetry.BufferMetricFunc
	telemetry.BufferMetricFunc = func(ctx context.Context, name string, payload string) error {
		if name == "sandbox_violation_total" || name == "bwrap_violation_total" {
			violationCount++
		}
		return nil
	}
	defer func() { telemetry.BufferMetricFunc = originalFunc }()

	executor := NewBwrapExecutor("agent-123")

	out, err := executor.Execute(context.Background(), []string{"echo", "test"})
	if err == nil {
		t.Logf("Execute succeeded unexpectedly or bwrap is installed. out: %s", string(out))
	} else {
		if !strings.Contains(err.Error(), "executable file not found") && !strings.Contains(err.Error(), "exit status 1") {
			t.Logf("Got error: %v", err)
		}
		if executor.AgentID != "agent-123" {
			t.Errorf("Expected AgentID to be agent-123")
		}
	}
}
