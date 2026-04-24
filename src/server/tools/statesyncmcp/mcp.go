package statesyncmcp

import (
	"context"
	"errors"
	"fmt"

	"github.com/onehumancorp/mono/src/server/auth"
)

// StateSyncProvider abstracts the local-to-cloud synchronization logic.
type StateSyncProvider interface {
	SyncUp(ctx context.Context, claims *auth.Claims) (map[string]interface{}, error)
	SyncDown(ctx context.Context, claims *auth.Claims) (map[string]interface{}, error)
	GetStatus(ctx context.Context, claims *auth.Claims) (map[string]interface{}, error)
	CRDTPush(ctx context.Context, payload map[string]interface{}, claims *auth.Claims) (map[string]interface{}, error)
	CRDTPull(ctx context.Context, entityID string, claims *auth.Claims) (map[string]interface{}, error)
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
		{
			Name:        "crdt_push",
			Description: "Push CRDT state updates locally to eventually synchronize with the Cloud.",
			InputSchema: `{"type": "object", "properties": {"id": {"type": "string"}, "entity_id": {"type": "string"}, "data": {"type": "string"}, "updated_at": {"type": "string"}}, "required": ["id", "entity_id", "data", "updated_at"]}`,
		},
		{
			Name:        "crdt_pull",
			Description: "Pull the latest CRDT state vector for a given entity.",
			InputSchema: `{"type": "object", "properties": {"entity_id": {"type": "string"}}, "required": ["entity_id"]}`,
		},
	}
}

// CallTool executes a tool by name.
func (m *StateSyncMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	if !m.isLocal {
		return map[string]interface{}{
			"status": "skipped",
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
	case "crdt_push":
		return m.provider.CRDTPush(ctx, arguments, claims)
	case "crdt_pull":
		entityID, ok := arguments["entity_id"].(string)
		if !ok || entityID == "" {
			return nil, errors.New("missing or invalid entity_id")
		}
		return m.provider.CRDTPull(ctx, entityID, claims)
	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}
