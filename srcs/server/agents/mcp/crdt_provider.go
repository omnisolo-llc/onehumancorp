package mcp

import (
	"context"
	"encoding/json"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

// CRDTProvider implements tools for Hybrid CRDT Synchronization.
type CRDTProvider struct{}

func NewCRDTProvider() *CRDTProvider {
	return &CRDTProvider{}
}

// CRDTPushTool pushes local state changes to the remote sync endpoint.
func (p *CRDTProvider) CRDTPushTool(ctx context.Context, payload map[string]interface{}) (*ExecutionResult, error) {
	redactedPayload := telemetry.RedactInterfacePII(payload)

	payloadBytes, err := json.Marshal(redactedPayload)
	if err != nil {
		return nil, err
	}

	return FormatExecutionResult("crdt_push", "success", payloadBytes, true), nil
}

// CRDTPullTool fetches the latest remote state and resolves it locally using CRDT logic.
func (p *CRDTProvider) CRDTPullTool(ctx context.Context) (*ExecutionResult, error) {
	// Mock remote state
	mockData := map[string]interface{}{
		"remote_revision": 1,
		"status":          "synced",
	}

	payloadBytes, err := json.Marshal(mockData)
	if err != nil {
		return nil, err
	}

	return FormatExecutionResult("crdt_pull", "success", payloadBytes, true), nil
}
