package harness

import (
	"errors"
	"strings"
	"testing"
)

func TestMacOSSandboxManager_WrapCommand_Default(t *testing.T) {
	manager := NewMacOSSandboxManager()

	cmd := "echo 'hello'"
	wrapped, err := manager.WrapCommand(cmd)
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}

	if !strings.HasPrefix(wrapped, "sandbox-exec -p '(version 1)\n(allow default)\n' bash -c 'echo '\\''hello'\\'''") {
		t.Errorf("Unexpected wrapped command: %s", wrapped)
	}
}

func TestMacOSSandboxManager_WrapCommand_WithPolicy(t *testing.T) {
	manager := NewMacOSSandboxManager()

	policyJSON := `{"read_only_paths": ["/etc", "/var\" injection"], "blocked_domains": ["evil.com\" injection"]}`
	err := manager.UpdateConfig(policyJSON)
	if err != nil {
		t.Fatalf("Failed to update config: %v", err)
	}

	cmd := "ls -la"
	wrapped, err := manager.WrapCommand(cmd)
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}

	if !strings.Contains(wrapped, "(deny file-write* (subpath \"/etc\"))") {
		t.Errorf("Missing read-only path /etc in policy")
	}
	if !strings.Contains(wrapped, "(deny file-write* (subpath \"/var injection\"))") {
		t.Errorf("Missing read-only path /var in policy")
	}
	if !strings.Contains(wrapped, "(deny network-outbound (remote tcp \"evil.com injection:*\"))") {
		t.Errorf("Missing domain block evil.com in policy")
	}
	if !strings.Contains(wrapped, "bash -c 'ls -la'") {
		t.Errorf("Missing command execution in policy")
	}
}

func TestMacOSSandboxManager_UpdateConfig_Error(t *testing.T) {
	manager := NewMacOSSandboxManager()
	err := manager.UpdateConfig("invalid json")
	if err == nil {
		t.Errorf("Expected error for invalid json config")
	}
}

func TestMacOSSandboxManager_Evaluate_Denied(t *testing.T) {
	manager := NewMacOSSandboxManager()
	policyJSON := `{"disabled_commands": ["rm"], "disabled_patterns": ["/etc/passwd"]}`
	manager.UpdateConfig(policyJSON)

	cmd := "rm -rf /"
	_, err := manager.WrapCommand(cmd)
	if err == nil {
		t.Errorf("Expected command 'rm' to be denied")
	}

	cmd2 := "echo; rm"
	_, err2 := manager.WrapCommand(cmd2)
	if err2 == nil {
		t.Errorf("Expected bypassed command 'echo; rm' to be denied")
	}

	cmd3 := "cat /etc/passwd"
	_, err3 := manager.WrapCommand(cmd3)
	if err3 == nil {
		t.Errorf("Expected command with pattern '/etc/passwd' to be denied")
	}
}

func TestMacOSSandboxManager_AnnotateError(t *testing.T) {
	manager := NewMacOSSandboxManager()

	stdout := "some error output"
	err := errors.New("exit status 1")
	annotated := manager.AnnotateError(err, stdout)

	expected := "SANDBOX_FAILURE: exit status 1\nSTDOUT:\nsome error output"

	if annotated != expected {
		t.Errorf("Unexpected error annotation: got %s, expected %s", annotated, expected)
	}
}

func TestMacOSSandboxManager_Execute(t *testing.T) {
	manager := NewMacOSSandboxManager()
	// Test error returned from wrap command
	policyJSON := `{"disabled_commands": ["rm"]}`
	manager.UpdateConfig(policyJSON)

	_, err := manager.Execute("rm -rf /")
	if err == nil {
		t.Errorf("Expected error from execute due to disabled command")
	}

	_, _ = manager.Execute("echo hello")
}
