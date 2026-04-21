package mcp

import (
	"context"
	"encoding/json"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

func TestHybridContextTool(t *testing.T) {
	tool := &HybridContextTool{}
	ctx := context.Background()

	metricCalled := false
	var recordedMetric string
	var recordedPayload string
	telemetry.BufferMetricFunc = func(ctx context.Context, metricType string, payload string) error {
		metricCalled = true
		recordedMetric = metricType
		recordedPayload = payload
		return nil
	}
	defer func() { telemetry.BufferMetricFunc = nil }()

	payload := map[string]interface{}{
		"action": "click",
		"widget": "button",
	}

	res, err := tool.Execute(ctx, payload)
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}

	if !metricCalled {
		t.Errorf("Expected telemetry.BufferMetricFunc to be called")
	}
	if recordedMetric != "hybrid_ui_context" {
		t.Errorf("Expected metric 'hybrid_ui_context', got %s", recordedMetric)
	}

	expectedPayload, _ := json.Marshal(payload)
	if string(expectedPayload) != recordedPayload {
		t.Errorf("Expected payload %s, got %s", string(expectedPayload), recordedPayload)
	}

	if res == nil {
		t.Fatalf("Expected execution result, got nil")
	}
	if res.ToolID != "hybrid_context" {
		t.Errorf("Expected ToolID 'hybrid_context', got %s", res.ToolID)
	}
	if res.Status != "success" {
		t.Errorf("Expected status 'success', got %s", res.Status)
	}
	if res.HybridEscalation != false {
		t.Errorf("Expected HybridEscalation to be false, got %v", res.HybridEscalation)
	}
}
