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
    if len(m.pending) > limit {
        return m.pending[:limit], nil
    }
    return m.pending, nil
}

func (m *mockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
    m.synced = append(m.synced, ids...)
    return nil
}

func (m *mockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    m.process = append(m.process, records...)
    return nil
}

func TestRAGSyncServiceMock(t *testing.T) {
    ctx := context.Background()
    svc := &mockRAGSyncService{
        pending: []RAGSyncRecord{
            {ID: "1", Context: "test1", SyncStatus: SyncStatusPending},
            {ID: "2", Context: "test2", SyncStatus: SyncStatusPending},
        },
    }

    records, err := svc.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("expected no error, got %v", err)
    }
    if len(records) != 2 {
        t.Errorf("expected 2 pending records, got %d", len(records))
    }

    err = svc.MarkSynced(ctx, []string{"1"})
    if err != nil {
        t.Fatalf("expected no error, got %v", err)
    }
    if len(svc.synced) != 1 || svc.synced[0] != "1" {
        t.Errorf("expected marked synced record 1, got %v", svc.synced)
    }

    err = svc.ProcessIncomingSync(ctx, []RAGSyncRecord{
        {ID: "3", Context: "test3", SyncStatus: SyncStatusSynced, LastSyncAt: time.Now()},
    })
    if err != nil {
        t.Fatalf("expected no error, got %v", err)
    }
    if len(svc.process) != 1 || svc.process[0].ID != "3" {
        t.Errorf("expected processed record 3, got %v", svc.process)
    }
}
