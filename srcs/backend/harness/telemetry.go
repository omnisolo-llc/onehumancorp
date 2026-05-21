package harness

import "context"

// Global definition of SandboxTelemetryEmitter so it can be implemented by the caller or shared
type SandboxTelemetryEmitter interface {
	RecordViolation(ctx context.Context, violationType, details string) error
}

// Bwrap executor should also accept the telemetry emitter to record file access denials
