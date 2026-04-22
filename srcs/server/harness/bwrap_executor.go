package harness

import (
	"bytes"
	"context"
	"fmt"
	"os/exec"
)

// BwrapExecutor wraps the bwrap CLI command to execute commands within a sandbox.
type BwrapExecutor struct {
	emitter SandboxTelemetryEmitter
}

// NewBwrapExecutor creates a new BwrapExecutor.
func NewBwrapExecutor(emitter SandboxTelemetryEmitter) *BwrapExecutor {
	if emitter == nil {
		emitter = &DefaultSandboxTelemetryEmitter{}
	}
	return &BwrapExecutor{emitter: emitter}
}

// Execute runs the given command using bwrap with read-only root and isolated /tmp.
func (e *BwrapExecutor) Execute(ctx context.Context, agentID string, env map[string]string, cmdName string, args ...string) (Result, error) {
	bwrapArgs := []string{
		"--unshare-pid",
		"--unshare-uts",
		"--unshare-ipc",
		"--unshare-cgroup",
		"--proc", "/proc",
		"--dev", "/dev",
		"--tmpfs", "/tmp",
		"--ro-bind", "/", "/",
		"--",
		cmdName,
	}
	bwrapArgs = append(bwrapArgs, args...)

	cmd := exec.CommandContext(ctx, "bwrap", bwrapArgs...)

	cmd.Env = append(cmd.Environ(), "PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin")
	var envList []string
	for k, v := range env {
		envList = append(envList, fmt.Sprintf("%s=%s", k, v))
	}
	if len(envList) > 0 {
		cmd.Env = append(cmd.Env, envList...)
	}

	var stdout, stderr bytes.Buffer
	cmd.Stdout = &stdout
	cmd.Stderr = &stderr

	err := cmd.Run()
	exitCode := 0
	if err != nil {
		if exitError, ok := err.(*exec.ExitError); ok {
			exitCode = exitError.ExitCode()
			if exitCode == 1 && bytes.Contains(stderr.Bytes(), []byte("bwrap:")) {
				e.emitter.EmitViolation(ctx, "fs_read", agentID, "/")
			}
		} else {
			return Result{}, fmt.Errorf("failed to run bwrap: %w", err)
		}
	}

	return Result{
		Stdout:   stdout.String(),
		Stderr:   stderr.String(),
		ExitCode: exitCode,
	}, nil
}
