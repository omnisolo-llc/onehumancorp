package orchestration

import (
	"context"
	"testing"
	"encoding/json"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

func TestToolParameterAutoCorrection_Telemetry(t *testing.T) {
	hub := NewHub()
	defer hub.Close()

	var recordCalled bool
	telemetry.BufferMetricFunc = func(ctx context.Context, metricType string, payload string) error {
		if metricType == "ohc_tool_autocorrection_total" {
			recordCalled = true
		}
		return nil
	}
	defer func() { telemetry.BufferMetricFunc = nil }()

	agentID := "test-agent"
	eventID := "event-123"

	payloadObj := map[string]interface{}{
		"id": "100",
		"name": "test",
	}
	payload, _ := json.Marshal(payloadObj)

	err := hub.ToolParameterAutoCorrection(eventID, agentID, payload)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if !recordCalled {
		t.Errorf("expected RecordToolAutoCorrection to be called")
	}
}
