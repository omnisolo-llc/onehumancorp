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
            ID:         "test-id",
            Context:    "test-context",
            Vector:     []float32{1.0, 2.0},
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

func TestMockRAGSyncService(t *testing.T) {
    svc := &mockRAGSyncService{}
    ctx := context.Background()

    records, err := svc.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if len(records) != 1 {
        t.Fatalf("expected 1 record, got %d", len(records))
    }

    err = svc.MarkSynced(ctx, []string{records[0].ID})
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }

    err = svc.ProcessIncomingSync(ctx, records)
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
}
