package hub

import (
    "context"
    "testing"
)

type mockRAGSyncService struct{}

func (m *mockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    return []RAGSyncRecord{{ID: "1", SyncStatus: SyncStatusPending}}, nil
}

func (m *mockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
    return nil
}

func (m *mockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    return nil
}

func TestRAGSyncService(t *testing.T) {
    service := &mockRAGSyncService{}
    ctx := context.Background()

    records, err := service.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if len(records) != 1 {
        t.Fatalf("expected 1 record, got %d", len(records))
    }

    err = service.MarkSynced(ctx, []string{"1"})
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }

    err = service.ProcessIncomingSync(ctx, records)
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
}

func TestInitRAGSyncMetrics(t *testing.T) {
    err := InitRAGSyncMetrics(nil)
    if err != nil {
        t.Fatalf("unexpected error initializing metrics: %v", err)
    }
}
