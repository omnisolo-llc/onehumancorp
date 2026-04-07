package statesyncmcp

import (
	"context"
)

// StateSyncProvider defines the interface for local-to-cloud state synchronization
type StateSyncProvider interface {
	SyncUp(ctx context.Context, tenantID string) (interface{}, error)
	SyncDown(ctx context.Context, tenantID string) (interface{}, error)
	GetStatus(ctx context.Context, tenantID string) (interface{}, error)
	IsLocal() bool
}
