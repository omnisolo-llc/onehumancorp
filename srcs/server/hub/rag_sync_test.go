package hub

import (
    "context"
    "database/sql"
    "path/filepath"
    "testing"

    "github.com/onehumancorp/mono/srcs/server/db"
    _ "modernc.org/sqlite"
)

func TestRAGSyncService(t *testing.T) {
    tempDir := t.TempDir()
    dbPath := filepath.Join(tempDir, "test.db")

    sqlDB, err := sql.Open("sqlite", dbPath)
    if err != nil {
        t.Fatalf("failed to open sqlite: %v", err)
    }
    defer sqlDB.Close()

    provider := db.NewSqliteProvider(sqlDB)

    ctx := context.Background()

    _, err = provider.Exec(ctx, "CREATE TABLE swarm_memory_embeddings (memory_id TEXT PRIMARY KEY, context TEXT NOT NULL, vector_embedding BLOB, sync_status VARCHAR(50) DEFAULT 'pending', last_sync_at TIMESTAMP NULL)")
    if err != nil {
        t.Fatalf("failed to create table: %v", err)
    }

    svc := NewRAGSyncService(provider)

    records := []RAGSyncRecord{
        {ID: "m1", Context: "ctx1", Vector: []byte{1, 2, 3}},
        {ID: "m2", Context: "ctx2", Vector: []byte{4, 5, 6}},
    }

    err = svc.ProcessIncomingSync(ctx, records)
    if err != nil {
        t.Fatalf("ProcessIncomingSync failed: %v", err)
    }

    _, err = provider.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES ('m3', 'ctx3', 'pending')")
    if err != nil {
        t.Fatalf("failed to insert pending record: %v", err)
    }

    pending, err := svc.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("FetchPendingSyncs failed: %v", err)
    }

    if len(pending) != 1 || pending[0].ID != "m3" {
        t.Fatalf("expected 1 pending record 'm3', got %v", pending)
    }

    err = svc.MarkSynced(ctx, []string{"m3"})
    if err != nil {
        t.Fatalf("MarkSynced failed: %v", err)
    }

    pendingAfter, err := svc.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("FetchPendingSyncs after MarkSynced failed: %v", err)
    }

    if len(pendingAfter) != 0 {
        t.Fatalf("expected 0 pending records after MarkSynced, got %v", pendingAfter)
    }
}
