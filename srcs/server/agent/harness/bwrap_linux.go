//go:build linux

package harness

import (
	"context"
	"os"
	"os/exec"
	"time"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type BwrapHarness struct{}

func NewIsolationHarness() IsolationHarness {
	return &BwrapHarness{}
}

func (h *BwrapHarness) Execute(ctx context.Context, execCtx ExecutionContext) ([]byte, error) {
	start := time.Now()
	defer func() {
		mode := "standalone"
		if os.Getenv("OHC_MULTITENANT") == "true" {
			mode = "cloud"
		}
		telemetry.RecordHarnessExecutionDuration(ctx, time.Since(start).Seconds(), mode, "bwrap")
	}()

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

	// Append the actual command to execute
	args = append(args, "--")
	args = append(args, execCtx.Command...)

	cmd := exec.CommandContext(ctx, "bwrap", args...)
	return cmd.CombinedOutput()
}
