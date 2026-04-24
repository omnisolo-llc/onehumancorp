package hub

import (
    "context"
    "testing"
    "time"
)

type mockRAGSyncService struct {
    pending []RAGSyncRecord
    synced  []string
    process []RAGSyncRecord
}

func (m *mockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    if limit > len(m.pending) {
        return m.pending, nil
    }
    return m.pending[:limit], nil
}

func (m *mockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
    m.synced = append(m.synced, ids...)
    return nil
}

func (m *mockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    m.process = append(m.process, records...)
    return nil
}

func TestRAGSyncService(t *testing.T) {
    mock := &mockRAGSyncService{
        pending: []RAGSyncRecord{
            {ID: "1", Context: "test", SyncStatus: SyncStatusPending},
        },
    }

    ctx := context.Background()

    // Test Fetch
    records, err := mock.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if len(records) != 1 {
        t.Fatalf("expected 1 record, got %d", len(records))
    }

    // Test MarkSynced
    err = mock.MarkSynced(ctx, []string{"1"})
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if len(mock.synced) != 1 || mock.synced[0] != "1" {
        t.Fatalf("expected 1 synced record with ID 1, got %v", mock.synced)
    }

    // Test ProcessIncomingSync
    err = mock.ProcessIncomingSync(ctx, []RAGSyncRecord{
        {ID: "2", Context: "test2", SyncStatus: SyncStatusSynced, LastSyncAt: time.Now()},
    })
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if len(mock.process) != 1 || mock.process[0].ID != "2" {
        t.Fatalf("expected 1 processed record with ID 2, got %v", mock.process)
    }
}
