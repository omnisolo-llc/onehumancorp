package orchestration

import (
	"context"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

// SandboxAdapter defines the interface for interacting with the agent sandbox
type SandboxAdapter interface {
	// EmitViolation reports a sandbox violation (e.g., unauthorized file or network access)
	EmitViolation(ctx context.Context, violationType, agentID, path string)
}

type defaultSandboxAdapter struct{}

func NewSandboxAdapter() SandboxAdapter {
	return &defaultSandboxAdapter{}
}

func (a *defaultSandboxAdapter) EmitViolation(ctx context.Context, violationType, agentID, path string) {
	telemetry.RecordSandboxViolation(ctx, violationType, agentID, path)
}
