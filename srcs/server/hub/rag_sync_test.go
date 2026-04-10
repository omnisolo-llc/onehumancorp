package hub

import (
    "context"
    "testing"
    "time"
)

type mockRAGSyncService struct {
    records []RAGSyncRecord
    synced  []string
}

func (m *mockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    return m.records, nil
}

func (m *mockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
    m.synced = append(m.synced, ids...)
    return nil
}

func (m *mockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    m.records = append(m.records, records...)
    return nil
}

func TestMockRAGSyncService(t *testing.T) {
    mock := &mockRAGSyncService{}
    ctx := context.Background()

    record := RAGSyncRecord{
        ID:         "1",
        Context:    "test context",
        Vector:     []float32{0.1, 0.2},
        SyncStatus: SyncStatusPending,
        LastSyncAt: time.Now(),
    }

    err := mock.ProcessIncomingSync(ctx, []RAGSyncRecord{record})
    if err != nil {
        t.Fatalf("ProcessIncomingSync failed: %v", err)
    }

    pending, err := mock.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("FetchPendingSyncs failed: %v", err)
    }
    if len(pending) != 1 || pending[0].ID != "1" {
        t.Errorf("Expected 1 pending record with ID '1', got %+v", pending)
    }

    err = mock.MarkSynced(ctx, []string{"1"})
    if err != nil {
        t.Fatalf("MarkSynced failed: %v", err)
    }
    if len(mock.synced) != 1 || mock.synced[0] != "1" {
        t.Errorf("Expected 1 synced record with ID '1', got %+v", mock.synced)
    }
}
