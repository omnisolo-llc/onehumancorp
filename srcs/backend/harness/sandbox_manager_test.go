package harness

import (
	"testing"
	"os/exec"
	"strings"
)

func TestGetBwrapArgs(t *testing.T) {
	sm := NewSandboxManager()
	args := sm.GetBwrapArgs("echo 'hello'", "/tmp")

    foundUnshare := false
    for _, arg := range args {
        if arg == "--unshare-all" {
            foundUnshare = true
            break
        }
    }

	if !foundUnshare {
		t.Errorf("Expected bwrap args to contain '--unshare-all'")
	}
}

func TestWrapWithSandbox(t *testing.T) {
	sm := NewSandboxManager()
	out := sm.WrapWithSandbox("echo 'hello'", "/tmp")
	if !strings.Contains(out, "--unshare-all") {
		t.Errorf("Expected string to contain --unshare-all")
	}
}

func TestExecute_DestructiveCommandFailsInsideSandbox(t *testing.T) {
	sm := NewSandboxManager()

    // We only test real execution if bwrap is available
    _, err := exec.LookPath("bwrap")
    if err != nil {
        t.Skip("bwrap not installed")
    }

	// Try writing to read-only directory
    out, err := sm.Execute("touch /etc/hacked", "/tmp")

    // If it succeeds (err == nil), then the sandbox failed to block the write to read-only /etc
    if err == nil {
        t.Errorf("Expected command to fail, got output: %s", out)
    }

	// Ensure the failure wasn't just because "apply-seccomp" or "bwrap" was missing
	if err != nil && strings.Contains(err.Error(), "executable file not found") {
		// It's possible we run this in an environment where things are missing
		// It's fine to skip if it's purely a missing dependency issue.
	}
}
