//go:build darwin

package harness

import (
	"bytes"
	"context"
	"fmt"
	"os/exec"
)

// DarwinSandboxManager implements SandboxManager using sandbox-exec.
type DarwinSandboxManager struct {
    workspaceDir string
}

// NewSandboxManager creates a new SandboxManager for macOS.
func NewSandboxManager(workspaceDir string) SandboxManager {
	return &DarwinSandboxManager{workspaceDir: workspaceDir}
}

// ExecuteCommand runs a command within the sandbox-exec sandbox.
func (sm *DarwinSandboxManager) ExecuteCommand(ctx context.Context, command string, args ...string) (string, string, error) {
	// Generate a basic sandbox profile
	profile := "(version 1)\n" +
		"(deny default)\n" +
		"(allow file-read*)\n" +
		"(allow process-exec)\n" +
        "(allow process-fork)\n" +
		"(allow sysctl-read)\n" +
		"(deny network*)\n"

    if sm.workspaceDir != "" {
        profile += fmt.Sprintf("(allow file-write* (subpath \"%s\"))\n", sm.workspaceDir)
    }

	sandboxArgs := []string{
		"-p", profile,
		command,
	}
	sandboxArgs = append(sandboxArgs, args...)

	cmd := exec.CommandContext(ctx, "sandbox-exec", sandboxArgs...)

	var stdout, stderr bytes.Buffer
	cmd.Stdout = &stdout
	cmd.Stderr = &stderr

	err := cmd.Run()

	return stdout.String(), stderr.String(), err
}
