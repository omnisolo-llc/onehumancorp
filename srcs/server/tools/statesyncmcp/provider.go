package statesyncmcp

import (
	"context"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// SyncStatus represents the current synchronization status.
type SyncStatus struct {
	LastSyncTime string `json:"last_sync_time"`
	PendingItems int    `json:"pending_items"`
	Status       string `json:"status"`
}

// StateSyncProvider defines the interface for synchronizing local state with the cloud.
type StateSyncProvider interface {
	// SyncUp pushes local state changes to the cloud.
	SyncUp(ctx context.Context, claims *auth.Claims) (map[string]interface{}, error)

	// SyncDown fetches state changes from the cloud to the local state.
	SyncDown(ctx context.Context, claims *auth.Claims) (map[string]interface{}, error)

	// GetStatus returns the current synchronization status.
	GetStatus(ctx context.Context, claims *auth.Claims) (*SyncStatus, error)
}
