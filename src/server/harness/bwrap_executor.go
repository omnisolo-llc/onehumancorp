package harness

import (
	"bytes"
	"context"
	"fmt"
	"os/exec"
)

// BwrapExecutor wraps the bwrap CLI command to create strict filesystem mounts.
type BwrapExecutor struct{}

// NewBwrapExecutor creates a new BwrapExecutor.
func NewBwrapExecutor() *BwrapExecutor {
	return &BwrapExecutor{}
}

// Execute runs the command in a bwrap sandbox with read-only root and isolated /tmp.
func (e *BwrapExecutor) Execute(ctx context.Context, command string) (Result, error) {
	args := []string{
		"--unshare-pid",
		"--unshare-uts",
		"--unshare-ipc",
		"--unshare-cgroup",
		"--proc", "/proc",
		"--dev", "/dev",
		"--ro-bind", "/", "/",
		"--tmpfs", "/tmp",
		"--",
		"bash", "-c", command,
	}

	cmd := exec.CommandContext(ctx, "bwrap", args...)

	var stdout, stderr bytes.Buffer
	cmd.Stdout = &stdout
	cmd.Stderr = &stderr

	err := cmd.Run()
	exitCode := 0
	if err != nil {
		if exitError, ok := err.(*exec.ExitError); ok {
			exitCode = exitError.ExitCode()
		} else {
			return Result{Stdout: stdout.String(), Stderr: stderr.String(), ExitCode: -1}, fmt.Errorf("failed to run bwrap: %w", err)
		}
	}

	return Result{
		Stdout:   stdout.String(),
		Stderr:   stderr.String(),
		ExitCode: exitCode,
	}, nil
}
