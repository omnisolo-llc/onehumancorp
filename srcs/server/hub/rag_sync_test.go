package hub

import (
    "context"
    "database/sql"
    "testing"

    _ "modernc.org/sqlite"
    "github.com/onehumancorp/mono/srcs/server/db"
)

func TestRAGSyncService(t *testing.T) {
    sqlDB, err := sql.Open("sqlite", "file::memory:?cache=shared")
    if err != nil {
        t.Fatalf("Failed to open sqlite memory db: %v", err)
    }
    defer sqlDB.Close()

    provider := db.NewSqliteProvider(sqlDB)
    defer provider.Close()

    _, err = provider.Exec(context.Background(), `CREATE TABLE swarm_memory_embeddings (
        memory_id TEXT PRIMARY KEY,
        context TEXT NOT NULL,
        vector_embedding BLOB,
        sync_status TEXT DEFAULT 'pending',
        last_sync_at TIMESTAMPTZ NULL
    )`)
    if err != nil {
        t.Fatalf("Failed to create table: %v", err)
    }

    svc := NewRAGSyncService(provider)
    ctx := context.Background()

    _, err = provider.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status) VALUES ('1', 'ctx1', X'00', 'pending')")
    if err != nil {
        t.Fatalf("Failed to insert: %v", err)
    }

    pending, err := svc.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("Failed to fetch pending: %v", err)
    }
    if len(pending) != 1 || pending[0].ID != "1" {
        t.Errorf("Expected 1 pending record with ID 1, got %v", pending)
    }

    err = svc.MarkSynced(ctx, []string{"1"})
    if err != nil {
        t.Fatalf("Failed to mark synced: %v", err)
    }

    pending, err = svc.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("Failed to fetch pending again: %v", err)
    }
    if len(pending) != 0 {
        t.Errorf("Expected 0 pending records, got %v", pending)
    }

    err = svc.ProcessIncomingSync(ctx, []RAGSyncRecord{{ID: "2", Context: "ctx2", Vector: []byte{0x00}, SyncStatus: SyncStatusPending}})
    if err != nil {
        t.Fatalf("Failed to process incoming: %v", err)
    }
    var exists bool
    err = provider.QueryRow(ctx, "SELECT EXISTS(SELECT 1 FROM swarm_memory_embeddings WHERE memory_id = '2')").Scan(&exists)
    if err != nil || !exists {
        t.Fatalf("Expected incoming record to be inserted")
    }
}
