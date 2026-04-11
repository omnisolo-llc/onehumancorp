package mcp

import (
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

// Tool represents an MCP tool provided by a server.
type Tool struct {
	Name        string          `json:"name"`
	Description string          `json:"description"`
	InputSchema json.RawMessage `json:"inputSchema"`
}
