//go:build linux

package harness

import (
	"context"
	"os/exec"
	"strings"
	"time"

	"github.com/onehumancorp/mono/src/server_old/telemetry"
)

type BwrapHarness struct{}

func NewIsolationHarness() IsolationHarness {
	return NewPermissionInterceptor(&BwrapHarness{})
}

func (h *BwrapHarness) Execute(ctx context.Context, execCtx ExecutionContext) ([]byte, error) {
	telemetry.RecordBubblewrapSpawn(ctx)
	start := time.Now()

	args := []string{
		"--unshare-net",
		"--unshare-pid",
		"--dev", "/dev",
		"--ro-bind", "/", "/",
		// Prevent access to sensitive directories explicitly by mapping them to empty temp dirs
		"--tmpfs", "/etc",
		"--tmpfs", "/root",
	}

	// Add additional allowed paths if any
	for _, path := range execCtx.AllowedPaths {
		args = append(args, "--bind", path, path)
	}

	for _, path := range execCtx.AllowReadPaths {
		args = append(args, "--ro-bind", path, path)
	}

	for _, path := range execCtx.DenyWritePaths {
		args = append(args, "--tmpfs", path)
	}

	// Append the actual command to execute
	args = append(args, "--")
	args = append(args, execCtx.Command...)


	cmd := exec.CommandContext(ctx, "bwrap", args...)
	if execCtx.NetworkProxy != "" {
		cmd.Env = append(cmd.Environ(), "HTTP_PROXY="+execCtx.NetworkProxy, "HTTPS_PROXY="+execCtx.NetworkProxy)
	}
	out, err := cmd.CombinedOutput()

	duration := time.Since(start).Seconds()
	telemetry.RecordBubblewrapExecutionLatency(ctx, duration)

	if err != nil {
		if exitErr, ok := err.(*exec.ExitError); ok {
			// Common violation codes: 126 (command invoked cannot execute)
			// Or check the output for "Permission denied"
			if exitErr.ExitCode() != 0 && (strings.Contains(string(out), "Permission denied") || exitErr.ExitCode() == 126) {
				telemetry.RecordBubblewrapViolation(ctx)
			}
		} else {
			// if it's not an ExitError, it might be a failure to start bwrap at all.
			// Let's also check if it contains Permission denied just in case.
			if strings.Contains(err.Error(), "permission denied") {
				telemetry.RecordBubblewrapViolation(ctx)
			}
		}
	}

	return out, err
}
