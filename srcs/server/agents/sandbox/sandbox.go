package sandbox

import (
	"bytes"
	"context"
	"fmt"
	"os"
	"os/exec"
)

// SandboxManager manages a secure environment for running arbitrary shell commands.
type SandboxManager struct {
	tmpDir string
}

// NewSandboxManager creates a new SandboxManager and initializes its secure temporary directory.
func NewSandboxManager() (*SandboxManager, error) {
	// Create a secure temporary directory with 0700 permissions
	tmpDir, err := os.MkdirTemp("", "sandbox-*")
	if err != nil {
		return nil, fmt.Errorf("failed to create sandbox tmpdir: %w", err)
	}

	err = os.Chmod(tmpDir, 0700)
	if err != nil {
		os.RemoveAll(tmpDir) // Cleanup on failure
		return nil, fmt.Errorf("failed to chmod sandbox tmpdir: %w", err)
	}

	return &SandboxManager{tmpDir: tmpDir}, nil
}

// Cleanup removes the temporary directory created for the sandbox.
func (sm *SandboxManager) Cleanup() error {
	if sm.tmpDir != "" {
		err := os.RemoveAll(sm.tmpDir)
		if err != nil {
			return fmt.Errorf("failed to cleanup sandbox tmpdir: %w", err)
		}
		sm.tmpDir = ""
	}
	return nil
}

// Execute runs a command in the sandbox. It enforces a timeout via the context,
// disables extended globs, and sets the TMPDIR to the sandbox's secure directory.
func (sm *SandboxManager) Execute(ctx context.Context, command string) (string, string, error) {
	// Wrap the command to disable extended globs in bash
	wrappedCmd := fmt.Sprintf("shopt -u extglob 2>/dev/null || true; %s", command)

	cmd := exec.CommandContext(ctx, "bash", "-c", wrappedCmd)

	// Build environment, filtering out any existing TMPDIR and adding our sandbox TMPDIR
	var env []string
	for _, e := range os.Environ() {
		if len(e) >= 7 && e[:7] == "TMPDIR=" {
			continue
		}
		env = append(env, e)
	}
	env = append(env, fmt.Sprintf("TMPDIR=%s", sm.tmpDir))
	cmd.Env = env

	var stdout, stderr bytes.Buffer
	cmd.Stdout = &stdout
	cmd.Stderr = &stderr

	err := cmd.Run()

	return stdout.String(), stderr.String(), err
}
