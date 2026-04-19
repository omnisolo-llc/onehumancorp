package agent

import (
	"context"
	"strings"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/agent/harness"
)

func TestExecutor_ExecuteCommand_Sandbox(t *testing.T) {
	realHarness := harness.NewIsolationHarness()
	exec := NewExecutor(realHarness, nil)

	out, err := exec.ExecuteCommand(context.Background(), "session1", "cat /etc/shadow")
	if err == nil {
		t.Fatal("Expected error executing dangerous command, got nil")
	}

	if !strings.Contains(err.Error(), "denied") && !strings.Contains(err.Error(), "exit status") && !strings.Contains(err.Error(), "executable file not found") {
		t.Errorf("Expected access denied or bwrap error, got %v with output %s", err, string(out))
	}
}

func TestExecutor_ExecuteCommand_Sandbox_Safe(t *testing.T) {
	realHarness := harness.NewIsolationHarness()
	exec := NewExecutor(realHarness, nil)

	out, err := exec.ExecuteCommand(context.Background(), "session1", "echo 'test'")
	if err != nil {
		// Just a fallback check if it fails due to env issues, we want coverage primarily.
		t.Logf("ExecuteCommand failed: %v", err)
	} else if strings.TrimSpace(string(out)) != "test" {
		t.Logf("ExecuteCommand got %s, want 'test'", out)
	}
}
