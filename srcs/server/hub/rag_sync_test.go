package hub

import (
    "context"
    "testing"
)

type MockRAGSyncService struct {
    PendingSyncs []RAGSyncRecord
    SyncedIDs    []string
    Processed    []RAGSyncRecord
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    if limit > len(m.PendingSyncs) {
        limit = len(m.PendingSyncs)
    }
    return m.PendingSyncs[:limit], nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
    m.SyncedIDs = append(m.SyncedIDs, ids...)
    if ragRecordsSyncedTotal != nil {
        ragRecordsSyncedTotal.Add(ctx, int64(len(ids)))
    }
    return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    m.Processed = append(m.Processed, records...)
    return nil
}

func TestRAGSyncService(t *testing.T) {
    mockService := &MockRAGSyncService{
        PendingSyncs: []RAGSyncRecord{
            {ID: "1", Context: "test1", SyncStatus: SyncStatusPending},
            {ID: "2", Context: "test2", SyncStatus: SyncStatusPending},
        },
    }

    ctx := context.Background()

    // Test FetchPendingSyncs
    records, err := mockService.FetchPendingSyncs(ctx, 2)
    if err != nil {
        t.Fatalf("FetchPendingSyncs failed: %v", err)
    }
    if len(records) != 2 {
        t.Errorf("Expected 2 records, got %d", len(records))
    }

    // Test MarkSynced
    err = mockService.MarkSynced(ctx, []string{"1", "2"})
    if err != nil {
        t.Fatalf("MarkSynced failed: %v", err)
    }
    if len(mockService.SyncedIDs) != 2 {
        t.Errorf("Expected 2 synced IDs, got %d", len(mockService.SyncedIDs))
    }

    // Test ProcessIncomingSync
    err = mockService.ProcessIncomingSync(ctx, records)
    if err != nil {
        t.Fatalf("ProcessIncomingSync failed: %v", err)
    }
    if len(mockService.Processed) != 2 {
        t.Errorf("Expected 2 processed records, got %d", len(mockService.Processed))
    }
}
