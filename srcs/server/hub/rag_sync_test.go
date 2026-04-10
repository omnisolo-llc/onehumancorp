package hub

import (
    "context"
    "errors"
    "testing"
    "time"
)

// MockRAGSyncService is a mock implementation of RAGSyncService for testing
type MockRAGSyncService struct {
    Records []RAGSyncRecord
    Err     error
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    if m.Err != nil {
        return nil, m.Err
    }

    var pending []RAGSyncRecord
    for _, r := range m.Records {
        if r.SyncStatus == SyncStatusPending {
            pending = append(pending, r)
        }
    }

    if len(pending) > limit {
        pending = pending[:limit]
    }

    return pending, nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
    if m.Err != nil {
        return m.Err
    }

    idMap := make(map[string]bool)
    for _, id := range ids {
        idMap[id] = true
    }

    for i, r := range m.Records {
        if idMap[r.ID] {
            m.Records[i].SyncStatus = SyncStatusSynced
            now := time.Now()
            m.Records[i].LastSyncAt = &now
        }
    }

    return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    if m.Err != nil {
        return m.Err
    }

    m.Records = append(m.Records, records...)
    return nil
}

func TestFetchPendingSyncs(t *testing.T) {
    svc := &MockRAGSyncService{
        Records: []RAGSyncRecord{
            {ID: "1", SyncStatus: SyncStatusPending},
            {ID: "2", SyncStatus: SyncStatusSynced},
            {ID: "3", SyncStatus: SyncStatusPending},
        },
    }

    pending, err := svc.FetchPendingSyncs(context.Background(), 10)
    if err != nil {
        t.Fatalf("expected no error, got %v", err)
    }

    if len(pending) != 2 {
        t.Fatalf("expected 2 pending records, got %d", len(pending))
    }

    if pending[0].ID != "1" || pending[1].ID != "3" {
        t.Fatalf("unexpected pending records: %+v", pending)
    }
}

func TestMarkSynced(t *testing.T) {
    svc := &MockRAGSyncService{
        Records: []RAGSyncRecord{
            {ID: "1", SyncStatus: SyncStatusPending},
            {ID: "2", SyncStatus: SyncStatusPending},
        },
    }

    err := svc.MarkSynced(context.Background(), []string{"1"})
    if err != nil {
        t.Fatalf("expected no error, got %v", err)
    }

    if svc.Records[0].SyncStatus != SyncStatusSynced {
        t.Fatalf("expected record 1 to be synced, got %s", svc.Records[0].SyncStatus)
    }

    if svc.Records[1].SyncStatus != SyncStatusPending {
        t.Fatalf("expected record 2 to be pending, got %s", svc.Records[1].SyncStatus)
    }
}

func TestProcessIncomingSync(t *testing.T) {
    svc := &MockRAGSyncService{}

    records := []RAGSyncRecord{
        {ID: "1", SyncStatus: SyncStatusSynced},
    }

    err := svc.ProcessIncomingSync(context.Background(), records)
    if err != nil {
        t.Fatalf("expected no error, got %v", err)
    }

    if len(svc.Records) != 1 {
        t.Fatalf("expected 1 record, got %d", len(svc.Records))
    }

    if svc.Records[0].ID != "1" {
        t.Fatalf("unexpected record: %+v", svc.Records[0])
    }
}

func TestMockError(t *testing.T) {
    expectedErr := errors.New("mock error")
    svc := &MockRAGSyncService{Err: expectedErr}

    _, err := svc.FetchPendingSyncs(context.Background(), 10)
    if err != expectedErr {
        t.Fatalf("expected error %v, got %v", expectedErr, err)
    }

    err = svc.MarkSynced(context.Background(), []string{"1"})
    if err != expectedErr {
        t.Fatalf("expected error %v, got %v", expectedErr, err)
    }

    err = svc.ProcessIncomingSync(context.Background(), nil)
    if err != expectedErr {
        t.Fatalf("expected error %v, got %v", expectedErr, err)
    }
}
