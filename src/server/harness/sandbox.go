package harness

import (
	"context"
	"fmt"
	"os/exec"
	"runtime"

)

// SandboxManager defines the interface for an OS-level sandbox wrapper
type SandboxManager interface {
	// Execute runs the given command inside a sandbox and returns the stdout and stderr
	Execute(ctx context.Context, command []string, workDir string) ([]byte, error)
}

// BwrapSandboxManager implements SandboxManager for Linux using bubblewrap (bwrap)
type BwrapSandboxManager struct{}

// NewSandboxManager creates a new SandboxManager appropriate for the current OS
func NewSandboxManager() (SandboxManager, error) {
	if runtime.GOOS == "linux" {
		return &BwrapSandboxManager{}, nil
	} else if runtime.GOOS == "darwin" {
		return &MacSandboxManager{}, nil
	}
	return nil, fmt.Errorf("unsupported OS for sandboxing: %s", runtime.GOOS)
}

// Execute runs the command inside a bwrap sandbox
func (m *BwrapSandboxManager) Execute(ctx context.Context, command []string, workDir string) ([]byte, error) {
	// Restrict to read-only root, read-write workDir, and empty dev, proc, sys
	bwrapArgs := []string{
		"--ro-bind", "/", "/",
		"--dev", "/dev",
		"--proc", "/proc",
		"--unshare-all",
		"--share-net", // Required if the agent needs network, but can be restricted later
		"--bind", workDir, workDir,
		"--chdir", workDir,
	}

	bwrapArgs = append(bwrapArgs, command...)
	cmd := exec.CommandContext(ctx, "bwrap", bwrapArgs...)

	output, err := cmd.CombinedOutput()
	if err != nil {
		return output, fmt.Errorf("bwrap execution failed: %w, output: %s", err, string(output))
	}
	return output, nil
}

// MacSandboxManager implements SandboxManager for macOS using sandbox-exec
type MacSandboxManager struct{}

// Execute runs the command inside a sandbox-exec profile
func (m *MacSandboxManager) Execute(ctx context.Context, command []string, workDir string) ([]byte, error) {
	// Basic sandbox profile allowing read everywhere but write only to workDir
	profile := fmt.Sprintf(`
(version 1)
(deny default)
(allow file-read*)
(allow file-write* (subpath "%s"))
(allow network*)
(allow process-exec*)
(allow process-fork)
(allow sysctl-read)
`, workDir)

	args := []string{"-p", profile}
	args = append(args, command...)

	cmd := exec.CommandContext(ctx, "sandbox-exec", args...)
	output, err := cmd.CombinedOutput()
	if err != nil {
		return output, fmt.Errorf("sandbox-exec execution failed: %w, output: %s", err, string(output))
	}
	return output, nil
}
