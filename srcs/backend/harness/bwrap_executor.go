package harness

import (
	"context"
	"os/exec"
)

type BwrapExecutor struct {}

func NewBwrapExecutor() *BwrapExecutor {
	return &BwrapExecutor{}
}

func (e *BwrapExecutor) Execute(ctx context.Context, cmd string, args ...string) ([]byte, error) {
	bwrapArgs := []string{
		"--ro-bind", "/", "/",
		"--tmpfs", "/tmp",
		"--unshare-all",
		"--share-net",
		"--",
		cmd,
	}
	bwrapArgs = append(bwrapArgs, args...)

	execCmd := exec.CommandContext(ctx, "bwrap", bwrapArgs...)
	return execCmd.CombinedOutput()
}
