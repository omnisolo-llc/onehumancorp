package mcp

import (
	"context"
	"encoding/json"
	"fmt"
)

// SyncPayload represents the payload for synchronization.
type SyncPayload struct {
	TenantID string `json:"tenant_id"`
	AgentID  string `json:"agent_id"`
	Delta    []byte `json:"delta"`
	HLC      int64  `json:"hlc_timestamp"`
}

// HybridSyncTool provides state synchronization operations for hybrid environments.
type HybridSyncTool struct {
	LocalHLC int64
}

// NewHybridSyncTool creates a new instance of HybridSyncTool.
func NewHybridSyncTool() *HybridSyncTool {
	return &HybridSyncTool{}
}

// Execute performs the hybrid synchronization action.
func (t *HybridSyncTool) Execute(ctx context.Context, payload map[string]interface{}) (*ExecutionResult, error) {
	action, ok := payload["action"].(string)
	if !ok {
		return nil, fmt.Errorf("action is required")
	}

	switch action {
	case "sync_state":
		return t.syncState(payload)
	case "resolve_conflicts":
		return t.resolveConflicts(payload)
	default:
		return nil, fmt.Errorf("unknown action: %s", action)
	}
}

// syncState handles the sync_state operation.
func (t *HybridSyncTool) syncState(payload map[string]interface{}) (*ExecutionResult, error) {
	_, err := json.Marshal(payload)
	if err != nil {
		return nil, err
	}

	result := map[string]interface{}{
		"status": "synced",
	}
	resultBytes, _ := json.Marshal(result)

	return FormatExecutionResult("hybrid_sync", "success", resultBytes, true), nil
}

// resolveConflicts handles the resolve_conflicts operation using LWW (Last-Write-Wins) based on HLC.
func (t *HybridSyncTool) resolveConflicts(payload map[string]interface{}) (*ExecutionResult, error) {
	localHLC, _ := payload["local_hlc"].(float64)
	remoteHLC, _ := payload["remote_hlc"].(float64)

	var winner string
	if localHLC > remoteHLC {
		winner = "local"
	} else if remoteHLC > localHLC {
		winner = "remote"
	} else {
		winner = "tie_broken_by_remote" // Simplified tie-breaking
	}

	result := map[string]interface{}{
		"status": "resolved",
		"winner": winner,
	}
	resultBytes, _ := json.Marshal(result)

	return FormatExecutionResult("hybrid_sync", "success", resultBytes, true), nil
}
