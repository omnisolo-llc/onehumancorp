package telemetry

import (
	"context"
	"testing"
)

func TestRecordToolAutoCorrection(t *testing.T) {
	// Simple test to ensure it doesn't panic when globals are nil
	RecordToolAutoCorrection(context.Background(), "agent-1", "SYSTEM", true)
	RecordToolAutoCorrection(context.Background(), "agent-1", "SYSTEM", false)

	// Init and test with meter
	t.Setenv("OHC_STANDALONE", "true")
	cleanup, err := InitTelemetry()
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	defer cleanup()

	RecordToolAutoCorrection(context.Background(), "agent-1", "SYSTEM", true)
}

func TestRecordDeliberationPhaseDuration(t *testing.T) {
	RecordDeliberationPhaseDuration(context.Background(), "plan-1", "PROPOSE", 1.5)

	t.Setenv("OHC_STANDALONE", "true")
	cleanup, err := InitTelemetry()
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	defer cleanup()

	RecordDeliberationPhaseDuration(context.Background(), "plan-1", "PROPOSE", 1.5)
}
