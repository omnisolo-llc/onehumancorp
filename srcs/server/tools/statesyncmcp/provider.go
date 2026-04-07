package statesyncmcp

import (
	"context"
)

type SyncStatus string

const (
	SyncStatusPending SyncStatus = "pending"
	SyncStatusSynced  SyncStatus = "synced"
	SyncStatusError   SyncStatus = "error"
)

type SyncStatusResponse struct {
	PendingUp   int `json:"pending_up"`
	PendingDown int `json:"pending_down"`
	LastSync    string `json:"last_sync"`
}

type SyncResult struct {
	SyncedRecords int `json:"synced_records"`
	Errors        int `json:"errors"`
}

type StateSyncProvider interface {
	SyncUp(ctx context.Context) (SyncResult, error)
	SyncDown(ctx context.Context) (SyncResult, error)
	GetStatus(ctx context.Context) (SyncStatusResponse, error)
}
