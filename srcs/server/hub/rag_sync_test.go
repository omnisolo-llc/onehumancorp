package hub

import (
    "context"
    "testing"
    "time"

)

type MockRAGSyncService struct {
    Records []RAGSyncRecord
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
    idMap := make(map[string]bool)
    for _, id := range ids {
        idMap[id] = true
    }
    for i, r := range m.Records {
        if idMap[r.ID] {
            m.Records[i].SyncStatus = SyncStatusSynced
            m.Records[i].LastSyncAt = time.Now()
        }
    }
    return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    m.Records = append(m.Records, records...)
    return nil
}

func TestMockRAGSyncService(t *testing.T) {
    ctx := context.Background()
    svc := &MockRAGSyncService{
        Records: []RAGSyncRecord{
            {ID: "1", SyncStatus: SyncStatusPending},
            {ID: "2", SyncStatus: SyncStatusSynced},
            {ID: "3", SyncStatus: SyncStatusPending},
        },
    }

    pending, err := svc.FetchPendingSyncs(ctx, 2)
    if err != nil {
        t.Fatalf("FetchPendingSyncs failed: %v", err)
    }
    if len(pending) != 2 {
        t.Errorf("Expected 2 pending records, got %d", len(pending))
    }

    err = svc.MarkSynced(ctx, []string{"1"})
    if err != nil {
        t.Fatalf("MarkSynced failed: %v", err)
    }

    pending, err = svc.FetchPendingSyncs(ctx, 2)
    if err != nil {
        t.Fatalf("FetchPendingSyncs failed: %v", err)
    }
    if len(pending) != 1 {
        t.Errorf("Expected 1 pending record, got %d", len(pending))
    }
}

func TestInitRAGSyncMetrics(t *testing.T) {
    // We pass nil to verify the nil-check safety
    err := InitRAGSyncMetrics(nil)
    if err != nil {
        t.Fatalf("InitRAGSyncMetrics failed with nil meter: %v", err)
    }
}
