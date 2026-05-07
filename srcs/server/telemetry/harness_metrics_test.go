package telemetry

import (
	"context"
	"os"
	"testing"
)

func TestRecordHarnessExecutionDuration(t *testing.T) {
	ctx := context.Background()
	err := RecordHarnessExecutionDuration(ctx, 1500.0)
	if err != nil {
		t.Errorf("Expected no error, got %v", err)
	}
}

func TestTelemetryStandaloneOptOut(t *testing.T) {
	os.Setenv("STANDALONE_MODE", "true")
	os.Setenv("OHC_TELEMETRY_ENABLED", "false")
	defer os.Unsetenv("STANDALONE_MODE")
	defer os.Unsetenv("OHC_TELEMETRY_ENABLED")

	if isTelemetryEnabled() {
		t.Errorf("Expected telemetry to be disabled when STANDALONE_MODE=true and OHC_TELEMETRY_ENABLED=false")
	}

	// Make sure the functions don't panic or fail when disabled
	ctx := context.Background()
	if err := RecordHarnessExecutionDuration(ctx, 1.0); err != nil {
		t.Errorf("Expected no error, got %v", err)
	}
}

func TestTelemetryStandaloneOptIn(t *testing.T) {
	os.Setenv("STANDALONE_MODE", "true")
	os.Setenv("OHC_TELEMETRY_ENABLED", "true")
	defer os.Unsetenv("STANDALONE_MODE")
	defer os.Unsetenv("OHC_TELEMETRY_ENABLED")

	if !isTelemetryEnabled() {
		t.Errorf("Expected telemetry to be enabled when STANDALONE_MODE=true and OHC_TELEMETRY_ENABLED=true")
	}
}

func TestTelemetryCloudMode(t *testing.T) {
	os.Setenv("STANDALONE_MODE", "false")
	os.Setenv("OHC_STANDALONE", "false")
	os.Setenv("OHC_TELEMETRY_ENABLED", "false")
	defer os.Unsetenv("STANDALONE_MODE")
	defer os.Unsetenv("OHC_STANDALONE")
	defer os.Unsetenv("OHC_TELEMETRY_ENABLED")

	if !isTelemetryEnabled() {
		t.Errorf("Expected telemetry to be unconditionally enabled in Cloud mode")
	}
}

func TestRecordHarnessToolInvocation(t *testing.T) {
	ctx := context.Background()
	err := RecordHarnessToolInvocation(ctx, "test_tool")
	if err != nil {
		t.Errorf("Expected no error, got %v", err)
	}
}

func TestRecordHarnessViolation(t *testing.T) {
	ctx := context.Background()
	err := RecordHarnessViolation(ctx, "timeout")
	if err != nil {
		t.Errorf("Expected no error, got %v", err)
	}
}
