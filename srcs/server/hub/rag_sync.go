package hub

import (
	"context"
	"time"
)

// SyncStatus represents the synchronization status of a memory.
type SyncStatus string

const (
	SyncStatusPending SyncStatus = "pending"
	SyncStatusSynced  SyncStatus = "synced"
	SyncStatusError   SyncStatus = "error"
)

// RAGSyncRecord represents a memory record intended for sync.
type RAGSyncRecord struct {
	ID         string
	Context    string
	Vector     []float32 // Convert to string internally for SQLite compat if needed
	SyncStatus SyncStatus
	LastSyncAt time.Time
}

// RAGSyncService defines the interface for synchronizing RAG memories between
// a local Standalone SQLite node and the multi-tenant Cloud PostgreSQL Orchestrator.
type RAGSyncService interface {
	// FetchPendingSyncs retrieves records from the local DB that need syncing.
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)

	// MarkSynced updates the local DB after a successful sync to the cloud.
	MarkSynced(ctx context.Context, ids []string) error

	// ProcessIncomingSync handles data pushed from a standalone client into the cloud DB.
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}
