package mcp

import (
	"context"
	"encoding/json"
	"testing"
)

func TestHybridSyncTool_Execute_SyncState(t *testing.T) {
	tool := NewHybridSyncTool()
	payload := map[string]interface{}{
		"action": "sync_state",
		"tenant_id": "tenant-1",
		"agent_id": "agent-1",
	}

	res, err := tool.Execute(context.Background(), payload)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if res.ToolID != "hybrid_sync" {
		t.Errorf("expected tool_id hybrid_sync, got %s", res.ToolID)
	}

	var resultData map[string]interface{}
	if err := json.Unmarshal(res.ResultData, &resultData); err != nil {
		t.Fatalf("failed to unmarshal result data: %v", err)
	}

	if status, ok := resultData["status"].(string); !ok || status != "synced" {
		t.Errorf("expected status 'synced', got %v", resultData["status"])
	}
}

func TestHybridSyncTool_Execute_ResolveConflicts_LocalWins(t *testing.T) {
	tool := NewHybridSyncTool()
	payload := map[string]interface{}{
		"action":     "resolve_conflicts",
		"local_hlc":  float64(200),
		"remote_hlc": float64(100),
	}

	res, err := tool.Execute(context.Background(), payload)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	var resultData map[string]interface{}
	json.Unmarshal(res.ResultData, &resultData)

	if winner, ok := resultData["winner"].(string); !ok || winner != "local" {
		t.Errorf("expected winner 'local', got %v", resultData["winner"])
	}
}

func TestHybridSyncTool_Execute_ResolveConflicts_RemoteWins(t *testing.T) {
	tool := NewHybridSyncTool()
	payload := map[string]interface{}{
		"action":     "resolve_conflicts",
		"local_hlc":  float64(100),
		"remote_hlc": float64(200),
	}

	res, err := tool.Execute(context.Background(), payload)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	var resultData map[string]interface{}
	json.Unmarshal(res.ResultData, &resultData)

	if winner, ok := resultData["winner"].(string); !ok || winner != "remote" {
		t.Errorf("expected winner 'remote', got %v", resultData["winner"])
	}
}

func TestHybridSyncTool_Execute_ResolveConflicts_Tie(t *testing.T) {
	tool := NewHybridSyncTool()
	payload := map[string]interface{}{
		"action":     "resolve_conflicts",
		"local_hlc":  float64(100),
		"remote_hlc": float64(100),
	}

	res, err := tool.Execute(context.Background(), payload)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	var resultData map[string]interface{}
	json.Unmarshal(res.ResultData, &resultData)

	if winner, ok := resultData["winner"].(string); !ok || winner != "tie_broken_by_remote" {
		t.Errorf("expected winner 'tie_broken_by_remote', got %v", resultData["winner"])
	}
}

func TestHybridSyncTool_Execute_MissingAction(t *testing.T) {
	tool := NewHybridSyncTool()
	payload := map[string]interface{}{
		"tenant_id": "tenant-1",
	}

	_, err := tool.Execute(context.Background(), payload)
	if err == nil {
		t.Errorf("expected error for missing action, got nil")
	}
}

func TestHybridSyncTool_Execute_UnknownAction(t *testing.T) {
	tool := NewHybridSyncTool()
	payload := map[string]interface{}{
		"action": "unknown",
	}

	_, err := tool.Execute(context.Background(), payload)
	if err == nil {
		t.Errorf("expected error for unknown action, got nil")
	}
}
