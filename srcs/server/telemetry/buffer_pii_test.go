package telemetry

import (
	"context"
	"strings"
	"testing"
)

func TestBufferMetricFunc_PIIRedaction(t *testing.T) {
	var capturedPayload string
	BufferMetricFunc = func(ctx context.Context, metricType string, payload string) error {
		capturedPayload = payload
		return nil
	}
	defer func() { BufferMetricFunc = nil }()

	ctx := context.Background()

	// Capture originals and mock meter
	origAgentCalls := agentApiCallsCounter
	mockM := &mockMeter{}
	_ = InitWithMeter(mockM)

	// In `RecordAgentApiCall`, it just puts agent_id, role, api. Let's make one of them a PII string.
	RecordAgentApiCall(ctx, "agent1@example.com", "role", "api-call")

	if !strings.Contains(capturedPayload, "[REDACTED_EMAIL]") {
		t.Errorf("Expected PII to be redacted in payload, got %v", capturedPayload)
	}

	if strings.Contains(capturedPayload, "agent1@example.com") {
		t.Errorf("Expected email to not be in payload, got %v", capturedPayload)
	}

	// Restore
	agentApiCallsCounter = origAgentCalls
}
