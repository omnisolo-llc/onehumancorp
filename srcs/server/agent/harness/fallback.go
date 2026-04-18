//go:build !linux && !darwin

package harness

import (
	"context"
	"os"
	"os/exec"
	"time"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type FallbackHarness struct{}

func NewIsolationHarness() IsolationHarness {
	return &FallbackHarness{}
}

func (h *FallbackHarness) Execute(ctx context.Context, execCtx ExecutionContext) ([]byte, error) {
	start := time.Now()
	defer func() {
		mode := "standalone"
		if os.Getenv("OHC_MULTITENANT") == "true" {
			mode = "cloud"
		}
		telemetry.RecordHarnessExecutionDuration(ctx, time.Since(start).Seconds(), mode, "fallback")
	}()

	cmd := exec.CommandContext(ctx, execCtx.Command[0], execCtx.Command[1:]...)
	return cmd.CombinedOutput()
}
