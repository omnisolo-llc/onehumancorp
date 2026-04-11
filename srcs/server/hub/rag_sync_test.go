package hub

import (
    "context"
    "testing"
)

type mockRAGSyncService struct {
    pending []RAGSyncRecord
    synced  []string
    cloud   []RAGSyncRecord
}

func (m *mockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    if limit > len(m.pending) {
        limit = len(m.pending)
    }
    return m.pending[:limit], nil
}

func (m *mockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
    m.synced = append(m.synced, ids...)
    return nil
}

func (m *mockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    m.cloud = append(m.cloud, records...)
    return nil
}

func TestRAGSyncServiceFlow(t *testing.T) {
    ctx := context.Background()
    mockService := &mockRAGSyncService{
        pending: []RAGSyncRecord{
            {ID: "m1", Context: "test context 1", Vector: []byte("vec1"), SyncStatus: SyncStatusPending},
        },
    }

    // Fetch pending
    pending, err := mockService.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("FetchPendingSyncs failed: %v", err)
    }
    if len(pending) != 1 {
        t.Fatalf("expected 1 pending record, got %d", len(pending))
    }

    // Push to cloud
    err = mockService.ProcessIncomingSync(ctx, pending)
    if err != nil {
        t.Fatalf("ProcessIncomingSync failed: %v", err)
    }
    if len(mockService.cloud) != 1 {
        t.Fatalf("expected 1 cloud record, got %d", len(mockService.cloud))
    }

    // Mark synced locally
    err = mockService.MarkSynced(ctx, []string{pending[0].ID})
    if err != nil {
        t.Fatalf("MarkSynced failed: %v", err)
    }
    if len(mockService.synced) != 1 {
        t.Fatalf("expected 1 synced record id, got %d", len(mockService.synced))
    }
}
