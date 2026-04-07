package statesyncmcp

import (
	"context"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// SyncStatus represents the status of a sync operation.
type SyncStatus string

const (
	SyncStatusPending SyncStatus = "pending"
	SyncStatusSynced  SyncStatus = "synced"
	SyncStatusError   SyncStatus = "error"
)

// SyncResult holds the result of a sync operation.
type SyncResult struct {
	SyncedCount int
	Errors      []string
}

// StateSyncProvider abstracts the local-to-cloud state synchronization logic.
type StateSyncProvider interface {
	// SyncUp pushes unsynced local state to the cloud.
	SyncUp(ctx context.Context, claims *auth.Claims) (SyncResult, error)

	// SyncDown fetches completed tasks/state from the cloud and updates the local database.
	SyncDown(ctx context.Context, claims *auth.Claims) (SyncResult, error)

	// GetStatus returns the current sync status.
	GetStatus(ctx context.Context, claims *auth.Claims) (interface{}, error)
}
