//go:build darwin

package harness

import (
	"context"
	"os/exec"
)

type SandboxHarness struct{}

func NewIsolationHarness() IsolationHarness {
	return NewPermissionInterceptor(&SandboxHarness{})
}

func (h *SandboxHarness) Execute(ctx context.Context, execCtx ExecutionContext) ([]byte, error) {
	profile := "(version 1)\n(deny default)\n(allow process-exec)\n"

	// Add allowed paths
	for _, path := range execCtx.AllowedPaths {
		profile += "(allow file-read* (subpath \"" + path + "\"))\n"
		profile += "(allow file-write* (subpath \"" + path + "\"))\n"
	}

	// Allow basic system reads
	profile += "(allow file-read* (subpath \"/usr\"))\n"
	profile += "(allow file-read* (subpath \"/bin\"))\n"
	profile += "(allow file-read* (subpath \"/sbin\"))\n"

	args := []string{"-p", profile}
	args = append(args, execCtx.Command...)


	cmd := exec.CommandContext(ctx, "sandbox-exec", args...)
	if execCtx.NetworkProxy != "" {
		cmd.Env = append(cmd.Environ(), "HTTP_PROXY="+execCtx.NetworkProxy, "HTTPS_PROXY="+execCtx.NetworkProxy)
	}
	return cmd.CombinedOutput()
}
