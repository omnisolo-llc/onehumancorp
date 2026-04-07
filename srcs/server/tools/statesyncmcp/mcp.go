package statesyncmcp

import (
	"context"
	"errors"
	"fmt"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// StateSyncMCP implements the MCP interface for state sync operations.
type StateSyncMCP struct {
	provider StateSyncProvider
}

// NewStateSyncMCP creates a new StateSyncMCP instance.
func NewStateSyncMCP(provider StateSyncProvider) *StateSyncMCP {
	return &StateSyncMCP{
		provider: provider,
	}
}

// Tool represents an MCP tool definition.
type Tool struct {
	Name        string `json:"name"`
	Description string `json:"description"`
	InputSchema string `json:"inputSchema"`
}

// ListTools returns the list of available tools.
func (m *StateSyncMCP) ListTools() []Tool {
	return []Tool{
		{
			Name:        "sync_local_to_cloud",
			Description: "Synchronizes local state changes to the cloud.",
			InputSchema: `{"type": "object", "properties": {}}`,
		},
		{
			Name:        "sync_cloud_to_local",
			Description: "Fetches completed tasks and state from the cloud to the local database.",
			InputSchema: `{"type": "object", "properties": {}}`,
		},
		{
			Name:        "get_sync_status",
			Description: "Retrieves the current synchronization status.",
			InputSchema: `{"type": "object", "properties": {}}`,
		},
	}
}

// CallTool executes a tool by name.
func (m *StateSyncMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return nil, errors.New("unauthorized: missing claims")
	}

	if !m.provider.IsLocal() {
		// Provide a fallback mock or no-op if running natively in the Cloud without a local SQLite counterpart
		return map[string]interface{}{
			"status": "success",
			"mode":   "cloud",
			"message": "sync operations are no-op in cloud mode",
		}, nil
	}

	switch toolName {
	case "sync_local_to_cloud":
		return m.provider.SyncUp(ctx, claims.OrganizationID)
	case "sync_cloud_to_local":
		return m.provider.SyncDown(ctx, claims.OrganizationID)
	case "get_sync_status":
		return m.provider.GetStatus(ctx, claims.OrganizationID)
	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}
