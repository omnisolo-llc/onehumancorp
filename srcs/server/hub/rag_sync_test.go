package hub

import (
    "context"
    "testing"
    "time"
)

type MockRAGSyncService struct {
    Records  []RAGSyncRecord
    Synced   []string
    Incoming []RAGSyncRecord
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    var pending []RAGSyncRecord
    for _, r := range m.Records {
        if r.SyncStatus == SyncStatusPending {
            pending = append(pending, r)
            if len(pending) == limit {
                break
            }
        }
    }
    return pending, nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
    m.Synced = append(m.Synced, ids...)
    for i, r := range m.Records {
        for _, id := range ids {
            if r.ID == id {
                m.Records[i].SyncStatus = SyncStatusSynced
                m.Records[i].LastSyncAt = time.Now()
            }
        }
    }
    return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    m.Incoming = append(m.Incoming, records...)
    return nil
}

func TestRAGSyncServiceMock(t *testing.T) {
    mock := &MockRAGSyncService{
        Records: []RAGSyncRecord{
            {ID: "1", Context: "test1", SyncStatus: SyncStatusPending},
            {ID: "2", Context: "test2", SyncStatus: SyncStatusPending},
        },
    }

    ctx := context.Background()
    pending, err := mock.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if len(pending) != 2 {
        t.Errorf("expected 2 pending records, got %d", len(pending))
    }

    err = mock.MarkSynced(ctx, []string{"1"})
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }

    pending, _ = mock.FetchPendingSyncs(ctx, 10)
    if len(pending) != 1 {
        t.Errorf("expected 1 pending record after marking synced, got %d", len(pending))
    }

    err = mock.ProcessIncomingSync(ctx, []RAGSyncRecord{{ID: "3", Context: "test3"}})
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if len(mock.Incoming) != 1 {
        t.Errorf("expected 1 incoming record, got %d", len(mock.Incoming))
    }

    // Test metrics initialization with nil (skips)
    InitRAGSyncMetrics(nil)
}
