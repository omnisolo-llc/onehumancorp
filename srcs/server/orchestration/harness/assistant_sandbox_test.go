package harness

import (
	"strings"
	"testing"
)

func TestAssistantSandboxManager_Initialization(t *testing.T) {
	manager := NewAssistantSandboxManager()
	if manager == nil {
		t.Fatalf("Expected manager to be initialized")
	}
	if manager.adapter == nil {
		t.Fatalf("Expected underlying adapter to be initialized")
	}
}

func TestAssistantSandboxManager_WrapCommand(t *testing.T) {
	manager := NewAssistantSandboxManager()

	// Update config to ensure no policies block the test
	err := manager.UpdateConfig("{}")
	if err != nil {
		t.Fatalf("Failed to update config: %v", err)
	}

	cmd := "echo 'hello'"
	wrapped, err := manager.WrapCommand(cmd)
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}

	// The wrapped command should contain either 'bwrap' or 'sandbox-exec'
	if !strings.Contains(wrapped, "bwrap") && !strings.Contains(wrapped, "sandbox-exec") {
		t.Errorf("Expected OS-level isolation (bwrap or sandbox-exec), got: %s", wrapped)
	}
}

func TestAssistantSandboxManager_UpdateConfig_Error(t *testing.T) {
	manager := NewAssistantSandboxManager()
	err := manager.UpdateConfig("{invalid json}")
	if err == nil {
		t.Errorf("Expected error for invalid JSON")
	}
}

func TestAssistantSandboxManager_AnnotateError(t *testing.T) {
	manager := NewAssistantSandboxManager()
	errStr := manager.AnnotateError(nil, "output")

	// Should contain either BWRAP_FAILURE or SANDBOX_FAILURE
	if !strings.Contains(errStr, "FAILURE") || !strings.Contains(errStr, "output") {
		t.Errorf("Expected failure annotation with output, got %s", errStr)
	}
}

func TestAssistantSandboxManager_Execute(t *testing.T) {
	manager := NewAssistantSandboxManager()

	err := manager.UpdateConfig("{}")
	if err != nil {
		t.Fatalf("Failed to update config: %v", err)
	}

	_, _ = manager.Execute("echo 1")
}
