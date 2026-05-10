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
	os.Setenv("OHC_STANDALONE", "true")
	os.Setenv("OHC_TELEMETRY_ENABLED", "false")
	defer os.Unsetenv("OHC_STANDALONE")
	defer os.Unsetenv("OHC_TELEMETRY_ENABLED")

	if isTelemetryEnabled() {
		t.Errorf("Expected telemetry to be disabled when OHC_STANDALONE=true and OHC_TELEMETRY_ENABLED=false")
	}

	// Make sure the functions don't panic or fail when disabled
	ctx := context.Background()
	if err := RecordHarnessExecutionDuration(ctx, 1.0); err != nil {
		t.Errorf("Expected no error, got %v", err)
	}
}

func TestTelemetryStandaloneOptIn(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	os.Setenv("OHC_TELEMETRY_ENABLED", "true")
	defer os.Unsetenv("OHC_STANDALONE")
	defer os.Unsetenv("OHC_TELEMETRY_ENABLED")

	if !isTelemetryEnabled() {
		t.Errorf("Expected telemetry to be enabled when OHC_STANDALONE=true and OHC_TELEMETRY_ENABLED=true")
	}
}

func TestTelemetryCloudMode(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "false")
	os.Setenv("OHC_TELEMETRY_ENABLED", "false")
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

func TestGetDeploymentModeAttribute(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	attr := getDeploymentModeAttribute()
	if attr.Value.AsString() != "standalone" {
		t.Errorf("Expected standalone mode attribute, got %v", attr.Value.AsString())
	}
	os.Unsetenv("OHC_STANDALONE")
	os.Setenv("OHC_STANDALONE", "false")
	attr = getDeploymentModeAttribute()
	if attr.Value.AsString() != "cloud" {
		t.Errorf("Expected cloud mode attribute, got %v", attr.Value.AsString())
	}
	os.Unsetenv("OHC_STANDALONE")
}

func TestRecordHarnessInitLatency(t *testing.T) {
	ctx := context.Background()
	err := RecordHarnessInitLatency(ctx, 1.0)
	if err != nil {
		t.Errorf("Expected no error, got %v", err)
	}
}

func TestRecordHarnessDbIOLatency(t *testing.T) {
	ctx := context.Background()
	err := RecordHarnessDbIOLatency(ctx, 0.5, "SELECT")
	if err != nil {
		t.Errorf("Expected no error, got %v", err)
	}
}

func TestRecordBubblewrapSpawn(t *testing.T) {
	ctx := context.Background()
	err := RecordBubblewrapSpawn(ctx)
	if err != nil {
		t.Errorf("Expected no error, got %v", err)
	}
}

func TestRecordBubblewrapExecutionLatency(t *testing.T) {
	ctx := context.Background()
	err := RecordBubblewrapExecutionLatency(ctx, 1.5)
	if err != nil {
		t.Errorf("Expected no error, got %v", err)
	}
}

func TestRecordBubblewrapViolation(t *testing.T) {
	ctx := context.Background()
	err := RecordBubblewrapViolation(ctx, "policy_denied")
	if err != nil {
		t.Errorf("Expected no error, got %v", err)
	}
}

func init() {
	// Initialize a dummy sync engine for tests to avoid nil issues
	// when bufferMetricHelper is called.
}
