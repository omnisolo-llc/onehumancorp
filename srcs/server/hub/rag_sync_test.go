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

func TestMockRAGSyncService(t *testing.T) {
    mock := &mockRAGSyncService{
        pending: []RAGSyncRecord{
            {ID: "1", Context: "test context", SyncStatus: SyncStatusPending},
        },
    }

    ctx := context.Background()

    records, err := mock.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("expected no error, got %v", err)
    }
    if len(records) != 1 {
        t.Fatalf("expected 1 record, got %d", len(records))
    }

    err = mock.MarkSynced(ctx, []string{"1"})
    if err != nil {
        t.Fatalf("expected no error, got %v", err)
    }
    if len(mock.synced) != 1 || mock.synced[0] != "1" {
        t.Fatalf("expected id '1' to be marked as synced")
    }

    err = mock.ProcessIncomingSync(ctx, []RAGSyncRecord{
        {ID: "2", Context: "incoming test context", SyncStatus: SyncStatusSynced, LastSyncAt: time.Now()},
    })
    if err != nil {
        t.Fatalf("expected no error, got %v", err)
    }
    if len(mock.process) != 1 || mock.process[0].ID != "2" {
        t.Fatalf("expected incoming record '2' to be processed")
    }
}
