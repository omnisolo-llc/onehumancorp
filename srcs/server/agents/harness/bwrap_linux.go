//go:build linux

package harness

import (
	"context"
	"os/exec"
	"strings"
	"time"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"github.com/onehumancorp/mono/srcs/server/agents/harness/network"
	"fmt"
)

type BwrapHarness struct{}

func NewIsolationHarness() IsolationHarness {
	return NewPermissionInterceptor(&BwrapHarness{})
}

func (h *BwrapHarness) Execute(ctx context.Context, execCtx ExecutionContext) ([]byte, error) {
	telemetry.RecordBubblewrapSpawn(ctx)
	start := time.Now()

	if len(execCtx.AllowedDomains) > 0 {
		socketPath := fmt.Sprintf("/tmp/ohc-proxy-%d.sock", time.Now().UnixNano())
		proxy := network.NewNetworkBridgeProxy(socketPath, execCtx.AllowedDomains)
		if err := proxy.Start(); err != nil {
			return nil, fmt.Errorf("failed to start network bridge proxy: %w", err)
		}
		defer proxy.Stop()
		execCtx.NetworkProxy = "unix://" + socketPath
	}

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

	if strings.HasPrefix(execCtx.NetworkProxy, "unix://") {
		socketPath := strings.TrimPrefix(execCtx.NetworkProxy, "unix://")
		args = append(args, "--bind", socketPath, socketPath)
		// We override the command to spawn a background socat in the container
		// Then it executes the user command. We use bash to orchestrate this.
		// Use "$@" to preserve exact arguments securely.
		bashCmd := fmt.Sprintf("socat TCP-LISTEN:3128,fork UNIX-CONNECT:%s & sleep 0.1; exec \"$@\"", socketPath)
		args = append(args, "--")
		args = append(args, "bash", "-c", bashCmd, "--")
		args = append(args, execCtx.Command...)
	} else {
		// Append the actual command to execute
		args = append(args, "--")
		args = append(args, execCtx.Command...)
	}


	cmd := exec.CommandContext(ctx, "bwrap", args...)

	// Start with default environment
	cmd.Env = cmd.Environ()
	// Add standard PATH explicitly
	hasPath := false
	for _, env := range cmd.Env {
		if strings.HasPrefix(env, "PATH=") {
			hasPath = true
			break
		}
	}
	if !hasPath {
		cmd.Env = append(cmd.Env, "PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin")
	}

	if execCtx.NetworkProxy != "" {
		proxyStr := execCtx.NetworkProxy
		if strings.HasPrefix(execCtx.NetworkProxy, "unix://") {
			proxyStr = "http://127.0.0.1:3128"
		}
		cmd.Env = append(cmd.Env,
			"HTTP_PROXY="+proxyStr,
			"HTTPS_PROXY="+proxyStr,
			"ALL_PROXY="+proxyStr,
		)
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
