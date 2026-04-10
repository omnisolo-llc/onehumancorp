package hub

import (
    "context"
    "database/sql"
    "testing"
    "time"
    "github.com/onehumancorp/mono/srcs/server/db"
    _ "modernc.org/sqlite"
)

func setupTestDB(t *testing.T) db.Provider {
    d, err := sql.Open("sqlite", ":memory:")
    if err != nil {
        t.Fatalf("failed to open memory db: %v", err)
    }

    provider := db.NewSqliteProvider(d)

    createTable := `
    CREATE TABLE IF NOT EXISTS swarm_memory_embeddings (
        memory_id        TEXT PRIMARY KEY,
        context          TEXT NOT NULL,
        vector_embedding BYTEA,
        source_plugin    TEXT,
        created_at       TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
        sync_status      VARCHAR(50) DEFAULT 'pending',
        last_sync_at     TIMESTAMPTZ NULL
    );`

    _, err = provider.Exec(context.Background(), createTable)
    if err != nil {
        t.Fatalf("failed to create table: %v", err)
    }

    return provider
}

func TestRAGSyncService(t *testing.T) {
    InitRAGSyncMetrics()
    provider := setupTestDB(t)
    defer provider.Close()

    svc := NewRAGSyncService(provider)
    ctx := context.Background()

    // Test ProcessIncomingSync
    now := time.Now().UTC().Truncate(time.Second)
    rec1 := RAGSyncRecord{
        ID:         "mem-1",
        Context:    "test context 1",
        Vector:     []float32{1.1, 2.2},
        SyncStatus: SyncStatusPending,
        LastSyncAt: now,
    }

    err := svc.ProcessIncomingSync(ctx, []RAGSyncRecord{rec1})
    if err != nil {
        t.Fatalf("ProcessIncomingSync failed: %v", err)
    }

    // Test FetchPendingSyncs
    pending, err := svc.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("FetchPendingSyncs failed: %v", err)
    }
    if len(pending) != 1 {
        t.Fatalf("expected 1 pending record, got %d", len(pending))
    }
    if pending[0].ID != "mem-1" || pending[0].Vector[0] != 1.1 {
        t.Fatalf("unexpected record content: %+v", pending[0])
    }

    if !pending[0].LastSyncAt.Equal(now) {
        t.Fatalf("unexpected last sync at: %v != %v", pending[0].LastSyncAt, now)
    }

    // Test MarkSynced
    err = svc.MarkSynced(ctx, []string{"mem-1"})
    if err != nil {
        t.Fatalf("MarkSynced failed: %v", err)
    }

    pendingAfter, err := svc.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("FetchPendingSyncs failed after mark: %v", err)
    }
    if len(pendingAfter) != 0 {
        t.Fatalf("expected 0 pending records after mark, got %d", len(pendingAfter))
    }
}
