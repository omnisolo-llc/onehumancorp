package hub

import (
    "context"
    "testing"
)

func TestRAGSyncService(t *testing.T) {
    service := NewRAGSyncService()
    ctx := context.Background()

    records, err := service.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if len(records) != 0 {
        t.Fatalf("expected 0 record, got %d", len(records))
    }

    err = service.MarkSynced(ctx, []string{"1"})
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }

    err = service.ProcessIncomingSync(ctx, records)
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
}
