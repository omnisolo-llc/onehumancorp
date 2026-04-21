//go:build linux

package harness

import (
	"context"
	"os/exec"
)

type BwrapHarness struct{}

func NewIsolationHarness() IsolationHarness {
	return &BwrapHarness{}
}

func (h *BwrapHarness) Execute(ctx context.Context, execCtx ExecutionContext) ([]byte, error) {
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
