package hub

import (
    "context"
    "testing"
    "time"
)

type mockRAGSyncService struct {
    records []RAGSyncRecord
    synced  []string
}

func (m *mockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    var pending []RAGSyncRecord
    for _, r := range m.records {
        if r.SyncStatus == SyncStatusPending {
            pending = append(pending, r)
            if len(pending) == limit {
                break
            }
        }
    }
    return pending, nil
}

func (m *mockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
    idMap := make(map[string]bool)
    for _, id := range ids {
        idMap[id] = true
    }
    for i, r := range m.records {
        if idMap[r.ID] {
            m.records[i].SyncStatus = SyncStatusSynced
            m.records[i].LastSyncAt = time.Now()
            m.synced = append(m.synced, r.ID)
        }
    }
    return nil
}

func (m *mockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    m.records = append(m.records, records...)
    return nil
}

func TestMockRAGSyncService(t *testing.T) {
    mock := &mockRAGSyncService{}
    ctx := context.Background()

    records := []RAGSyncRecord{
        {ID: "1", Context: "test 1", SyncStatus: SyncStatusPending},
        {ID: "2", Context: "test 2", SyncStatus: SyncStatusPending},
    }

    if err := mock.ProcessIncomingSync(ctx, records); err != nil {
        t.Fatalf("unexpected error: %v", err)
    }

    pending, err := mock.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if len(pending) != 2 {
        t.Errorf("expected 2 pending records, got %d", len(pending))
    }

    if err := mock.MarkSynced(ctx, []string{"1"}); err != nil {
        t.Fatalf("unexpected error: %v", err)
    }

    pending, err = mock.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if len(pending) != 1 || pending[0].ID != "2" {
        t.Errorf("expected 1 pending record with ID 2, got %v", pending)
    }
}
