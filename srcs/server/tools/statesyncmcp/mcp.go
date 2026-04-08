package statesyncmcp

import (
	"context"
	"errors"
	"fmt"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// StateSyncProvider abstracts the local-to-cloud synchronization logic.
type StateSyncProvider interface {
	SyncUp(ctx context.Context, claims *auth.Claims) (map[string]interface{}, error)
	SyncDown(ctx context.Context, claims *auth.Claims) (map[string]interface{}, error)
	GetStatus(ctx context.Context, claims *auth.Claims) (map[string]interface{}, error)
}

// Tool represents an MCP tool definition.
type Tool struct {
	Name        string `json:"name"`
	Description string `json:"description"`
	InputSchema string `json:"inputSchema"`
}

// StateSyncMCP implements the MCP interface for local-to-cloud state sync.
type StateSyncMCP struct {
	provider StateSyncProvider
	isLocal  bool
}

// NewStateSyncMCP creates a new StateSyncMCP instance.
func NewStateSyncMCP(provider StateSyncProvider, isLocal bool) *StateSyncMCP {
	return &StateSyncMCP{
		provider: provider,
		isLocal:  isLocal,
	}
}

// ListTools returns the list of available tools.
func (m *StateSyncMCP) ListTools() []Tool {
	return []Tool{
		{
			Name:        "sync_local_to_cloud",
			Description: "Synchronize local offline state to the cloud.",
			InputSchema: `{"type": "object", "properties": {}}`,
		},
		{
			Name:        "sync_cloud_to_local",
			Description: "Fetch completed tasks or delegations from the cloud.",
			InputSchema: `{"type": "object", "properties": {}}`,
		},
		{
			Name:        "get_sync_status",
			Description: "Get the current synchronization status.",
			InputSchema: `{"type": "object", "properties": {}}`,
		},
	}
}

// CallTool executes a tool by name.
func (m *StateSyncMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	if !m.isLocal {
		return map[string]interface{}{
			"status":  "skipped",
			"message": "Not running in Standalone/Local mode. Sync is a no-op.",
		}, nil
	}

	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return nil, errors.New("unauthorized: missing claims")
	}

	switch toolName {
	case "sync_local_to_cloud":
		return m.provider.SyncUp(ctx, claims)
	case "sync_cloud_to_local":
		return m.provider.SyncDown(ctx, claims)
	case "get_sync_status":
		return m.provider.GetStatus(ctx, claims)
	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}
