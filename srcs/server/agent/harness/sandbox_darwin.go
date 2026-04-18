//go:build darwin

package harness

import (
	"context"
	"os"
	"os/exec"
	"time"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type SandboxHarness struct{}

func NewIsolationHarness() IsolationHarness {
	return &SandboxHarness{}
}

func (h *SandboxHarness) Execute(ctx context.Context, execCtx ExecutionContext) ([]byte, error) {
	start := time.Now()
	defer func() {
		mode := "standalone"
		if os.Getenv("OHC_MULTITENANT") == "true" {
			mode = "cloud"
		}
		telemetry.RecordHarnessExecutionDuration(ctx, time.Since(start).Seconds(), mode, "sandbox_darwin")
	}()

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
	return cmd.CombinedOutput()
}
