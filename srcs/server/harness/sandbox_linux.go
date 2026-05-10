//go:build linux

package harness

import (
	"bytes"
	"context"
	"os/exec"
)

// LinuxSandboxManager implements SandboxManager using bubblewrap (bwrap).
type LinuxSandboxManager struct {
    workspaceDir string
}

// NewSandboxManager creates a new SandboxManager for Linux.
func NewSandboxManager(workspaceDir string) SandboxManager {
	return &LinuxSandboxManager{workspaceDir: workspaceDir}
}

// ExecuteCommand runs a command within the bwrap sandbox.
func (sm *LinuxSandboxManager) ExecuteCommand(ctx context.Context, command string, args ...string) (string, string, error) {
	bwrapArgs := []string{
		"--ro-bind", "/", "/",
		"--dev", "/dev",
		"--proc", "/proc",
		"--unshare-net",
	}

    // Instead of completely replacing /tmp with tmpfs which breaks if workspaceDir is in /tmp,
    // we just bind the workspaceDir to itself. We could restrict other places but this satisfies the basic requirement.
    if sm.workspaceDir != "" {
        bwrapArgs = append(bwrapArgs, "--bind", sm.workspaceDir, sm.workspaceDir)
    }

	bwrapArgs = append(bwrapArgs, "--")
	bwrapArgs = append(bwrapArgs, command)
	bwrapArgs = append(bwrapArgs, args...)

	cmd := exec.CommandContext(ctx, "bwrap", bwrapArgs...)

	var stdout, stderr bytes.Buffer
	cmd.Stdout = &stdout
	cmd.Stderr = &stderr

	err := cmd.Run()

	return stdout.String(), stderr.String(), err
}
