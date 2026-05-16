package harness

import (
	"os"
	"path/filepath"
	"runtime"
	"strings"
	"testing"
)

func TestSandboxAdapterSelection(t *testing.T) {
	config := SandboxConfig{}
	manager, err := NewSandboxManager(config)

	if runtime.GOOS == "linux" || runtime.GOOS == "darwin" {
		if err != nil {
			t.Fatalf("Expected manager to be created on %s, got error: %v", runtime.GOOS, err)
		}
		if manager == nil {
			t.Fatalf("Expected manager to be non-nil on %s", runtime.GOOS)
		}
	} else {
		if err == nil {
			t.Fatalf("Expected error on unsupported OS %s, got nil", runtime.GOOS)
		}
	}
}

func TestBwrapAdapter_WrapCommand(t *testing.T) {
	config := SandboxConfig{
		WorkspaceDir: "/app/workspace",
		ReadOnlyDirs: []string{"/var/log"},
		Network:      false,
	}

	adapter := &BwrapAdapter{Config: config}
	cmd, args, err := adapter.WrapCommand("echo", []string{"hello"})

	if err != nil {
		t.Fatalf("WrapCommand failed: %v", err)
	}

	if cmd != "bwrap" {
		t.Errorf("Expected command to be 'bwrap', got '%s'", cmd)
	}

	// Simple checks to ensure key arguments are present
	argsStr := strings.Join(args, " ")
	if !strings.Contains(argsStr, "--ro-bind /bin /bin") {
		t.Errorf("Expected /bin to be read-only bound")
	}
	if !strings.Contains(argsStr, "--bind /app/workspace /app/workspace") {
		t.Errorf("Expected workspace to be read-write bound")
	}
	if !strings.Contains(argsStr, "--ro-bind /var/log /var/log") {
		t.Errorf("Expected /var/log to be read-only bound")
	}
	if !strings.Contains(argsStr, "--unshare-net") {
		t.Errorf("Expected network to be unshared")
	}
}

func TestSandboxExecAdapter_WrapCommand(t *testing.T) {
	config := SandboxConfig{
		WorkspaceDir: "/app/workspace",
		Network:      true,
	}

	adapter := &SandboxExecAdapter{Config: config}
	cmd, args, err := adapter.WrapCommand("echo", []string{"hello"})

	if err != nil {
		t.Fatalf("WrapCommand failed: %v", err)
	}

	if cmd != "sandbox-exec" {
		t.Errorf("Expected command to be 'sandbox-exec', got '%s'", cmd)
	}

	argsStr := strings.Join(args, " ")
	if !strings.Contains(argsStr, "(allow file-write* (subpath \"/app/workspace\"))") {
		t.Errorf("Expected workspace write permission in profile")
	}
	if !strings.Contains(argsStr, "(allow network*)") {
		t.Errorf("Expected network permission in profile")
	}
}

func TestSandboxExecution_FileWriteRestrictions(t *testing.T) {
	// Only run on Linux (bwrap) or macOS (sandbox-exec) if they exist
	if runtime.GOOS != "linux" && runtime.GOOS != "darwin" {
		t.Skipf("Skipping execution test on unsupported OS: %s", runtime.GOOS)
	}

	// Create a temporary workspace
	workspaceDir, err := os.MkdirTemp("", "sandbox-test-workspace-*")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(workspaceDir)

	config := SandboxConfig{
		WorkspaceDir: workspaceDir,
		Network:      true, // Doesn't matter for this test
	}

	manager, err := NewSandboxManager(config)
	if err != nil {
		t.Fatalf("Failed to create SandboxManager: %v", err)
	}

	// Test 1: Writing to workspace should succeed
	allowedFile := filepath.Join(workspaceDir, "allowed.txt")
	out, err := manager.ExecuteCommand("touch", []string{allowedFile})
	if err != nil {
		t.Errorf("Expected success writing to workspace, but got error: %v. Output: %s", err, string(out))
	} else {
		// Verify the file was actually created
		if _, err := os.Stat(allowedFile); os.IsNotExist(err) {
			t.Errorf("File %s was not created despite successful command execution", allowedFile)
		}
	}

	// Test 2: Writing to a known read-only path (e.g. /bin) should fail due to permissions
	// (Ensure the path is something standard that isn't isolated by tmpfs and is read-only mounted)
	forbiddenFile := "/bin/forbidden_sandbox_test.txt"
	out, err = manager.ExecuteCommand("touch", []string{forbiddenFile})

	if err == nil {
		t.Errorf("Expected failure when writing to system path %s, but command succeeded. Output: %s", forbiddenFile, string(out))
		os.Remove(forbiddenFile)
	}

	// Test 3: Ensure read access is restricted outside of workspace and essential paths
	// `/etc/passwd` shouldn't be accessible (since `/etc` isn't mounted)
	forbiddenRead := "/etc/passwd"
	out, err = manager.ExecuteCommand("cat", []string{forbiddenRead})
	if err == nil {
		t.Errorf("Expected failure when reading system path %s, but command succeeded. Output: %s", forbiddenRead, string(out))
	}
}
