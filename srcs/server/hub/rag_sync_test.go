package hub

import (
    "context"
    "testing"
    "time"

    "github.com/onehumancorp/mono/srcs/server/db"
)

func TestRAGSyncService(t *testing.T) {
    t.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory")
    ctx := context.Background()

    provider, err := db.New(ctx)
    if err != nil {
        t.Fatalf("failed to create db provider: %v", err)
    }
    defer provider.Close()

    InitRAGSyncMetrics(nil)

    service := NewRAGSyncService(provider)

    // Setup Schema for test
    _, err = provider.Exec(ctx, `CREATE TABLE swarm_memory_embeddings (
        memory_id        TEXT PRIMARY KEY,
        context          TEXT NOT NULL,
        vector_embedding BLOB,
        source_plugin    TEXT,
        created_at       TEXT DEFAULT CURRENT_TIMESTAMP,
        sync_status      TEXT DEFAULT 'pending',
        last_sync_at     TEXT NULL
    )`)
    if err != nil {
        t.Fatalf("failed to create test schema: %v", err)
    }

    // Insert initial pending records
    _, err = provider.Exec(ctx, `
        INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status)
        VALUES ('mem1', 'context1', NULL, 'pending'), ('mem2', 'context2', NULL, 'pending')
    `)
    if err != nil {
        t.Fatalf("failed to insert initial data: %v", err)
    }

    // FetchPendingSyncs
    records, err := service.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("FetchPendingSyncs failed: %v", err)
    }
    if len(records) != 2 {
        t.Fatalf("expected 2 pending records, got %d", len(records))
    }

    // MarkSynced
    err = service.MarkSynced(ctx, []string{"mem1"})
    if err != nil {
        t.Fatalf("MarkSynced failed: %v", err)
    }

    records, err = service.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("FetchPendingSyncs failed: %v", err)
    }
    if len(records) != 1 {
        t.Fatalf("expected 1 pending record after mark synced, got %d", len(records))
    }

    // ProcessIncomingSync
    newRecords := []RAGSyncRecord{
        {ID: "mem3", Context: "context3", Vector: []byte{1, 2}, SyncStatus: SyncStatusSynced, LastSyncAt: time.Now()},
        {ID: "mem1", Context: "context1_updated", Vector: []byte{3, 4}, SyncStatus: SyncStatusSynced, LastSyncAt: time.Now()},
    }
    err = service.ProcessIncomingSync(ctx, newRecords)
    if err != nil {
        t.Fatalf("ProcessIncomingSync failed: %v", err)
    }
}
