package telemetry_test

import (
	"context"
	"encoding/json"
	"os"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

func TestStandaloneTelemetryPIIRedaction(t *testing.T) {
	originalBufferFunc := telemetry.BufferMetricFunc
	defer func() { telemetry.BufferMetricFunc = originalBufferFunc }()

	var receivedPayload string
	var receivedMetricType string

	telemetry.BufferMetricFunc = func(ctx context.Context, metricType string, payload string) error {
		receivedMetricType = metricType
		receivedPayload = payload
		return nil
	}

	ctx := context.Background()
	piiData := "testuser@example.com"

	telemetry.RecordCacheMiss(ctx, "fetching "+piiData, "redis")

	if receivedMetricType != "cache_miss" {
		t.Errorf("expected metric type 'cache_miss', got %q", receivedMetricType)
	}

	var parsedPayload map[string]interface{}
	err := json.Unmarshal([]byte(receivedPayload), &parsedPayload)
	if err != nil {
		t.Fatalf("failed to unmarshal payload: %v", err)
	}

	opVal, ok := parsedPayload["operation"].(string)
	if !ok {
		t.Fatalf("expected string operation in payload, got %v", parsedPayload["operation"])
	}

	if opVal != "fetching [REDACTED_EMAIL]" {
		t.Errorf("expected operation to be 'fetching [REDACTED_EMAIL]', got %q", opVal)
	}
}

func TestStandaloneTelemetryOptIn(t *testing.T) {
	// By default, InitTelemetry returns early if telemetry is not explicitly requested
	// and we are in standalone mode. Let's verify this behavior.

	// Setup env vars to simulate standalone mode where telemetry is NOT enabled
	os.Setenv("OHC_MULTITENANT", "false")
	os.Unsetenv("OHC_TELEMETRY_ENABLED")

	// Restore original BufferMetricFunc after test
	originalBufferFunc := telemetry.BufferMetricFunc
	defer func() { telemetry.BufferMetricFunc = originalBufferFunc }()

	// We'll set a mock func to prove it gets overwritten by InitTelemetry if disabled
	telemetry.BufferMetricFunc = func(ctx context.Context, metricType string, payload string) error {
		return nil
	}

	shutdown, err := telemetry.InitTelemetry()
	if err != nil {
		t.Fatalf("unexpected error during init: %v", err)
	}
	defer shutdown()

	// Assert that BufferMetricFunc is set to nil (opt-in enforced)
	if telemetry.BufferMetricFunc != nil {
		t.Errorf("expected BufferMetricFunc to be nil in standalone opt-out mode, got %v", telemetry.BufferMetricFunc)
	}

	// Now try with telemetry enabled
	os.Setenv("OHC_TELEMETRY_ENABLED", "true")

	// Set mock func again
	telemetry.BufferMetricFunc = func(ctx context.Context, metricType string, payload string) error {
		return nil
	}

	shutdown2, err2 := telemetry.InitTelemetry()
	if err2 != nil {
		t.Fatalf("unexpected error during init with telemetry enabled: %v", err2)
	}
	defer shutdown2()

	// Assert BufferMetricFunc is NOT set to nil, it should remain whatever the wrapper set it to,
	// or at least not be forced to nil by the privacy guard. (Actually InitTelemetry doesn't overwrite
	// it if it passes the privacy guard, so it should still be our mock function).
	if telemetry.BufferMetricFunc == nil {
		t.Errorf("expected BufferMetricFunc to NOT be nil in standalone opt-in mode")
	}
}
