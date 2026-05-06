package harness

import (
	"strings"
	"testing"
)

func TestBwrapSandboxWrapCommand(t *testing.T) {
	manager := NewBwrapExecutor()
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
	manager := NewBwrapExecutor()
	errStr := manager.AnnotateError(nil, "output")
	if !strings.Contains(errStr, "BWRAP_FAILURE") || !strings.Contains(errStr, "output") {
		t.Errorf("Expected BWRAP_FAILURE and output, got %s", errStr)
	}
}
