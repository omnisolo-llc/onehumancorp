package harness

import (
	"context"
)

// SandboxManager defines the interface for an OS-level sandbox wrapper
// used to restrict filesystem and network access during agent execution.
type SandboxManager interface {
	// ExecuteCommand runs a command within the sandbox and returns its output (stdout, stderr) and an error.
	ExecuteCommand(ctx context.Context, command string, args ...string) (string, string, error)
}
