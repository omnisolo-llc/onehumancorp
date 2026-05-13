package mcp

import (
	"context"
	"encoding/json"
	"fmt"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type ExecutionResult struct {
	ToolName    string
	Status      string
	ResultBytes []byte
	IsError     bool
}

func FormatExecutionResult(toolName, status string, resultBytes []byte, isError bool) *ExecutionResult {
	return &ExecutionResult{
		ToolName:    toolName,
		Status:      status,
		ResultBytes: resultBytes,
		IsError:     isError,
	}
}

type MCPTool interface {
	Execute(ctx context.Context, payload map[string]interface{}) (*ExecutionResult, error)
}

type HybridContextTool struct{}

func (t *HybridContextTool) Execute(ctx context.Context, payload map[string]interface{}) (*ExecutionResult, error) {
	payloadBytes, err := json.Marshal(payload)
	if err != nil {
		return nil, fmt.Errorf("failed to marshal payload: %w", err)
	}

	err = telemetry.BufferMetricFunc(ctx, "hybrid_ui_context", string(payloadBytes))
	if err != nil {
		return nil, fmt.Errorf("failed to buffer metric: %w", err)
	}

	resultBytes := []byte("successfully synced hybrid context")
	return FormatExecutionResult("hybrid_context", "success", resultBytes, false), nil
}
