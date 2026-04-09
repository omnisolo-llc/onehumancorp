package hub

import (
    "context"
    "testing"
)

type MockRAGSyncService struct {
    pending []RAGSyncRecord
    synced  []string
    errors  error
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    if m.errors != nil {
        return nil, m.errors
    }
    if len(m.pending) > limit {
        return m.pending[:limit], nil
    }
    return m.pending, nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
    if m.errors != nil {
        return m.errors
    }
    m.synced = append(m.synced, ids...)
    return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    if m.errors != nil {
        return m.errors
    }
    m.pending = append(m.pending, records...)
    return nil
}

func TestMockRAGSyncService(t *testing.T) {
    mock := &MockRAGSyncService{
        pending: []RAGSyncRecord{
            {ID: "1", Context: "test1", SyncStatus: SyncStatusPending},
            {ID: "2", Context: "test2", SyncStatus: SyncStatusPending},
        },
    }

    ctx := context.Background()

    // Test FetchPendingSyncs
    records, err := mock.FetchPendingSyncs(ctx, 1)
    if err != nil {
        t.Fatalf("expected no error, got %v", err)
    }
    if len(records) != 1 {
        t.Fatalf("expected 1 record, got %d", len(records))
    }

    // Test MarkSynced
    err = mock.MarkSynced(ctx, []string{"1"})
    if err != nil {
        t.Fatalf("expected no error, got %v", err)
    }
    if len(mock.synced) != 1 || mock.synced[0] != "1" {
        t.Fatalf("expected synced to contain '1', got %v", mock.synced)
    }

    // Test ProcessIncomingSync
    err = mock.ProcessIncomingSync(ctx, []RAGSyncRecord{{ID: "3", Context: "test3", SyncStatus: SyncStatusPending}})
    if err != nil {
        t.Fatalf("expected no error, got %v", err)
    }
    if len(mock.pending) != 3 {
        t.Fatalf("expected pending to contain 3 records, got %d", len(mock.pending))
    }
}
