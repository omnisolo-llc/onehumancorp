package hub

import (
    "context"
    "time"
)

type SyncStatus string

const (
    SyncStatusPending SyncStatus = "pending"
    SyncStatusSynced  SyncStatus = "synced"
    SyncStatusError   SyncStatus = "error"
)

type RAGSyncRecord struct {
    ID           string
    Context      string
    Vector       []byte
    SyncStatus   SyncStatus
    LastSyncAt   time.Time
}

type RAGSyncService interface {
    FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
    MarkSynced(ctx context.Context, ids []string) error
    ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type MockRAGSyncService struct{}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    return []RAGSyncRecord{}, nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
    return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    return nil
}
