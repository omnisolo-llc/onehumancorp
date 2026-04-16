package mcp

import (
	"context"
	"encoding/json"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

func TestHybridContextTool(t *testing.T) {
	called := false
	originalBuffer := telemetry.BufferMetricFunc
	defer func() {
		telemetry.BufferMetricFunc = originalBuffer
	}()

	telemetry.BufferMetricFunc = func(ctx context.Context, metricType string, payload string) error {
		if metricType != "hybrid_context_sync" {
			t.Errorf("Expected metricType 'hybrid_context_sync', got %s", metricType)
		}
		called = true
		return nil
	}

	tool := &HybridContextTool{}
	params := map[string]interface{}{
		"action":    "click",
		"component": "button",
	}

	result, err := tool.Execute(context.Background(), params)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	if !called {
		t.Fatal("Expected telemetry.BufferMetricFunc to be called")
	}

	if result.Status != "success" {
		t.Errorf("Expected status 'success', got %s", result.Status)
	}

	var data map[string]interface{}
	if err := json.Unmarshal(result.ResultData, &data); err != nil {
		t.Fatalf("Failed to unmarshal result data: %v", err)
	}
	if data["synced"] != true {
		t.Errorf("Expected synced=true in result data")
	}
}
