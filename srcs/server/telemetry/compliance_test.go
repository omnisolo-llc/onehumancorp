package telemetry_test

import (
	"context"
	"encoding/json"
	"strings"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
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

func TestBufferMetricFuncRedactsPII(t *testing.T) {
	var capturedPayload string
	telemetry.BufferMetricFunc = func(ctx context.Context, metricType string, payload string) error {
		capturedPayload = payload
		return nil
	}
	defer func() { telemetry.BufferMetricFunc = nil }()

	ctx := context.Background()
	taskID := "task-123 user@example.com"
	errStr := "error with ssn 123-45-6789"

	telemetry.RecordTaskFailed(ctx, taskID, errStr)

	if capturedPayload == "" {
		t.Fatalf("BufferMetricFunc was not called")
	}

	var payloadMap map[string]interface{}
	err := json.Unmarshal([]byte(capturedPayload), &payloadMap)
	if err != nil {
		t.Fatalf("Failed to unmarshal payload: %v", err)
	}

	taskIDVal, ok := payloadMap["task_id"].(string)
	if !ok {
		t.Fatalf("task_id is missing or not a string")
	}
	if strings.Contains(taskIDVal, "user@example.com") {
		t.Errorf("Email was not redacted in payload: %s", taskIDVal)
	}
	if !strings.Contains(taskIDVal, "[REDACTED_EMAIL]") {
		t.Errorf("Expected redacted email in payload: %s", taskIDVal)
	}

	errorVal, ok := payloadMap["error"].(string)
	if !ok {
		t.Fatalf("error is missing or not a string")
	}
	if strings.Contains(errorVal, "123-45-6789") {
		t.Errorf("SSN was not redacted in payload: %s", errorVal)
	}
	if !strings.Contains(errorVal, "[REDACTED_SSN]") {
		t.Errorf("Expected redacted SSN in payload: %s", errorVal)
	}
}
