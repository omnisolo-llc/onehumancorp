package statesyncmcp

import (
	"context"
	"errors"
	"fmt"
)

type Claims struct {
	OrganizationID string
}

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
