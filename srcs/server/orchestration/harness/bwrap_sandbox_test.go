package harness

import (
	"strings"
	"testing"
)

func TestBwrapSandboxWrapCommand(t *testing.T) {
	manager := NewBwrapSandboxManager()
	policyJSON := `{
		"disabled_commands": ["rm"],
		"disabled_patterns": ["/etc/passwd"]
	}`
	err := manager.UpdateConfig(policyJSON)
	if err != nil {
		t.Fatalf("Failed to update config: %v", err)
	}

	cmd := "ls -la"
	wrapped, err := manager.WrapCommand(cmd)
	if err != nil {
		t.Errorf("Expected no error, got %v", err)
	}
	if !strings.Contains(wrapped, "bwrap") {
		t.Errorf("Expected bwrap command, got %s", wrapped)
	}

	cmd = "rm -rf /"
	_, err = manager.WrapCommand(cmd)
	if err == nil {
		t.Errorf("Expected error for disabled command")
	}

	cmd = "cat /etc/passwd"
	_, err = manager.WrapCommand(cmd)
	if err == nil {
		t.Errorf("Expected error for disabled pattern")
	}
}

func TestBwrapSandboxAnnotateError(t *testing.T) {
	manager := NewBwrapSandboxManager()
	errStr := manager.AnnotateError(nil, "output")
	if !strings.Contains(errStr, "BWRAP_FAILURE") || !strings.Contains(errStr, "output") {
		t.Errorf("Expected BWRAP_FAILURE and output, got %s", errStr)
	}
}

func TestBwrapSandboxExecuteMetrics(t *testing.T) {
	manager := NewBwrapSandboxManager()
	// An empty policy means nothing is disabled
	err := manager.UpdateConfig("{}")
	if err != nil {
		t.Fatalf("Failed to update config: %v", err)
	}

	// This should trigger telemetry.RecordBubblewrapSpawn and telemetry.RecordBubblewrapExecutionLatency
	// It will likely fail to execute bwrap locally unless bwrap is installed, but the telemetry calls
	// happen before the execution error is returned.
	_, _ = manager.Execute("echo 1")
}
