package mcp

import (
	"context"
	"testing"
)

func TestHybridContextTool(t *testing.T) {
	tool := &HybridContextTool{}
	ctx := context.Background()
	payload := map[string]interface{}{
		"action": "click",
		"button": "submit",
	}

	result, err := tool.Execute(ctx, payload)
	if err != nil {
		t.Fatalf("expected no error, got: %v", err)
	}

	if result.ToolName != "hybrid_context" {
		t.Errorf("expected tool name 'hybrid_context', got '%s'", result.ToolName)
	}

	if result.Status != "success" {
		t.Errorf("expected status 'success', got '%s'", result.Status)
	}

	if result.IsError {
		t.Errorf("expected isError to be false, got true")
	}

	expectedResultBytes := "successfully synced hybrid context"
	if string(result.ResultBytes) != expectedResultBytes {
		t.Errorf("expected resultBytes '%s', got '%s'", expectedResultBytes, string(result.ResultBytes))
	}
}
