package hub

import (
    "context"
    "testing"
    "time"
)

type MockRAGSyncService struct {
    pending []RAGSyncRecord
    synced  []string
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    if limit > len(m.pending) {
        return m.pending, nil
    }
    return m.pending[:limit], nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
    m.synced = append(m.synced, ids...)
    return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    return nil
}

func TestRAGSyncService(t *testing.T) {
    mock := &MockRAGSyncService{
        pending: []RAGSyncRecord{
            {ID: "1", Context: "test", SyncStatus: SyncStatusPending, LastSyncAt: time.Now()},
        },
    }

    ctx := context.Background()
    records, err := mock.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if len(records) != 1 {
        t.Errorf("expected 1 record, got %d", len(records))
    }

    err = mock.MarkSynced(ctx, []string{"1"})
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if len(mock.synced) != 1 || mock.synced[0] != "1" {
        t.Errorf("expected 1 synced ID '1', got %v", mock.synced)
    }
}
