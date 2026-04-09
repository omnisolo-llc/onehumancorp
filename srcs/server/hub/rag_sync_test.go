package hub

import (
    "context"
    "testing"
    "time"
)

type mockRAGSyncService struct{}

func (m *mockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    return []RAGSyncRecord{
        {
            ID:         "1",
            Context:    "test",
            SyncStatus: SyncStatusPending,
            LastSyncAt: time.Now(),
        },
    }, nil
}

func (m *mockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
    return nil
}

func (m *mockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    return nil
}

func TestRAGSyncService(t *testing.T) {
    var service RAGSyncService = &mockRAGSyncService{}
    records, err := service.FetchPendingSyncs(context.Background(), 10)
    if err != nil {
        t.Fatalf("FetchPendingSyncs failed: %v", err)
    }
    if len(records) != 1 {
        t.Fatalf("expected 1 record, got %d", len(records))
    }
    err = service.MarkSynced(context.Background(), []string{"1"})
    if err != nil {
        t.Fatalf("MarkSynced failed: %v", err)
    }
    err = service.ProcessIncomingSync(context.Background(), records)
    if err != nil {
        t.Fatalf("ProcessIncomingSync failed: %v", err)
    }
}
