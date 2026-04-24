package harness

import (
	"context"
	"os/exec"
)

type ExecutionContext struct {
	Command      []string
	AllowedPaths []string
	NetworkProxy string
}

type IsolationHarness interface {
	Execute(ctx context.Context, execCtx ExecutionContext) ([]byte, error)
}

type BwrapHarness struct {
	// CommandRunner allows mocking execution in tests
	CommandRunner func(ctx context.Context, name string, args ...string) *exec.Cmd
}

func NewBwrapHarness() *BwrapHarness {
	return &BwrapHarness{
		CommandRunner: exec.CommandContext,
	}
}

func (h *BwrapHarness) BuildArgs(execCtx ExecutionContext) []string {
	args := []string{
		"--unshare-net",
		"--unshare-pid",
		"--dev", "/dev",
		"--ro-bind", "/", "/",
		"--tmpfs", "/tmp",
	}

	for _, path := range execCtx.AllowedPaths {
		args = append(args, "--bind", path, path)
	}

	args = append(args, "--")
	args = append(args, execCtx.Command...)
	return args
}

func (h *BwrapHarness) Execute(ctx context.Context, execCtx ExecutionContext) ([]byte, error) {
	args := h.BuildArgs(execCtx)

	cmd := h.CommandRunner(ctx, "bwrap", args...)
	if execCtx.NetworkProxy != "" {
		cmd.Env = append(cmd.Environ(), "HTTP_PROXY="+execCtx.NetworkProxy, "HTTPS_PROXY="+execCtx.NetworkProxy)
	}
	return cmd.CombinedOutput()
}
