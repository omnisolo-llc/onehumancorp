package statesyncmcp

import (
	"context"
	"errors"
	"fmt"
)

type Claims struct {
	OrganizationID string
}

type contextKey string

const ContextKeyClaims = contextKey("claims")

type SyncStatus struct {
	LastSyncTime string `json:"last_sync_time"`
	PendingTasks int    `json:"pending_tasks"`
	Status       string `json:"status"`
}

type StateSyncProvider interface {
	SyncUp(ctx context.Context, orgID string) (int, error)
	SyncDown(ctx context.Context, orgID string) (int, error)
	GetStatus(ctx context.Context, orgID string) (*SyncStatus, error)
}

type Hub interface {
	StateSync() StateSyncProvider
}

type StateSyncMCP struct {
	hub Hub
}

func NewStateSyncMCP(hub Hub) *StateSyncMCP {
	return &StateSyncMCP{
		hub: hub,
	}
}

func (s *StateSyncMCP) CallTool(ctx context.Context, name string, args map[string]interface{}) (interface{}, error) {
	claims, ok := ctx.Value(ContextKeyClaims).(*Claims)
	if !ok || claims == nil || claims.OrganizationID == "" {
		return nil, errors.New("unauthorized: missing claims or organization ID")
	}

	provider := s.hub.StateSync()
	if provider == nil {
		return nil, errors.New("state sync provider not configured")
	}

	switch name {
	case "sync_local_to_cloud":
		count, err := provider.SyncUp(ctx, claims.OrganizationID)
		if err != nil {
			return nil, fmt.Errorf("sync_local_to_cloud failed: %w", err)
		}
		return map[string]interface{}{
			"status":       "success",
			"synced_count": count,
		}, nil

	case "sync_cloud_to_local":
		count, err := provider.SyncDown(ctx, claims.OrganizationID)
		if err != nil {
			return nil, fmt.Errorf("sync_cloud_to_local failed: %w", err)
		}
		return map[string]interface{}{
			"status":       "success",
			"synced_count": count,
		}, nil

	case "get_sync_status":
		status, err := provider.GetStatus(ctx, claims.OrganizationID)
		if err != nil {
			return nil, fmt.Errorf("get_sync_status failed: %w", err)
		}
		return status, nil

	default:
		return nil, fmt.Errorf("unknown tool: %s", name)
	}
}

func (s *StateSyncMCP) ListTools() []map[string]interface{} {
	return []map[string]interface{}{
		{
			"name":        "sync_local_to_cloud",
			"description": "Synchronize local offline state to the cloud.",
			"parameters": map[string]interface{}{
				"type":       "object",
				"properties": map[string]interface{}{},
			},
		},
		{
			"name":        "sync_cloud_to_local",
			"description": "Fetch remote cloud state to the local environment.",
			"parameters": map[string]interface{}{
				"type":       "object",
				"properties": map[string]interface{}{},
			},
		},
		{
			"name":        "get_sync_status",
			"description": "Get the current synchronization status.",
			"parameters": map[string]interface{}{
				"type":       "object",
				"properties": map[string]interface{}{},
			},
		},
	}
}

