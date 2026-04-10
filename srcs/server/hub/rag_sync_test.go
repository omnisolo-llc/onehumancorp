package hub

import (
    "context"
    "testing"
)

type mockRAGSyncService struct {
    pending []RAGSyncRecord
    synced  []string
    process []RAGSyncRecord
}

func (m *mockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    if limit > len(m.pending) {
        return m.pending, nil
    }
    return m.pending[:limit], nil
}

func (m *mockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
    m.synced = append(m.synced, ids...)
    return nil
}

func (m *mockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    m.process = append(m.process, records...)
    return nil
}

func TestRAGSyncService(t *testing.T) {
    mock := &mockRAGSyncService{
        pending: []RAGSyncRecord{{ID: "1", SyncStatus: SyncStatusPending}},
    }

    records, err := mock.FetchPendingSyncs(context.Background(), 10)
    if err != nil || len(records) != 1 {
        t.Errorf("FetchPendingSyncs failed")
    }

    err = mock.MarkSynced(context.Background(), []string{"1"})
    if err != nil || len(mock.synced) != 1 {
        t.Errorf("MarkSynced failed")
    }

    err = mock.ProcessIncomingSync(context.Background(), records)
    if err != nil || len(mock.process) != 1 {
        t.Errorf("ProcessIncomingSync failed")
    }
}
