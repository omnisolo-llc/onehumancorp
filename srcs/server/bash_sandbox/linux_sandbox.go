package bash_sandbox

import (
	"bytes"
	"context"
	"fmt"
	"os/exec"
)

// LinuxSandbox wraps a command in a bwrap sandbox.
type LinuxSandbox struct {
	executablePath string
	workspaceDir   string
}

// NewLinuxSandbox creates a new LinuxSandbox.
func NewLinuxSandbox(executablePath string, workspaceDir string) *LinuxSandbox {
	return &LinuxSandbox{
		executablePath: executablePath,
		workspaceDir:   workspaceDir,
	}
}

// getBwrapArgs returns the arguments to pass to bwrap.
func (s *LinuxSandbox) getBwrapArgs(command string, args []string) []string {
	bwrapArgs := []string{
		"--unshare-net",
		"--unshare-pid",
		"--ro-bind", "/", "/",
		"--dev", "/dev",
		"--proc", "/proc",
		"--bind", s.workspaceDir, s.workspaceDir,
		"--die-with-parent",
		"--chdir", s.workspaceDir,
		command,
	}
	bwrapArgs = append(bwrapArgs, args...)
	return bwrapArgs
}

// Run executes the given command inside the bwrap sandbox.
func (s *LinuxSandbox) Run(ctx context.Context, command string, args []string) (string, error) {
	bwrapArgs := s.getBwrapArgs(command, args)

	cmd := exec.CommandContext(ctx, s.executablePath, bwrapArgs...)

	var stdout, stderr bytes.Buffer
	cmd.Stdout = &stdout
	cmd.Stderr = &stderr

	err := cmd.Run()
	if err != nil {
		return "", fmt.Errorf("command execution failed: %v, stderr: %s", err, stderr.String())
	}

	return stdout.String(), nil
}
