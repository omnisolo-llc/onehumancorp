package hub

import (
    "context"
    "testing"
    "time"
)

type mockRAGSyncService struct {
    records []RAGSyncRecord
}

func (m *mockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    var result []RAGSyncRecord
    for i, r := range m.records {
        if i >= limit {
            break
        }
        if r.SyncStatus == SyncStatusPending {
            result = append(result, r)
        }
    }
    return result, nil
}

func (m *mockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
    idMap := make(map[string]bool)
    for _, id := range ids {
        idMap[id] = true
    }
    for i, r := range m.records {
        if idMap[r.ID] {
            m.records[i].SyncStatus = SyncStatusSynced
        }
    }
    return nil
}

func (m *mockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    m.records = append(m.records, records...)
    return nil
}

func TestRAGSyncServiceMock(t *testing.T) {
    mock := &mockRAGSyncService{}

    records := []RAGSyncRecord{
        {ID: "1", Context: "test1", SyncStatus: SyncStatusPending, LastSyncAt: time.Now()},
        {ID: "2", Context: "test2", SyncStatus: SyncStatusPending, LastSyncAt: time.Now()},
    }

    err := mock.ProcessIncomingSync(context.Background(), records)
    if err != nil {
        t.Fatalf("ProcessIncomingSync failed: %v", err)
    }

    pending, err := mock.FetchPendingSyncs(context.Background(), 10)
    if err != nil {
        t.Fatalf("FetchPendingSyncs failed: %v", err)
    }
    if len(pending) != 2 {
        t.Errorf("Expected 2 pending records, got %d", len(pending))
    }

    err = mock.MarkSynced(context.Background(), []string{"1"})
    if err != nil {
        t.Fatalf("MarkSynced failed: %v", err)
    }

    pending, err = mock.FetchPendingSyncs(context.Background(), 10)
    if err != nil {
        t.Fatalf("FetchPendingSyncs failed: %v", err)
    }
    if len(pending) != 1 {
        t.Errorf("Expected 1 pending record, got %d", len(pending))
    }
}

func TestRAGSyncServiceImpl(t *testing.T) {
    service := NewRAGSyncService()

    err := service.ProcessIncomingSync(context.Background(), nil)
    if err != nil {
        t.Errorf("ProcessIncomingSync unexpected error: %v", err)
    }

    records, err := service.FetchPendingSyncs(context.Background(), 10)
    if err != nil {
        t.Errorf("FetchPendingSyncs unexpected error: %v", err)
    }
    if records != nil {
        t.Errorf("FetchPendingSyncs expected nil, got %v", records)
    }

    err = service.MarkSynced(context.Background(), []string{"1"})
    if err != nil {
        t.Errorf("MarkSynced unexpected error: %v", err)
    }
}
