package harness

import (
	"context"
	"os/exec"
)

// BwrapExecutor runs commands inside a Bubblewrap sandbox.
type BwrapExecutor struct {
}

// NewBwrapExecutor creates a new executor instance.
func NewBwrapExecutor() *BwrapExecutor {
	return &BwrapExecutor{}
}

// Execute wraps a command with bwrap to enforce filesystem isolation:
// --ro-bind / / : binds root read-only
// --tmpfs /tmp  : mounts a temporary, isolated /tmp directory
func (e *BwrapExecutor) Execute(ctx context.Context, command string, args ...string) ([]byte, error) {
	bwrapArgs := []string{
		"--unshare-all",
		"--ro-bind", "/", "/",
		"--tmpfs", "/tmp",
		"--",
		command,
	}
	bwrapArgs = append(bwrapArgs, args...)

	cmd := exec.CommandContext(ctx, "bwrap", bwrapArgs...)
	return cmd.CombinedOutput()
}
