package statesyncmcp

import (
	"context"
	"errors"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
)

// Tool represents an MCP tool definition.
type Tool struct {
	Name        string `json:"name"`
	Description string `json:"description"`
	InputSchema string `json:"inputSchema"`
}

// StateSyncProvider defines the interface for local-to-cloud state synchronization.
type StateSyncProvider interface {
	SyncUp(ctx context.Context, claims *auth.Claims) (interface{}, error)
	SyncDown(ctx context.Context, claims *auth.Claims) (interface{}, error)
	GetStatus(ctx context.Context, claims *auth.Claims) (interface{}, error)
}

// DefaultStateSyncProvider implements StateSyncProvider
type DefaultStateSyncProvider struct {
	provider db.Provider
}

// NewDefaultStateSyncProvider creates a new DefaultStateSyncProvider
func NewDefaultStateSyncProvider(provider db.Provider) *DefaultStateSyncProvider {
	return &DefaultStateSyncProvider{provider: provider}
}

// SyncUp implements StateSyncProvider.SyncUp
func (p *DefaultStateSyncProvider) SyncUp(ctx context.Context, claims *auth.Claims) (interface{}, error) {
	if !p.provider.IsSQLite() {
		return map[string]interface{}{"status": "success", "message": "no-op in cloud mode", "synced_count": 0}, nil
	}
	// TODO: query local SQLite DB for unsynced state transitions, serialize, push to cloud API
	return map[string]interface{}{"status": "success", "synced_count": 0, "message": "mock sync up complete"}, nil
}

// SyncDown implements StateSyncProvider.SyncDown
func (p *DefaultStateSyncProvider) SyncDown(ctx context.Context, claims *auth.Claims) (interface{}, error) {
	if !p.provider.IsSQLite() {
		return map[string]interface{}{"status": "success", "message": "no-op in cloud mode", "synced_count": 0}, nil
	}
	// TODO: fetch completed tasks from cloud, update local SQLite
	return map[string]interface{}{"status": "success", "synced_count": 0, "message": "mock sync down complete"}, nil
}

// GetStatus implements StateSyncProvider.GetStatus
func (p *DefaultStateSyncProvider) GetStatus(ctx context.Context, claims *auth.Claims) (interface{}, error) {
	mode := "cloud"
	if p.provider.IsSQLite() {
		mode = "standalone"
	}
	return map[string]interface{}{
		"status": "success",
		"mode": mode,
		"last_sync": time.Now().Format(time.RFC3339),
	}, nil
}

// StateSyncMCP implements the MCP interface for local-to-cloud state sync.
type StateSyncMCP struct {
	provider StateSyncProvider
}

// NewStateSyncMCP creates a new StateSyncMCP instance.
func NewStateSyncMCP(provider StateSyncProvider) *StateSyncMCP {
	return &StateSyncMCP{
		provider: provider,
	}
}

// ListTools returns the list of available tools.
func (m *StateSyncMCP) ListTools() []Tool {
	return []Tool{
		{
			Name:        "sync_local_to_cloud",
			Description: "Synchronizes local state to the cloud.",
			InputSchema: `{"type": "object", "properties": {}}`,
		},
		{
			Name:        "sync_cloud_to_local",
			Description: "Synchronizes cloud state to the local database.",
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
