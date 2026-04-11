package hub

import (
    "context"
    "testing"
    "time"
)

type MockRAGSyncService struct {}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    return []RAGSyncRecord{
        {
            ID: "1", Context: "test context", Vector: []byte{1, 2, 3}, SyncStatus: SyncStatusPending, LastSyncAt: time.Now(),
        },
    }, nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
    return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    return nil
}

func TestMockRAGSyncService(t *testing.T) {
    svc := &MockRAGSyncService{}
    ctx := context.Background()

    records, err := svc.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if len(records) != 1 {
        t.Fatalf("expected 1 record, got %d", len(records))
    }

    err = svc.MarkSynced(ctx, []string{"1"})
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }

    err = svc.ProcessIncomingSync(ctx, records)
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
}
