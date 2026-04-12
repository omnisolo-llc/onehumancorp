package hub

import (
	"context"
	"fmt"
	"time"
)

type SyncStatus string

const (
	SyncStatusPending SyncStatus = "pending"
	SyncStatusSynced  SyncStatus = "synced"
	SyncStatusError   SyncStatus = "error"
)

type RAGSyncRecord struct {
	ID         string
	Context    string
	Vector     []float32 // Convert to string internally for SQLite compat if needed
	SyncStatus SyncStatus
	LastSyncAt time.Time
}

type RAGSyncService interface {
	// FetchPendingSyncs retrieves records from the local DB that need syncing
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)

	// MarkSynced updates the local DB after a successful sync to the cloud
	MarkSynced(ctx context.Context, ids []string) error

	// ProcessIncomingSync handles data pushed from a standalone client into the cloud DB
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type DefaultRAGSyncService struct {
	// dependencies will be injected here
}

func NewDefaultRAGSyncService() *DefaultRAGSyncService {
	return &DefaultRAGSyncService{}
}

func (s *DefaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	// Concrete implementation for fetching from SQLite
	return nil, fmt.Errorf("not implemented")
}

func (s *DefaultRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	// Concrete implementation for updating SQLite status
	return fmt.Errorf("not implemented")
}

func (s *DefaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	// Concrete implementation for upserting to Postgres
	return fmt.Errorf("not implemented")
}
