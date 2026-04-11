package hub

import (
    "context"
    "testing"
    "database/sql"

    "github.com/onehumancorp/mono/srcs/server/db"
    _ "modernc.org/sqlite"
)

func TestRAGSyncService(t *testing.T) {
    sqliteDB, err := sql.Open("sqlite", "file::memory:?cache=shared")
    if err != nil {
        t.Fatalf("failed to open sqlite: %v", err)
    }
    defer sqliteDB.Close()

    provider := db.NewSqliteProvider(sqliteDB)
    ctx := context.Background()

    // Setup schema since it's an in-memory db
    _, err = provider.Exec(ctx, "CREATE TABLE swarm_memory_embeddings (memory_id TEXT PRIMARY KEY, context TEXT, vector_embedding BLOB, sync_status TEXT DEFAULT 'pending', last_sync_at TIMESTAMP NULL)")
    if err != nil {
        t.Fatalf("failed to create table: %v", err)
    }

    service := NewRAGSyncService(provider)

    // Test ProcessIncomingSync
    err = service.ProcessIncomingSync(ctx, []RAGSyncRecord{
        {ID: "m1", Context: "ctx1", Vector: []byte{1, 2, 3}, SyncStatus: SyncStatusPending},
    })
    if err != nil {
        t.Fatalf("expected no error, got %v", err)
    }

    // Test FetchPendingSyncs
    records, err := service.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("expected no error, got %v", err)
    }
    if len(records) != 1 {
        t.Fatalf("expected 1 record, got %d", len(records))
    }

    // Test MarkSynced
    err = service.MarkSynced(ctx, []string{"m1"})
    if err != nil {
        t.Fatalf("expected no error, got %v", err)
    }
}
