package mcp

import (
	"context"
	"encoding/json"
	"time"
)

type ExecutionResult struct {
	ToolID           string          `json:"tool_id"`
	Status           string          `json:"status"`
	ResultData       json.RawMessage `json:"result_data"`
	HybridEscalation bool            `json:"hybrid_escalation"`
	Escalation       bool            `json:"escalation"`
	ExecutedAt       time.Time       `json:"executed_at"`
}

// Ensure proper formatting of execution results
func FormatExecutionResult(toolID string, status string, resultData []byte, escalation bool) *ExecutionResult {
	return &ExecutionResult{
		ToolID:           toolID,
		Status:           status,
		ResultData:       resultData,
		HybridEscalation: escalation,
		Escalation:       escalation,
		ExecutedAt:       time.Now().UTC(),
	}
}

// Router manages routing tool executions to their respective handlers.
type Router struct {
	fsTools *FSMCPTools
}

// NewRouter creates a new Router with the provided FileSystemProvider.
func NewRouter(fsProvider FileSystemProvider) *Router {
	return &Router{
		fsTools: NewFSMCPTools(fsProvider),
	}
}

// ExecuteTool routes the tool execution to the appropriate handler.
func (r *Router) ExecuteTool(ctx context.Context, toolID string, argsRaw json.RawMessage) *ExecutionResult {
	switch toolID {
	case "read_file":
		return r.fsTools.ReadFile(ctx, argsRaw)
	case "write_file":
		return r.fsTools.WriteFile(ctx, argsRaw)
	case "list_directory":
		return r.fsTools.ListDirectory(ctx, argsRaw)
	default:
		return FormatExecutionResult(toolID, "error", []byte(`{"error":"unknown tool"}`), false)
	}
}
