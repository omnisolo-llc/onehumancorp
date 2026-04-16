package mcp

import (
	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"context"
	"encoding/json"
	"errors"


)

// MCPTool defines a common interface for MCP tools.
type MCPTool interface {
	Execute(ctx context.Context, payload map[string]interface{}) (*ExecutionResult, error)
}

// CRDTProvider groups CRDT related tools.
type CRDTProvider struct {
	PushTool *CRDTPushTool
	PullTool *CRDTPullTool
}

// NewCRDTProvider creates a new CRDTProvider with its tools initialized.
func NewCRDTProvider() *CRDTProvider {
	return &CRDTProvider{
		PushTool: &CRDTPushTool{},
		PullTool: &CRDTPullTool{},
	}
}

type CRDTPushTool struct{}

func (t *CRDTPushTool) Execute(ctx context.Context, payload map[string]interface{}) (*ExecutionResult, error) {
	redactedData := telemetry.RedactInterfacePII(payload)
	resultBytes, err := json.Marshal(redactedData)
	if err != nil {
		return nil, errors.New("failed to marshal redacted payload")
	}
	return FormatExecutionResult("crdt_push", "success", resultBytes, true), nil
}

type CRDTPullTool struct{}

func (t *CRDTPullTool) Execute(ctx context.Context, payload map[string]interface{}) (*ExecutionResult, error) {
	// Mock fetching remote state
	mockData := map[string]interface{}{"crdt_state": "latest_mocked_state"}
	resultBytes, err := json.Marshal(mockData)
	if err != nil {
		return nil, errors.New("failed to marshal mock data")
	}
	return FormatExecutionResult("crdt_pull", "success", resultBytes, true), nil
}
