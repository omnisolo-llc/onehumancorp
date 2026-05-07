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

	// Since manager.UpdateConfig starts the proxy, it generates a unique socket path
	if !strings.Contains(wrapped, "--bind /tmp/harness-proxy-") {
		t.Errorf("Expected --bind to be injected, got %s", wrapped)
	}
	if !strings.Contains(wrapped, "socat TCP4-LISTEN:3128,fork,bind=127.0.0.1 UNIX-CONNECT:/tmp/harness-proxy-") {
		t.Errorf("Expected socat command to be injected, got %s", wrapped)
	}
	if !strings.Contains(wrapped, "HTTP_PROXY=http://127.0.0.1:3128") {
		t.Errorf("Expected HTTP_PROXY inside bash command, got %s", wrapped)
	}
	if strings.Contains(wrapped, "--share-net") {
		t.Errorf("Expected --share-net to NOT be injected for unix isolation, got %s", wrapped)
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

func TestBwrapSandboxExecuteProxy(t *testing.T) {
	manager := NewBwrapSandboxManager()
	policyJSON := `{
		"disabled_commands": ["rm"]
	}`
	manager.UpdateConfig(policyJSON)

	// Execute fails because bwrap isn't installed in the test environment, but the error should
	// come from exec.Run.
	_, _ = manager.Execute("echo hello")

	_, err := manager.Execute("rm -rf /")
	if err == nil {
		t.Errorf("Expected error for disabled command")
	}
}

func TestBwrapSandboxAnnotateError(t *testing.T) {
	manager := NewBwrapSandboxManager()
	errStr := manager.AnnotateError(nil, "output")
	if !strings.Contains(errStr, "BWRAP_FAILURE") || !strings.Contains(errStr, "output") {
		t.Errorf("Expected BWRAP_FAILURE and output, got %s", errStr)
	}
}
