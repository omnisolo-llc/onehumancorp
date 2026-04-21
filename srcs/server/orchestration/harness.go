package orchestration

import (
	"context"
)

// SandboxAdapter defines the interface for an agent harness.
type SandboxAdapter interface {
	EmitViolation(ctx context.Context, violationType string, agentID string, path string)
}
