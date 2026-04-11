package hub

import (
    "context"
    "testing"
    "time"
)

func TestMockRAGSyncService(t *testing.T) {
    mock := &MockRAGSyncService{}
    ctx := context.Background()

    records, err := mock.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if len(records) != 0 {
        t.Fatalf("expected 0 records, got %d", len(records))
    }

    err = mock.MarkSynced(ctx, []string{"1"})
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }

    err = mock.ProcessIncomingSync(ctx, []RAGSyncRecord{
        {
            ID:         "1",
            Context:    "test",
            Vector:     []byte{1, 2, 3},
            SyncStatus: SyncStatusPending,
            LastSyncAt: time.Now(),
        },
    })
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
}
