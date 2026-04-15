package mcp

import (
	"context"
	"encoding/json"
	"testing"
)

func TestCRDTPushTool(t *testing.T) {
	provider := NewCRDTProvider()

	payload := map[string]interface{}{
		"user": "test@example.com",
		"data": "some data",
	}

	result, err := provider.CRDTPushTool(context.Background(), payload)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if result.ToolID != "crdt_push" {
		t.Errorf("expected ToolID to be 'crdt_push', got %s", result.ToolID)
	}

	if !result.HybridEscalation {
		t.Errorf("expected HybridEscalation to be true")
	}

	var resultData map[string]interface{}
	if err := json.Unmarshal(result.ResultData, &resultData); err != nil {
		t.Fatalf("failed to unmarshal result data: %v", err)
	}

	if resultData["user"] != "[REDACTED_EMAIL]" {
		t.Errorf("expected user to be redacted, got %v", resultData["user"])
	}
}

func TestCRDTPullTool(t *testing.T) {
	provider := NewCRDTProvider()

	result, err := provider.CRDTPullTool(context.Background())
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if result.ToolID != "crdt_pull" {
		t.Errorf("expected ToolID to be 'crdt_pull', got %s", result.ToolID)
	}

	if !result.HybridEscalation {
		t.Errorf("expected HybridEscalation to be true")
	}

	var resultData map[string]interface{}
	if err := json.Unmarshal(result.ResultData, &resultData); err != nil {
		t.Fatalf("failed to unmarshal result data: %v", err)
	}

	if resultData["status"] != "synced" {
		t.Errorf("expected status to be 'synced', got %v", resultData["status"])
	}
}
