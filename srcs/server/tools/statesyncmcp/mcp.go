package statesyncmcp

import (
	"context"
	"errors"
	"fmt"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
)

// StateSyncProvider defines the interface for synchronizing local and cloud state.
type StateSyncProvider interface {
	SyncUp(ctx context.Context, claims *auth.Claims) (interface{}, error)
	SyncDown(ctx context.Context, claims *auth.Claims) (interface{}, error)
	GetStatus(ctx context.Context, claims *auth.Claims) (interface{}, error)
}

// StateSyncMCP implements the MCP interface for state synchronization.
type StateSyncMCP struct {
	dbProvider   db.Provider
	syncProvider StateSyncProvider
}

// NewStateSyncMCP creates a new StateSyncMCP instance.
func NewStateSyncMCP(dbProvider db.Provider, syncProvider StateSyncProvider) *StateSyncMCP {
	return &StateSyncMCP{
		dbProvider:   dbProvider,
		syncProvider: syncProvider,
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
			Description: "Synchronizes local offline state up to the cloud backend.",
			InputSchema: `{"type": "object", "properties": {}}`,
		},
		{
			Name:        "sync_cloud_to_local",
			Description: "Synchronizes cloud state down to the local offline backend.",
			InputSchema: `{"type": "object", "properties": {}}`,
		},
		{
			Name:        "get_sync_status",
			Description: "Gets the current synchronization status between local and cloud.",
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

	// If we are not in local/standalone mode, we mock/noop since we are already in the cloud.
	if !m.dbProvider.IsSQLite() {
		return map[string]interface{}{
			"status":  "success",
			"message": "already in cloud mode, no sync needed",
			"mode":    "cloud",
		}, nil
	}

	switch toolName {
	case "sync_local_to_cloud":
		return m.syncProvider.SyncUp(ctx, claims)
	case "sync_cloud_to_local":
		return m.syncProvider.SyncDown(ctx, claims)
	case "get_sync_status":
		return m.syncProvider.GetStatus(ctx, claims)
	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}
