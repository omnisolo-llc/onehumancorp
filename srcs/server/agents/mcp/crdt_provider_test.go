package mcp

import (
	"context"
	"encoding/json"
	"strings"
	"testing"
)

func TestCRDTPushTool_Execute(t *testing.T) {
	provider := NewCRDTProvider()
	tool := provider.PushTool
	payload := map[string]interface{}{
		"email": "test@example.com",
		"data":  "sensitive_task_update",
	}

	ctx := context.Background()
	result, err := tool.Execute(ctx, payload)
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}

	if result.ToolID != "crdt_push" {
		t.Errorf("Expected ToolID crdt_push, got %s", result.ToolID)
	}
	if result.Status != "success" {
		t.Errorf("Expected Status success, got %s", result.Status)
	}
	if !result.HybridEscalation {
		t.Errorf("Expected HybridEscalation to be true")
	}

	var parsedResult map[string]interface{}
	err = json.Unmarshal(result.ResultData, &parsedResult)
	if err != nil {
		t.Fatalf("Failed to parse ResultData: %v", err)
	}

	// Verify PII redaction
	if emailVal, ok := parsedResult["email"].(string); ok {
		if !strings.Contains(emailVal, "[REDACTED]") && !strings.Contains(emailVal, "[REDACTED_EMAIL]") {
			t.Errorf("Expected email to be redacted, got %v", emailVal)
		}
	} else {
		t.Errorf("Email field missing or not a string in ResultData")
	}

	if parsedResult["data"] != "sensitive_task_update" {
		t.Errorf("Expected data to be sensitive_task_update, got %v", parsedResult["data"])
	}
}

func TestCRDTPullTool_Execute(t *testing.T) {
	provider := NewCRDTProvider()
	tool := provider.PullTool
	payload := map[string]interface{}{}

	ctx := context.Background()
	result, err := tool.Execute(ctx, payload)
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}

	if result.ToolID != "crdt_pull" {
		t.Errorf("Expected ToolID crdt_pull, got %s", result.ToolID)
	}
	if result.Status != "success" {
		t.Errorf("Expected Status success, got %s", result.Status)
	}
	if !result.HybridEscalation {
		t.Errorf("Expected HybridEscalation to be true")
	}

	var parsedResult map[string]interface{}
	err = json.Unmarshal(result.ResultData, &parsedResult)
	if err != nil {
		t.Fatalf("Failed to parse ResultData: %v", err)
	}

	if parsedResult["crdt_state"] != "latest_mocked_state" {
		t.Errorf("Expected crdt_state to be latest_mocked_state, got %v", parsedResult["crdt_state"])
	}
}
