package telemetry

import (
	"context"
	"testing"
)

func TestTelemetryCompliance_StandaloneOptIn(t *testing.T) {
	// Setup
	t.Setenv("OHC_STANDALONE", "true")
	t.Setenv("OHC_TELEMETRY_ENABLED", "")

	// Cache original value to restore later
	originalBufferMetricFunc := BufferMetricFunc
	defer func() {
		BufferMetricFunc = originalBufferMetricFunc
	}()

	// Action
	_, err := InitTelemetry()
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}

	// Verify that BufferMetricFunc is disabled in standalone mode by default
	if BufferMetricFunc != nil {
		t.Errorf("BufferMetricFunc should be nil in standalone mode when telemetry is not explicitly enabled")
	}

	// Setup explicitly enabled telemetry
	t.Setenv("OHC_TELEMETRY_ENABLED", "true")

	// Needs custom test init so that BufferMetricFunc is set for the test
	BufferMetricFunc = func(ctx context.Context, metricType string, payload string) error { return nil }

	// Action again
	_, err = InitTelemetry()
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}

	// Verify that BufferMetricFunc is NOT disabled when telemetry is explicitly enabled
	if BufferMetricFunc == nil {
		t.Errorf("BufferMetricFunc should not be nil in standalone mode when telemetry is explicitly enabled")
	}
}

func TestTelemetryCompliance_PIIRedaction(t *testing.T) {
	// Test PIIRedactingHandler and RedactInterfacePII
	input := map[string]interface{}{
		"email": "user@example.com",
		"phone": "555-123-4567",
		"ssn":   "000-00-0000",
		"safe":  "data",
	}

	redacted := RedactInterfacePII(input)
	redactedMap, ok := redacted.(map[string]interface{})
	if !ok {
		t.Fatalf("Expected map[string]interface{}, got %T", redacted)
	}

	if redactedMap["email"] != "[REDACTED_EMAIL]" {
		t.Errorf("Email was not properly redacted. Got: %v", redactedMap["email"])
	}

	if redactedMap["phone"] != "[REDACTED_PHONE]" {
		t.Errorf("Phone was not properly redacted. Got: %v", redactedMap["phone"])
	}

	if redactedMap["ssn"] != "[REDACTED_SSN]" {
		t.Errorf("SSN was not properly redacted. Got: %v", redactedMap["ssn"])
	}

	if redactedMap["safe"] != "data" {
		t.Errorf("Safe data was modified. Got: %v", redactedMap["safe"])
	}
}
