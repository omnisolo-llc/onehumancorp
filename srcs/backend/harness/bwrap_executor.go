package harness

import (
	"context"
	"fmt"
	"os"
	"os/exec"
	"strings"
)

type BwrapExecutor struct {
	BwrapBinary string
	Env         []string
	Telemetry   SandboxTelemetryEmitter
}

func NewBwrapExecutor(telemetry SandboxTelemetryEmitter) *BwrapExecutor {
	return &BwrapExecutor{
		BwrapBinary: "bwrap",
		Env:         os.Environ(),
		Telemetry:   telemetry,
	}
}

// Execute accepts cmd and an optional customEnv that will be merged with base env.
func (e *BwrapExecutor) Execute(ctx context.Context, cmd string, customEnv []string) (string, error) {
	args := []string{
		"--ro-bind", "/", "/",
		"--tmpfs", "/tmp",
		"--dev", "/dev",
		"--proc", "/proc",
		"--unshare-user",
		"--unshare-pid",
		"--unshare-uts",
		"--unshare-ipc",
		"--unshare-cgroup",
		"--",
		"sh", "-c", cmd,
	}

	command := exec.CommandContext(ctx, e.BwrapBinary, args...)

	// Avoid data race by combining env here rather than mutating the struct
	env := make([]string, 0, len(e.Env)+len(customEnv))
	env = append(env, e.Env...)
	env = append(env, customEnv...)
	command.Env = env

	output, err := command.CombinedOutput()
	if err != nil {
		if e.Telemetry != nil {
			_ = e.Telemetry.RecordViolation(ctx, "file_access_denial", fmt.Sprintf("bwrap execution failed with: %v. output: %s", err, string(output)))
		}
		return string(output), fmt.Errorf("bwrap execution failed: %w", err)
	}

	return strings.TrimSpace(string(output)), nil
}
