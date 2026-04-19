package telemetry

import (
	"context"
	"strings"
	"testing"
)

func TestBufferMetricFuncPII(t *testing.T) {
	var capturedPayload string
	BufferMetricFunc = func(ctx context.Context, metricType string, payload string) error {
		capturedPayload = payload
		return nil
	}
	defer func() { BufferMetricFunc = nil }()

	ctx := context.Background()
	mockM := &mockMeter{}
	_ = InitWithMeter(mockM)

	RecordTaskFailed(ctx, "task1", "user email is test@example.com")

	if !strings.Contains(capturedPayload, "[REDACTED_EMAIL]") {
		t.Errorf("expected payload to be redacted, got: %s", capturedPayload)
	}
	if strings.Contains(capturedPayload, "test@example.com") {
		t.Errorf("expected payload to not contain email, got: %s", capturedPayload)
	}
}
