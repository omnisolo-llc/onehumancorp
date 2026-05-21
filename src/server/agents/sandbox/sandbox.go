package sandbox

import (
	"context"
	"fmt"
	"os"
	"os/exec"
)

// SandboxManager struct
type SandboxManager struct {
	Dir string
}

// NewSandboxManager creates a new SandboxManager
func NewSandboxManager() (*SandboxManager, error) {
	dir, err := os.MkdirTemp("", "sandbox-*")
	if err != nil {
		return nil, err
	}
	err = os.Chmod(dir, 0700)
	if err != nil {
		os.RemoveAll(dir)
		return nil, err
	}
	return &SandboxManager{Dir: dir}, nil
}

// Cleanup removes the sandbox directory
func (s *SandboxManager) Cleanup() error {
	return os.RemoveAll(s.Dir)
}

// Execute runs the command in the sandbox
func (s *SandboxManager) Execute(ctx context.Context, cmd string) ([]byte, error) {
	// Wrap command for Bash execution to disable extended globs
	wrappedCmd := fmt.Sprintf("shopt -u extglob 2>/dev/null || true; %s", cmd)

	c := exec.CommandContext(ctx, "bash", "-c", wrappedCmd)

	// Force TMPDIR to sandbox directory
	c.Env = append(os.Environ(), fmt.Sprintf("TMPDIR=%s", s.Dir))

	return c.CombinedOutput()
}
