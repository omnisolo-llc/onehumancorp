package orchestration

import "context"

type SandboxTelemetryEmitter interface {
	EmitViolation(ctx context.Context, violationType, agentID, path string)
}
