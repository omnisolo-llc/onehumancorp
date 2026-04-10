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

func TestRAGSyncServiceMock(t *testing.T) {
    var service RAGSyncService = &mockRAGSyncService{}
    ctx := context.Background()
    records, err := service.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if len(records) != 1 || records[0].ID != "1" {
        t.Fatalf("unexpected records returned")
    }
}
