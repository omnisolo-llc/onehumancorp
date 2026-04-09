package hub

import (
    "context"
    "testing"
    "time"
)

// MockRAGSyncService is a mock implementation of RAGSyncService for testing.
type MockRAGSyncService struct {
    records []RAGSyncRecord
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    var pending []RAGSyncRecord
    for _, r := range m.records {
        if r.SyncStatus == SyncStatusPending {
            pending = append(pending, r)
        }
        if len(pending) == limit {
            break
        }
    }
    return pending, nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
    for i, r := range m.records {
        for _, id := range ids {
            if r.ID == id {
                m.records[i].SyncStatus = SyncStatusSynced
                m.records[i].LastSyncAt = time.Now()
            }
        }
    }
    return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    m.records = append(m.records, records...)
    return nil
}

func TestMockRAGSyncService(t *testing.T) {
    svc := &MockRAGSyncService{
        records: []RAGSyncRecord{
            {ID: "1", Context: "Test Context 1", SyncStatus: SyncStatusPending},
            {ID: "2", Context: "Test Context 2", SyncStatus: SyncStatusSynced},
        },
    }

    ctx := context.Background()
    pending, err := svc.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if len(pending) != 1 {
        t.Errorf("expected 1 pending record, got %d", len(pending))
    }
    if pending[0].ID != "1" {
        t.Errorf("expected ID 1, got %s", pending[0].ID)
    }

    err = svc.MarkSynced(ctx, []string{"1"})
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }

    pending, _ = svc.FetchPendingSyncs(ctx, 10)
    if len(pending) != 0 {
        t.Errorf("expected 0 pending records after MarkSynced, got %d", len(pending))
    }
}
