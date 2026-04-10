package hub

import (
    "context"
    "testing"
)

type MockRAGSyncService struct {
    pendingRecords []RAGSyncRecord
    syncedIDs      []string
    processed      []RAGSyncRecord
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    return m.pendingRecords, nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
    m.syncedIDs = append(m.syncedIDs, ids...)
    return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    m.processed = append(m.processed, records...)
    return nil
}

func TestMockRAGSyncService(t *testing.T) {
    ctx := context.Background()
    mockService := &MockRAGSyncService{
        pendingRecords: []RAGSyncRecord{
            {ID: "test-id-1", Context: "test context", SyncStatus: SyncStatusPending},
        },
    }

    records, err := mockService.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if len(records) != 1 {
        t.Errorf("expected 1 record, got %d", len(records))
    }

    err = mockService.MarkSynced(ctx, []string{"test-id-1"})
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if len(mockService.syncedIDs) != 1 || mockService.syncedIDs[0] != "test-id-1" {
        t.Errorf("expected synced ID test-id-1")
    }

    err = mockService.ProcessIncomingSync(ctx, records)
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if len(mockService.processed) != 1 {
        t.Errorf("expected 1 processed record")
    }
}
