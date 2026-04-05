package telemetry

import (
	"context"
	"encoding/json"
	"strings"
	"testing"
)

func TestBufferMetricPIIRedaction(t *testing.T) {
	var capturedPayload string
	BufferMetricFunc = func(ctx context.Context, metricType string, payload string) error {
		capturedPayload = payload
		return nil
	}
	defer func() { BufferMetricFunc = nil }()

	mockM := &mockMeter{}
	_ = InitWithMeter(mockM)

	ctx := context.Background()
	agentID := "agent-jules-smith@example.com"
	role := "tester"
	api := "some-api-123-45-6789"

	RecordAgentApiCall(ctx, agentID, role, api)

	var payloadMap map[string]interface{}
	if err := json.Unmarshal([]byte(capturedPayload), &payloadMap); err != nil {
		t.Fatalf("Failed to unmarshal payload: %v", err)
	}

	if strings.Contains(capturedPayload, "smith@example.com") {
		t.Errorf("PII not redacted: %v", capturedPayload)
	}

	if strings.Contains(capturedPayload, "123-45-6789") {
		t.Errorf("PII not redacted: %v", capturedPayload)
	}

	agentIDStr, _ := payloadMap["agent_id"].(string)
	if agentIDStr != "[REDACTED_EMAIL]" {
		t.Errorf("Expected redacted agent_id, got %v", agentIDStr)
	}
}
