package mcp

import (
	"context"
	"github.com/onehumancorp/mono/srcs/server/telemetry"

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


// HybridContextTool handles synchronization of UI telemetry back to the Cloud or local db.
type HybridContextTool struct{}

// Execute processes the hybrid UI interaction payload and buffers it for syncing.
func (t *HybridContextTool) Execute(ctx context.Context, params map[string]interface{}) (*ExecutionResult, error) {
	payloadBytes, err := json.Marshal(params)
	if err != nil {
		return nil, err
	}

	if telemetry.BufferMetricFunc != nil {
		_ = telemetry.BufferMetricFunc(ctx, "hybrid_context_sync", string(payloadBytes))
	}

	return FormatExecutionResult("hybrid_context", "success", []byte(`{"synced":true}`), false), nil
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
