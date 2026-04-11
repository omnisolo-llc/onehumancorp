package hub

import (
    "context"
    "testing"
    "time"
)

type mockRAGSyncService struct {
    pendingSyncs []RAGSyncRecord
    syncedIDs    []string
    incoming     []RAGSyncRecord
}

func (m *mockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    if limit > len(m.pendingSyncs) {
        return m.pendingSyncs, nil
    }
    return m.pendingSyncs[:limit], nil
}

func (m *mockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
    m.syncedIDs = append(m.syncedIDs, ids...)
    return nil
}

func (m *mockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    m.incoming = append(m.incoming, records...)
    return nil
}

func TestRAGSyncService(t *testing.T) {
    mockService := &mockRAGSyncService{
        pendingSyncs: []RAGSyncRecord{
            {ID: "1", Context: "Test Context 1", SyncStatus: SyncStatusPending, LastSyncAt: time.Now()},
            {ID: "2", Context: "Test Context 2", SyncStatus: SyncStatusPending, LastSyncAt: time.Now()},
        },
    }

    ctx := context.Background()

    t.Run("FetchPendingSyncs", func(t *testing.T) {
        records, err := mockService.FetchPendingSyncs(ctx, 2)
        if err != nil {
            t.Fatalf("unexpected error: %v", err)
        }
        if len(records) != 2 {
            t.Fatalf("expected 2 records, got %d", len(records))
        }
    })

    t.Run("MarkSynced", func(t *testing.T) {
        idsToSync := []string{"1", "2"}
        err := mockService.MarkSynced(ctx, idsToSync)
        if err != nil {
            t.Fatalf("unexpected error: %v", err)
        }
        if len(mockService.syncedIDs) != 2 {
            t.Fatalf("expected 2 synced IDs, got %d", len(mockService.syncedIDs))
        }
    })

    t.Run("ProcessIncomingSync", func(t *testing.T) {
        incomingRecords := []RAGSyncRecord{
            {ID: "3", Context: "Test Context 3", SyncStatus: SyncStatusSynced, LastSyncAt: time.Now()},
        }
        err := mockService.ProcessIncomingSync(ctx, incomingRecords)
        if err != nil {
            t.Fatalf("unexpected error: %v", err)
        }
        if len(mockService.incoming) != 1 {
            t.Fatalf("expected 1 incoming record, got %d", len(mockService.incoming))
        }
    })
}
