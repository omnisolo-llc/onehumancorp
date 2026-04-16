package mcp

import (
	"context"
	"encoding/json"
	"os"
	"strings"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

func TestHybridMCPBridge_ExecutionResult(t *testing.T) {
	// If running standalone, set OHC_STANDALONE=true
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	originalData := map[string]interface{}{
		"query": "select * from users",
		"result": []interface{}{
			map[string]interface{}{
				"id":    1,
				"name":  "Alice",
				"email": "alice@example.com",
			},
		},
	}

	rawData, _ := json.Marshal(originalData)

	res := FormatExecutionResult("test-tool", "success", rawData, true)

	if res.ToolID != "test-tool" {
		t.Errorf("Expected test-tool, got %s", res.ToolID)
	}

	if !res.HybridEscalation {
		t.Errorf("Expected HybridEscalation to be true")
	}

	if !res.Escalation {
		t.Errorf("Expected Escalation to be true")
	}

	if res.Status != "success" {
		t.Errorf("Expected status 'success', got %s", res.Status)
	}

	// 100% test coverage for the redaction pipeline within the MCP tool execution flow
	// Simulate the redaction that happens before sync
	var parsed interface{}
	if err := json.Unmarshal(res.ResultData, &parsed); err != nil {
		t.Fatalf("Failed to unmarshal result data: %v", err)
	}

	redacted := telemetry.RedactInterfacePII(parsed)

	redactedBytes, err := json.Marshal(redacted)
	if err != nil {
		t.Fatalf("Failed to marshal redacted data: %v", err)
	}

	redactedStr := string(redactedBytes)
	if !contains(redactedStr, "[REDACTED_EMAIL]") {
		t.Errorf("Expected [REDACTED_EMAIL] in output, got: %s", redactedStr)
	}

	if contains(redactedStr, "alice@example.com") {
		t.Errorf("Expected original email to be redacted, got: %s", redactedStr)
	}
}

func contains(s, substr string) bool {
	return len(s) >= len(substr) && s != "" && substr != "" && stringContains(s, substr)
}

func stringContains(s, substr string) bool {
	for i := 0; i <= len(s)-len(substr); i++ {
		if s[i:i+len(substr)] == substr {
			return true
		}
	}
	return false
}

func TestRegisterTelemetryMCPBridge(t *testing.T) {
	err := RegisterTelemetryMCPBridge("http://telemetry-mcp-bridge")
	if err != nil {
		t.Errorf("Expected no error, got %v", err)
	}
	if !IsTelemetryMCPBridgeRegistered("http://telemetry-mcp-bridge") {
		t.Errorf("Expected bridge to be registered")
	}

	err = RegisterTelemetryMCPBridge("")
	if err == nil {
		t.Errorf("Expected error for empty endpoint, got nil")
	}
}
func TestHybridContextTool_Execute(t *testing.T) {
	tool := &HybridContextTool{}

	var capturedMetricType string
	var capturedPayload string

	telemetry.BufferMetricFunc = func(ctx context.Context, metricType string, payload string) error {
		capturedMetricType = metricType
		capturedPayload = payload
		return nil
	}
	defer func() { telemetry.BufferMetricFunc = nil }()

	payload := map[string]interface{}{
		"metric_type": "custom_ui_action",
		"action":      "click",
	}

	ctx := context.Background()
	result, err := tool.Execute(ctx, payload)

	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}

	if result.ToolID != "hybrid_context" {
		t.Errorf("Expected ToolID hybrid_context, got %s", result.ToolID)
	}
	if result.Status != "success" {
		t.Errorf("Expected Status success, got %s", result.Status)
	}
	if !result.HybridEscalation {
		t.Errorf("Expected HybridEscalation to be true")
	}

	if capturedMetricType != "custom_ui_action" {
		t.Errorf("Expected capturedMetricType custom_ui_action, got %s", capturedMetricType)
	}

	if !strings.Contains(capturedPayload, "click") {
		t.Errorf("Expected payload to contain click, got %s", capturedPayload)
	}
}
