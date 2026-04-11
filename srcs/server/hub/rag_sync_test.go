package hub

import (
    "context"
    "database/sql"
    "os"
    "testing"
    "time"

    "github.com/onehumancorp/mono/srcs/server/db"
    _ "modernc.org/sqlite"
)

func TestRAGSyncService(t *testing.T) {
    f, err := os.CreateTemp("", "testdb-*.sqlite")
    if err != nil {
        t.Fatalf("failed to create temp file: %v", err)
    }
    f.Close()
    defer os.Remove(f.Name())

    sqlDB, err := sql.Open("sqlite", f.Name())
    if err != nil {
        t.Fatalf("failed to open sqlite: %v", err)
    }
    defer sqlDB.Close()

    _, err = sqlDB.Exec(`
        CREATE TABLE IF NOT EXISTS swarm_memory_embeddings (
            memory_id        TEXT PRIMARY KEY,
            context          TEXT NOT NULL,
            vector_embedding BLOB,
            source_plugin    TEXT,
            created_at       DATETIME DEFAULT CURRENT_TIMESTAMP,
            sync_status      VARCHAR(50) DEFAULT 'pending',
            last_sync_at     DATETIME NULL
        );
    `)
    if err != nil {
        t.Fatalf("failed to create table: %v", err)
    }

    provider := db.NewSqliteProvider(sqlDB)
    service := NewRAGSyncService(provider)
    ctx := context.Background()

    records := []RAGSyncRecord{
        {
            ID:         "mem1",
            Context:    "test context",
            Vector:     []byte{1, 2, 3},
            SyncStatus: SyncStatusPending,
            LastSyncAt: time.Now(),
        },
    }
    err = service.ProcessIncomingSync(ctx, records)
    if err != nil {
        t.Fatalf("ProcessIncomingSync failed: %v", err)
    }

    pending, err := service.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("FetchPendingSyncs failed: %v", err)
    }
    if len(pending) != 1 {
        t.Fatalf("expected 1 pending record, got %d", len(pending))
    }
    if pending[0].ID != "mem1" {
        t.Fatalf("expected ID mem1, got %s", pending[0].ID)
    }

    err = service.MarkSynced(ctx, []string{"mem1"})
    if err != nil {
        t.Fatalf("MarkSynced failed: %v", err)
    }

    pendingAfter, err := service.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("FetchPendingSyncs failed: %v", err)
    }
    if len(pendingAfter) != 0 {
        t.Fatalf("expected 0 pending records after MarkSynced, got %d", len(pendingAfter))
    }
}
