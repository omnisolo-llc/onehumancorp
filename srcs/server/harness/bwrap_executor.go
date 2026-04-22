package harness

import (
	"bytes"
	"context"
	"os/exec"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type BwrapExecutor struct {
	AgentID string
}

func NewBwrapExecutor(agentID string) *BwrapExecutor {
	return &BwrapExecutor{
		AgentID: agentID,
	}
}

func (e *BwrapExecutor) Execute(ctx context.Context, cmd []string) ([]byte, error) {
	socketPath := "/tmp/ohc_proxy_" + e.AgentID + ".sock"
	args := []string{
		"--ro-bind", "/", "/",
		"--tmpfs", "/tmp",
		"--unshare-net",
		"--bind", socketPath, socketPath,
		"--",
	}
	args = append(args, cmd...)

	bwrapCmd := exec.CommandContext(ctx, "bwrap", args...)
	bwrapCmd.Env = []string{
		"PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin",
		"HTTP_PROXY=unix://" + socketPath,
		"HTTPS_PROXY=unix://" + socketPath,
	}

	var outBuf bytes.Buffer
	var errBuf bytes.Buffer
	bwrapCmd.Stdout = &outBuf
	bwrapCmd.Stderr = &errBuf

	err := bwrapCmd.Run()
	if err != nil {
		if exitError, ok := err.(*exec.ExitError); ok {
			if exitError.ExitCode() == 1 && bytes.Contains(errBuf.Bytes(), []byte("bwrap:")) {
				telemetry.RecordSandboxViolation(ctx, "bwrap_violation", e.AgentID, "")
			}
		} else {
			if !bytes.Contains([]byte(err.Error()), []byte("executable file not found")) {
				telemetry.RecordSandboxViolation(ctx, "bwrap_violation", e.AgentID, "")
			}
		}
	}

	return outBuf.Bytes(), err
}
