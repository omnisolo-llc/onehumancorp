package statesyncmcp

import (
	"context"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// LocalSyncProvider implements StateSyncProvider for standalone local modes.
type LocalSyncProvider struct {
	// Add dependencies like db.Provider here
}

func NewLocalSyncProvider() *LocalSyncProvider {
	return &LocalSyncProvider{}
}

func (p *LocalSyncProvider) SyncUp(ctx context.Context) (SyncResult, error) {
	// Ensure tenant context
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return SyncResult{}, fmt.Errorf("unauthorized: missing claims")
	}

	// TODO: integrate with db.SyncProvider to extract local state
	return SyncResult{
		SyncedRecords: 0,
		Errors:        0,
	}, nil
}

func (p *LocalSyncProvider) SyncDown(ctx context.Context) (SyncResult, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return SyncResult{}, fmt.Errorf("unauthorized: missing claims")
	}

	// TODO: fetch from cloud and update local db
	return SyncResult{
		SyncedRecords: 0,
		Errors:        0,
	}, nil
}

func (p *LocalSyncProvider) GetStatus(ctx context.Context) (SyncStatusResponse, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return SyncStatusResponse{}, fmt.Errorf("unauthorized: missing claims")
	}

	// TODO: determine actual counts
	return SyncStatusResponse{
		PendingUp:   0,
		PendingDown: 0,
		LastSync:    time.Now().Format(time.RFC3339),
	}, nil
}

// CloudSyncProvider implements StateSyncProvider for cloud modes where local state doesn't exist.
// This is essentially a no-op or fallback.
type CloudSyncProvider struct {
}

func NewCloudSyncProvider() *CloudSyncProvider {
	return &CloudSyncProvider{}
}

func (p *CloudSyncProvider) SyncUp(ctx context.Context) (SyncResult, error) {
	return SyncResult{}, fmt.Errorf("cloud mode: sync up not supported")
}

func (p *CloudSyncProvider) SyncDown(ctx context.Context) (SyncResult, error) {
	return SyncResult{}, fmt.Errorf("cloud mode: sync down not supported")
}

func (p *CloudSyncProvider) GetStatus(ctx context.Context) (SyncStatusResponse, error) {
	return SyncStatusResponse{
		PendingUp:   0,
		PendingDown: 0,
		LastSync:    "Never",
	}, nil
}
