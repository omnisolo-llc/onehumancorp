package telemetry

import (
	"context"
	"testing"
)

func TestHarnessMetrics(t *testing.T) {
	_, err := InitTelemetry()
	if err != nil {
		t.Fatalf("Failed to initialize telemetry: %v", err)
	}

	// First initialize to avoid nil panic
	m := meter
	if m == nil {
		t.Skip("meter is nil, telemetry likely opted out in this test environment")
	}

	err = initHarnessMetrics(m)
	if err != nil {
		t.Fatalf("Failed to initialize harness metrics: %v", err)
	}

	if HarnessExecutionDurationSeconds == nil {
		t.Error("Expected HarnessExecutionDurationSeconds to be initialized")
	}

	if HarnessToolInvocationsTotal == nil {
		t.Error("Expected HarnessToolInvocationsTotal to be initialized")
	}

	if HarnessViolationsTotal == nil {
		t.Error("Expected HarnessViolationsTotal to be initialized")
	}

	ctx := context.Background()

	// They should not panic when called
	RecordHarnessExecutionDuration(ctx, 1.23)
	RecordHarnessToolInvocation(ctx, "bwrap")
	RecordHarnessViolation(ctx, "test_violation")
}
