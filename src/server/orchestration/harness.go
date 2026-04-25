package orchestration

import (
	"context"
	"errors"

	"github.com/onehumancorp/mono/src/server/agents/harness"
	"github.com/onehumancorp/mono/src/server/telemetry"
)

// SandboxAdapter defines the interface for an agent harness.
type SandboxAdapter interface {
	EmitViolation(ctx context.Context, violationType string, agentID string, path string)
}

// DefaultSandboxAdapter is the default implementation of SandboxAdapter.
type DefaultSandboxAdapter struct{}

// EmitViolation emits a sandbox violation metric.
func (d *DefaultSandboxAdapter) EmitViolation(ctx context.Context, violationType string, agentID string, path string) {
	telemetry.RecordSandboxViolation(ctx, violationType, agentID, path)
}

type HarnessGateway struct {
	Backends map[string]harness.HarnessBackend
}

func NewHarnessGateway() *HarnessGateway {
	return &HarnessGateway{
		Backends: map[string]harness.HarnessBackend{
			"local":  &harness.LocalBackend{Isolation: harness.NewIsolationHarness()},
			"docker": &harness.DockerBackend{},
		},
	}
}

func (g *HarnessGateway) Execute(ctx context.Context, backendType string, execCtx harness.ExecutionContext) ([]byte, error) {
	backend, ok := g.Backends[backendType]
	if !ok {
		return nil, errors.New("unsupported backend type")
	}
	return backend.Execute(ctx, execCtx)
}
