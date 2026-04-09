package hub

import (
    "context"
    "testing"
    "time"
)

type MockRAGSyncService struct {
    FetchPendingSyncsFunc   func(ctx context.Context, limit int) ([]RAGSyncRecord, error)
    MarkSyncedFunc          func(ctx context.Context, ids []string) error
    ProcessIncomingSyncFunc func(ctx context.Context, records []RAGSyncRecord) error
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    return m.FetchPendingSyncsFunc(ctx, limit)
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
    return m.MarkSyncedFunc(ctx, ids)
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    return m.ProcessIncomingSyncFunc(ctx, records)
}

func TestMockRAGSyncService(t *testing.T) {
    mock := &MockRAGSyncService{
        FetchPendingSyncsFunc: func(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
            return []RAGSyncRecord{{ID: "1", SyncStatus: SyncStatusPending, LastSyncAt: time.Now()}}, nil
        },
    }
    records, err := mock.FetchPendingSyncs(context.Background(), 10)
    if err != nil {
        t.Fatalf("expected no error, got %v", err)
    }
    if len(records) != 1 || records[0].ID != "1" {
        t.Fatalf("unexpected records: %+v", records)
    }
}
