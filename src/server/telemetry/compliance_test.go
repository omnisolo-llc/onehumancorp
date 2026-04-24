package telemetry_test

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/src/server/telemetry"
)

func TestStandaloneTelemetryCompliance(t *testing.T) {
	// Test Opt-Out
	t.Setenv("OHC_MULTITENANT", "false")
	t.Setenv("OHC_TELEMETRY_ENABLED", "false")

	telemetry.BufferMetricFunc = func(ctx context.Context, metricType string, payload string) error {
		return nil
	}

	cleanup, err := telemetry.InitTelemetry()
	if err != nil {
		t.Fatalf("InitTelemetry failed: %v", err)
	}
	if cleanup != nil {
		defer cleanup()
	}

	if telemetry.BufferMetricFunc != nil {
		t.Errorf("BufferMetricFunc should be nil when standalone telemetry is disabled")
	}

	// Test Opt-In
	t.Setenv("OHC_TELEMETRY_ENABLED", "true")

	// We don't want to re-init full prometheus if not needed, just testing logic.
	// But InitTelemetry creates a new registry, so it's safe.
	cleanup2, err2 := telemetry.InitTelemetry()
	if err2 != nil {
		t.Fatalf("InitTelemetry failed: %v", err2)
	}
	if cleanup2 != nil {
		defer cleanup2()
	}

	// BufferMetricFunc is not explicitly set by InitTelemetry in true case unless provided earlier.
	// Let's set it and verify it's NOT cleared.
	telemetry.BufferMetricFunc = func(ctx context.Context, metricType string, payload string) error {
		return nil
	}
	cleanup3, err3 := telemetry.InitTelemetry()
	if err3 != nil {
		t.Fatalf("InitTelemetry failed: %v", err3)
	}
	if cleanup3 != nil {
		defer cleanup3()
	}
	if telemetry.BufferMetricFunc == nil {
		t.Errorf("BufferMetricFunc should NOT be nil when standalone telemetry is enabled")
	}
}

