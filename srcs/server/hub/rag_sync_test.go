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
    if len(pending) > limit {
        pending = pending[:limit]
    }
    return pending, nil
}

func (m *mockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
    for i, r := range m.records {
        for _, id := range ids {
            if r.ID == id {
                m.records[i].SyncStatus = SyncStatusSynced
                m.records[i].LastSyncAt = time.Now()
            }
        }
    }
    return nil
}

func (m *mockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    m.records = append(m.records, records...)
    return nil
}

func TestMockRAGSyncService(t *testing.T) {
    ctx := context.Background()
    svc := &mockRAGSyncService{}

    records := []RAGSyncRecord{
        {ID: "1", Context: "test1", SyncStatus: SyncStatusPending},
        {ID: "2", Context: "test2", SyncStatus: SyncStatusPending},
    }

    err := svc.ProcessIncomingSync(ctx, records)
    if err != nil {
        t.Fatalf("ProcessIncomingSync failed: %v", err)
    }

    pending, err := svc.FetchPendingSyncs(ctx, 1)
    if err != nil {
        t.Fatalf("FetchPendingSyncs failed: %v", err)
    }
    if len(pending) != 1 {
        t.Fatalf("expected 1 pending record, got %d", len(pending))
    }

    err = svc.MarkSynced(ctx, []string{"1"})
    if err != nil {
        t.Fatalf("MarkSynced failed: %v", err)
    }

    pending, err = svc.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("FetchPendingSyncs failed: %v", err)
    }
    if len(pending) != 1 || pending[0].ID != "2" {
        t.Fatalf("expected pending record to be ID '2', got %v", pending)
    }
}
