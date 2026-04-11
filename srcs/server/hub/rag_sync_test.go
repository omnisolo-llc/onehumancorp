package hub

import (
    "context"
    "testing"
    "time"
)

type mockRAGSyncService struct {
    records []RAGSyncRecord
}

func (m *mockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    var pending []RAGSyncRecord
    for _, r := range m.records {
        if r.SyncStatus == SyncStatusPending {
            pending = append(pending, r)
        }
    }
    if limit > 0 && len(pending) > limit {
        return pending[:limit], nil
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
        }
    }
    return nil
}

func (m *mockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    m.records = append(m.records, records...)
    return nil
}

func TestMockRAGSyncService(t *testing.T) {
    mock := &mockRAGSyncService{
        records: []RAGSyncRecord{
            {ID: "1", SyncStatus: SyncStatusPending},
            {ID: "2", SyncStatus: SyncStatusSynced},
            {ID: "3", SyncStatus: SyncStatusPending},
        },
    }

    ctx := context.Background()

    pending, err := mock.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if len(pending) != 2 {
        t.Fatalf("expected 2 pending records, got %d", len(pending))
    }

    err = mock.MarkSynced(ctx, []string{"1"})
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }

    pending, err = mock.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if len(pending) != 1 || pending[0].ID != "3" {
        t.Fatalf("expected 1 pending record with ID '3', got %d", len(pending))
    }

    newRec := RAGSyncRecord{ID: "4", SyncStatus: SyncStatusSynced}
    err = mock.ProcessIncomingSync(ctx, []RAGSyncRecord{newRec})
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
}
