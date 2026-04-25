package orchestration

import (
	"github.com/onehumancorp/mono/src/server/telemetry"
	"context"
)

// SandboxAdapter defines the interface for an agent harness.
type SandboxAdapter interface {
	EmitViolation(ctx context.Context, violationType string, agentID string, path string)
}

// DefaultSandboxAdapter is the default implementation of SandboxAdapter.
type DefaultSandboxAdapter struct {}

// EmitViolation emits a sandbox violation metric.
func (d *DefaultSandboxAdapter) EmitViolation(ctx context.Context, violationType string, agentID string, path string) {
	telemetry.RecordSandboxViolation(ctx, violationType, agentID, path)
}
