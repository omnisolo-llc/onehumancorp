package statesyncmcp

import (
	"context"
	"errors"
	"fmt"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// StateSyncMCP implements the MCP interface for local-to-cloud state synchronization.
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
			Description: "Pushes unsynchronized local state (Standalone Mode) to the Cloud backend.",
			InputSchema: `{"type": "object", "properties": {}}`,
		},
		{
			Name:        "sync_cloud_to_local",
			Description: "Fetches completed task delegations and updates from the Cloud backend into the local state.",
			InputSchema: `{"type": "object", "properties": {}}`,
		},
		{
			Name:        "get_sync_status",
			Description: "Retrieves the current synchronization status, including unsynced item counts and last sync timestamp.",
			InputSchema: `{"type": "object", "properties": {}}`,
		},
	}
}

// CallTool executes a tool by name.
func (m *StateSyncMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	claims := auth.ClaimsFromContext(ctx)
	// We don't strictly require claims if running in local standalone mode,
	// but the provider might need it to construct requests if it exists.
	// If the provider dictates it needs it, we let it fail there.

	switch toolName {
	case "sync_local_to_cloud":
		return m.syncLocalToCloud(ctx, claims)
	case "sync_cloud_to_local":
		return m.syncCloudToLocal(ctx, claims)
	case "get_sync_status":
		return m.getSyncStatus(ctx, claims)
	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}

func (m *StateSyncMCP) syncLocalToCloud(ctx context.Context, claims *auth.Claims) (interface{}, error) {
	if m.provider == nil {
		return nil, errors.New("sync provider not configured")
	}

	result, err := m.provider.SyncUp(ctx, claims)
	if err != nil {
		return nil, fmt.Errorf("sync_local_to_cloud failed: %w", err)
	}

	return result, nil
}

func (m *StateSyncMCP) syncCloudToLocal(ctx context.Context, claims *auth.Claims) (interface{}, error) {
	if m.provider == nil {
		return nil, errors.New("sync provider not configured")
	}

	result, err := m.provider.SyncDown(ctx, claims)
	if err != nil {
		return nil, fmt.Errorf("sync_cloud_to_local failed: %w", err)
	}

	return result, nil
}

func (m *StateSyncMCP) getSyncStatus(ctx context.Context, claims *auth.Claims) (interface{}, error) {
	if m.provider == nil {
		return nil, errors.New("sync provider not configured")
	}

	status, err := m.provider.GetStatus(ctx, claims)
	if err != nil {
		return nil, fmt.Errorf("get_sync_status failed: %w", err)
	}

	return map[string]interface{}{
		"status":         "success",
		"last_sync_time": status.LastSyncTime,
		"pending_items":  status.PendingItems,
		"sync_state":     status.Status,
	}, nil
}
