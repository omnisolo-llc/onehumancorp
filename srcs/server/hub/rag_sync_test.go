package hub_test

import (
    "context"
    "testing"

    "github.com/onehumancorp/mono/srcs/server/hub"
)

type mockRAGSync struct{}

func (m *mockRAGSync) FetchPendingSyncs(ctx context.Context, limit int) ([]hub.RAGSyncRecord, error) {
    return []hub.RAGSyncRecord{}, nil
}

func (m *mockRAGSync) MarkSynced(ctx context.Context, ids []string) error {
    return nil
}

func (m *mockRAGSync) ProcessIncomingSync(ctx context.Context, records []hub.RAGSyncRecord) error {
    return nil
}

func TestMockRAGSyncService(t *testing.T) {
    var svc hub.RAGSyncService = &mockRAGSync{}
    ctx := context.Background()
    _, err := svc.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("FetchPendingSyncs error: %v", err)
    }
    err = svc.MarkSynced(ctx, []string{"id-1"})
    if err != nil {
        t.Fatalf("MarkSynced error: %v", err)
    }
    err = svc.ProcessIncomingSync(ctx, []hub.RAGSyncRecord{
        {ID: "id-1", SyncStatus: hub.SyncStatusSynced, LastSyncAt: nil},
    })
    if err != nil {
        t.Fatalf("ProcessIncomingSync error: %v", err)
    }
}
