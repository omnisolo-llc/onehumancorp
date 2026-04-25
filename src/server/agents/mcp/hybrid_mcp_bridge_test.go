package mcp

import (
	"encoding/json"
	"os"
	"testing"
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

	// The redaction now happens internally in FormatExecutionResult when escalation=true
	redactedStr := string(res.ResultData)
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