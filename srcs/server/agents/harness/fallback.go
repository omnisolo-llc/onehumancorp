//go:build !linux && !darwin

package harness

import (
	"bytes"
	"context"
	"os/exec"
)

type FallbackHarness struct{}

func NewIsolationHarness() IsolationHarness {
	return NewPermissionInterceptor(&FallbackHarness{})
}

func (h *FallbackHarness) Execute(ctx context.Context, execCtx ExecutionContext) ([]byte, []byte, error) {

	cmd := exec.CommandContext(ctx, execCtx.Command[0], execCtx.Command[1:]...)
	if execCtx.NetworkProxy != "" {
		cmd.Env = append(cmd.Environ(), "HTTP_PROXY="+execCtx.NetworkProxy, "HTTPS_PROXY="+execCtx.NetworkProxy)
	}

	var stdout, stderr bytes.Buffer
	cmd.Stdout = &stdout
	cmd.Stderr = &stderr

	err := cmd.Run()
	return stdout.Bytes(), stderr.Bytes(), err
}
