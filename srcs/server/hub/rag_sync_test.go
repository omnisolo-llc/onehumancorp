package hub

import (
    "context"
    "testing"
    "time"
)

type MockRAGSyncService struct {
    pendingRecords []RAGSyncRecord
    syncedIDs      []string
    processed      []RAGSyncRecord
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    if limit > len(m.pendingRecords) {
        return m.pendingRecords, nil
    }
    return m.pendingRecords[:limit], nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
    m.syncedIDs = append(m.syncedIDs, ids...)
    return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    m.processed = append(m.processed, records...)
    return nil
}

func TestRAGSyncService(t *testing.T) {
    mock := &MockRAGSyncService{
        pendingRecords: []RAGSyncRecord{
            {ID: "1", Context: "test1", SyncStatus: SyncStatusPending},
            {ID: "2", Context: "test2", SyncStatus: SyncStatusPending},
        },
    }

    ctx := context.Background()

    // Test FetchPendingSyncs
    records, err := mock.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if len(records) != 2 {
        t.Errorf("expected 2 records, got %d", len(records))
    }

    // Test MarkSynced
    err = mock.MarkSynced(ctx, []string{"1"})
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if len(mock.syncedIDs) != 1 || mock.syncedIDs[0] != "1" {
        t.Errorf("expected to mark id '1' as synced, got %v", mock.syncedIDs)
    }

    // Test ProcessIncomingSync
    incoming := []RAGSyncRecord{
        {ID: "3", Context: "test3", SyncStatus: SyncStatusSynced, LastSyncAt: time.Now()},
    }
    err = mock.ProcessIncomingSync(ctx, incoming)
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if len(mock.processed) != 1 || mock.processed[0].ID != "3" {
        t.Errorf("expected to process incoming record '3', got %v", mock.processed)
    }
}
