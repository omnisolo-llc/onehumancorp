package orchestration

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/src/server/telemetry"
	"github.com/prometheus/client_golang/prometheus"
)

func TestDefaultSandboxAdapter_EmitViolation(t *testing.T) {
	prometheus.DefaultRegisterer = prometheus.NewRegistry()
	telemetry.InitTelemetry()

	adapter := &DefaultSandboxAdapter{}
	ctx := context.Background()
	// Should not panic
	adapter.EmitViolation(ctx, "fs_read", "agent-123", "/etc/passwd")
}
