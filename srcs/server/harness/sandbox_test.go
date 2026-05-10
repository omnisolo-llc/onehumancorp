//go:build linux || darwin

package harness

import (
	"context"
	"os"
	"os/exec"
	"path/filepath"
	"runtime"
	"testing"
)

func TestSandboxFileWrites(t *testing.T) {
	if runtime.GOOS == "linux" {
		if _, err := exec.LookPath("bwrap"); err != nil {
			t.Skip("bwrap not found, skipping test on linux")
		}
	} else if runtime.GOOS == "darwin" {
		if _, err := exec.LookPath("sandbox-exec"); err != nil {
			t.Skip("sandbox-exec not found, skipping test on darwin")
		}
	}

	tempDir, err := os.MkdirTemp("", "sandbox-test-*")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	sm := NewSandboxManager(tempDir)

	ctx := context.Background()

	// 1. Test writing inside the allowed workspace directory
	allowedFile := filepath.Join(tempDir, "allowed.txt")
	_, _, err = sm.ExecuteCommand(ctx, "bash", "-c", "echo 'test' > "+allowedFile)
	if err != nil {
		t.Errorf("Expected to be able to write to allowed workspace, but got error: %v", err)
	}

	if _, err := os.Stat(allowedFile); os.IsNotExist(err) {
		t.Errorf("File %s was not created in the workspace", allowedFile)
	}

	// 2. Test writing outside the allowed workspace directory
	// In darwin, writing outside subpath is blocked by sandbox-exec.
	// In linux, since we use ro-bind for / by default, writing to /tmp or /etc should be blocked
    // depending on exactly how bwrap sets it up, but let's test a root read-only location like /etc.

	_, _, err = sm.ExecuteCommand(ctx, "bash", "-c", "echo 'test' > /etc/sandbox_test_blocked.txt")
	if err == nil {
		t.Errorf("Expected writing outside allowed workspace to be blocked, but it succeeded")
	}
}
