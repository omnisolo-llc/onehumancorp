package hub

import (
    "context"
    "testing"
    "time"
)

// MockRAGSyncService is a mock implementation of RAGSyncService for testing.
type MockRAGSyncService struct {
    PendingRecords []RAGSyncRecord
    MarkedIDs      []string
    ProcessedRecords []RAGSyncRecord
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    if len(m.PendingRecords) > limit {
        return m.PendingRecords[:limit], nil
    }
    return m.PendingRecords, nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
    m.MarkedIDs = append(m.MarkedIDs, ids...)
    return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    m.ProcessedRecords = append(m.ProcessedRecords, records...)
    return nil
}

func TestMockRAGSyncService(t *testing.T) {
    mockSvc := &MockRAGSyncService{
        PendingRecords: []RAGSyncRecord{
            {ID: "1", Context: "test context 1", SyncStatus: SyncStatusPending, LastSyncAt: time.Now()},
            {ID: "2", Context: "test context 2", SyncStatus: SyncStatusPending, LastSyncAt: time.Now()},
        },
    }

    ctx := context.Background()

    // Test FetchPendingSyncs
    records, err := mockSvc.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if len(records) != 2 {
        t.Errorf("expected 2 pending records, got %d", len(records))
    }

    // Test MarkSynced
    idsToMark := []string{"1", "2"}
    err = mockSvc.MarkSynced(ctx, idsToMark)
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if len(mockSvc.MarkedIDs) != 2 {
        t.Errorf("expected 2 marked IDs, got %d", len(mockSvc.MarkedIDs))
    }

    // Test ProcessIncomingSync
    incomingRecords := []RAGSyncRecord{
        {ID: "3", Context: "test context 3", SyncStatus: SyncStatusSynced, LastSyncAt: time.Now()},
    }
    err = mockSvc.ProcessIncomingSync(ctx, incomingRecords)
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if len(mockSvc.ProcessedRecords) != 1 {
        t.Errorf("expected 1 processed record, got %d", len(mockSvc.ProcessedRecords))
    }
}
