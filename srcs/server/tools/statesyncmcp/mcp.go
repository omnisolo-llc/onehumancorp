package statesyncmcp

import (
	"context"
	"errors"
	"fmt"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
)

// StateSyncProvider defines the interface for local-to-cloud sync operations.
type StateSyncProvider interface {
	SyncUp(ctx context.Context, claims *auth.Claims) error
	SyncDown(ctx context.Context, claims *auth.Claims) error
	GetStatus(ctx context.Context, claims *auth.Claims) (map[string]interface{}, error)
}

// StateSyncMCP implements the MCP interface for local-to-cloud state sync.
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
			Description: "Pushes unsynced local state to the cloud.",
			InputSchema: `{"type": "object", "properties": {}}`,
		},
		{
			Name:        "sync_cloud_to_local",
			Description: "Pulls updated state from the cloud to the local database.",
			InputSchema: `{"type": "object", "properties": {}}`,
		},
		{
			Name:        "get_sync_status",
			Description: "Retrieves the current sync status.",
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

	// No-op if running natively in the Cloud without a local SQLite counterpart
	if !m.dbProvider.IsSQLite() {
		return map[string]interface{}{
			"status": "success",
			"message": "running in cloud mode, sync not required",
		}, nil
	}

	switch toolName {
	case "sync_local_to_cloud":
		err := m.syncProvider.SyncUp(ctx, claims)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{
			"status": "success",
			"message": "sync up completed",
		}, nil
	case "sync_cloud_to_local":
		err := m.syncProvider.SyncDown(ctx, claims)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{
			"status": "success",
			"message": "sync down completed",
		}, nil
	case "get_sync_status":
		return m.syncProvider.GetStatus(ctx, claims)
	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}
