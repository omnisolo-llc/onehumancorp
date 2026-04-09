package hub

import (
    "context"
    "testing"
)

// mockRAGSyncService is a mock implementation of RAGSyncService for testing purposes.
type mockRAGSyncService struct {
    records []RAGSyncRecord
    synced  []string
    pushed  []RAGSyncRecord
}

func (m *mockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    var pending []RAGSyncRecord
    for i, r := range m.records {
        if i >= limit {
            break
        }
        if r.SyncStatus == SyncStatusPending {
            pending = append(pending, r)
        }
    }
    return pending, nil
}

func (m *mockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
    m.synced = append(m.synced, ids...)
    return nil
}

func (m *mockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    m.pushed = append(m.pushed, records...)
    return nil
}

func TestRAGSyncService_Flow(t *testing.T) {
    mock := &mockRAGSyncService{
        records: []RAGSyncRecord{
            {ID: "1", Context: "test1", SyncStatus: SyncStatusPending},
            {ID: "2", Context: "test2", SyncStatus: SyncStatusPending},
        },
    }

    ctx := context.Background()

    pending, err := mock.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if len(pending) != 2 {
        t.Errorf("expected 2 pending records, got %d", len(pending))
    }

    err = mock.MarkSynced(ctx, []string{"1", "2"})
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if len(mock.synced) != 2 {
        t.Errorf("expected 2 synced records, got %d", len(mock.synced))
    }

    err = mock.ProcessIncomingSync(ctx, []RAGSyncRecord{
        {ID: "3", Context: "test3"},
    })
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if len(mock.pushed) != 1 {
        t.Errorf("expected 1 pushed record, got %d", len(mock.pushed))
    }
}
