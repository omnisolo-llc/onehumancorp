package statesyncmcp

import (
	"context"
)

type StateSyncProvider interface {
	SyncUp(ctx context.Context) (interface{}, error)
	SyncDown(ctx context.Context) (interface{}, error)
	GetStatus(ctx context.Context) (interface{}, error)
}

type SyncTool struct {
	Name        string `json:"name"`
	Description string `json:"description"`
	InputSchema string `json:"inputSchema"`
}

type Tool = SyncTool

type StateSyncMCP struct {
	provider StateSyncProvider
}

func NewStateSyncMCP(provider StateSyncProvider) *StateSyncMCP {
	return &StateSyncMCP{
		provider: provider,
	}
}

func (m *StateSyncMCP) ListTools() []Tool {
	return []Tool{
		{
			Name:        "sync_local_to_cloud",
			Description: "Synchronizes local SQLite state (e.g., KAIROS Shared Task List, agent missions) up to the Cloud PostgreSQL backend.",
			InputSchema: `{"type": "object", "properties": {}}`,
		},
		{
			Name:        "sync_cloud_to_local",
			Description: "Fetches completed tasks and state updates from the cloud and updates the local SQLite database.",
			InputSchema: `{"type": "object", "properties": {}}`,
		},
		{
			Name:        "get_sync_status",
			Description: "Returns the current synchronization status between local and cloud.",
			InputSchema: `{"type": "object", "properties": {}}`,
		},
	}
}

func (m *StateSyncMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	switch toolName {
	case "sync_local_to_cloud":
		return m.provider.SyncUp(ctx)
	case "sync_cloud_to_local":
		return m.provider.SyncDown(ctx)
	case "get_sync_status":
		return m.provider.GetStatus(ctx)
	default:
		return nil, nil
	}
}
