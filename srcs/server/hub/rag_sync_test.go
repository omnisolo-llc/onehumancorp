package hub

import (
    "context"
    "testing"
    "time"
)

type MockRAGSyncService struct {
    Records map[string]RAGSyncRecord
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    var res []RAGSyncRecord
    for _, r := range m.Records {
        if r.SyncStatus == SyncStatusPending {
            res = append(res, r)
            if len(res) == limit {
                break
            }
        }
    }
    return res, nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
    for _, id := range ids {
        if r, ok := m.Records[id]; ok {
            r.SyncStatus = SyncStatusSynced
            r.LastSyncAt = time.Now()
            m.Records[id] = r
        }
    }
    return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    for _, r := range records {
        m.Records[r.ID] = r
    }
    return nil
}

func TestRAGSyncService(t *testing.T) {
    svc := &MockRAGSyncService{
        Records: map[string]RAGSyncRecord{
            "1": {ID: "1", SyncStatus: SyncStatusPending},
            "2": {ID: "2", SyncStatus: SyncStatusPending},
        },
    }

    ctx := context.Background()
    pending, err := svc.FetchPendingSyncs(ctx, 10)
    if err != nil || len(pending) != 2 {
        t.Fatalf("Expected 2 pending syncs, got %d", len(pending))
    }

    err = svc.MarkSynced(ctx, []string{"1"})
    if err != nil {
        t.Fatal(err)
    }

    pending, _ = svc.FetchPendingSyncs(ctx, 10)
    if len(pending) != 1 {
        t.Fatalf("Expected 1 pending sync, got %d", len(pending))
    }
}
