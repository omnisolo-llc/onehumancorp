package hub

import (
    "context"
    "testing"
)

type MockRAGSyncService struct{}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    return []RAGSyncRecord{{ID: "1", Context: "test", Vector: []float32{0.1}, SyncStatus: SyncStatusPending}}, nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
    return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    return nil
}

func TestRAGSyncService(t *testing.T) {
    svc := &MockRAGSyncService{}
    ctx := context.Background()
    records, err := svc.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Errorf("FetchPendingSyncs failed: %v", err)
    }
    if len(records) != 1 || records[0].ID != "1" {
        t.Errorf("Unexpected records: %+v", records)
    }
    err = svc.MarkSynced(ctx, []string{"1"})
    if err != nil {
        t.Errorf("MarkSynced failed: %v", err)
    }
    err = svc.ProcessIncomingSync(ctx, records)
    if err != nil {
        t.Errorf("ProcessIncomingSync failed: %v", err)
    }
}
