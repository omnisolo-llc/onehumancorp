//go:build !linux && !darwin

package harness

import (
	"context"
	"os/exec"
)

type FallbackHarness struct{}

func NewIsolationHarness() IsolationHarness {
	return &FallbackHarness{}
}

func (h *FallbackHarness) Execute(ctx context.Context, execCtx ExecutionContext) ([]byte, error) {
	cmd := exec.CommandContext(ctx, execCtx.Command[0], execCtx.Command[1:]...)
	return cmd.CombinedOutput()
}
