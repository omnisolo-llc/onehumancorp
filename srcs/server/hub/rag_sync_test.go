package hub

import (
    "context"
    "testing"
    "time"
)

type MockRAGSyncService struct {
    records map[string]RAGSyncRecord
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    var res []RAGSyncRecord
    for _, r := range m.records {
        if r.SyncStatus == SyncStatusPending {
            res = append(res, r)
            if len(res) >= limit {
                break
            }
        }
    }
    return res, nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
    for _, id := range ids {
        if r, ok := m.records[id]; ok {
            r.SyncStatus = SyncStatusSynced
            r.LastSyncAt = time.Now()
            m.records[id] = r
        }
    }
    return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    if m.records == nil {
        m.records = make(map[string]RAGSyncRecord)
    }
    for _, r := range records {
        r.SyncStatus = SyncStatusSynced
        r.LastSyncAt = time.Now()
        m.records[r.ID] = r
    }
    return nil
}

func TestMockRAGSyncService(t *testing.T) {
    mock := &MockRAGSyncService{
        records: map[string]RAGSyncRecord{
            "1": {ID: "1", Context: "test 1", SyncStatus: SyncStatusPending},
            "2": {ID: "2", Context: "test 2", SyncStatus: SyncStatusPending},
            "3": {ID: "3", Context: "test 3", SyncStatus: SyncStatusSynced},
        },
    }

    ctx := context.Background()
    pending, _ := mock.FetchPendingSyncs(ctx, 10)
    if len(pending) != 2 {
        t.Errorf("Expected 2 pending syncs, got %d", len(pending))
    }

    _ = mock.MarkSynced(ctx, []string{"1"})
    pending, _ = mock.FetchPendingSyncs(ctx, 10)
    if len(pending) != 1 {
        t.Errorf("Expected 1 pending syncs, got %d", len(pending))
    }

    newRecords := []RAGSyncRecord{
        {ID: "4", Context: "test 4", SyncStatus: SyncStatusPending},
    }
    _ = mock.ProcessIncomingSync(ctx, newRecords)
    if mock.records["4"].SyncStatus != SyncStatusSynced {
         t.Errorf("Expected incoming sync to be marked as synced")
    }
}
