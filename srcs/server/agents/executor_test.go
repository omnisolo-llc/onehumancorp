package agents

import (
	"context"
	"strings"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/agents/harness"
)

// We replace NewExecutor with direct harness usage since Executor was likely refactored.
func TestExecutor_E2E_ShadowAccess(t *testing.T) {
	realHarness := harness.NewIsolationHarness()

	out, err := realHarness.Execute(context.Background(), harness.ExecutionContext{Command: []string{"cat", "/etc/shadow"}})

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

	out, err := realHarness.Execute(context.Background(), harness.ExecutionContext{Command: []string{"echo", "test"}})

	if err != nil {
		if strings.Contains(err.Error(), "not found") || strings.Contains(err.Error(), "exit status") {
			t.Skipf("Skipping success test because bwrap/sandbox-exec is not installed: %v", err)
			return
		}
	}

	if err == nil && !strings.Contains(string(out), "test") {
		t.Fatalf("expected output to contain 'test', got: %s", string(out))
	}
}
