package hub

import (
    "context"
    "testing"
    "database/sql"
    "github.com/onehumancorp/mono/srcs/server/db"
    _ "modernc.org/sqlite"
)

func TestRAGSyncService(t *testing.T) {
    sqliteDB, err := sql.Open("sqlite", ":memory:")
    if err != nil {
        t.Fatalf("failed to open memory db: %v", err)
    }
    defer sqliteDB.Close()

    dbProv := &db.DB{Provider: db.NewSqliteProvider(sqliteDB)}

    ctx := context.Background()

    _, err = dbProv.Exec(ctx, `
        CREATE TABLE swarm_memory_embeddings (
            memory_id TEXT PRIMARY KEY,
            context TEXT NOT NULL,
            vector_embedding BLOB,
            source_plugin TEXT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            sync_status VARCHAR(50) DEFAULT 'pending',
            last_sync_at TIMESTAMP
        )
    `)
    if err != nil {
        t.Fatalf("failed to create table: %v", err)
    }

    _, err = dbProv.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES ('1', 'ctx1', 'pending')")
    if err != nil {
        t.Fatalf("failed to insert initial record: %v", err)
    }

    svc := NewRAGSyncService(dbProv)

    records, err := svc.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if len(records) != 1 {
        t.Fatalf("expected 1 record, got %d", len(records))
    }
    if records[0].ID != "1" {
        t.Errorf("expected id 1, got %s", records[0].ID)
    }

    err = svc.MarkSynced(ctx, []string{"1"})
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }

    // Verify it was marked synced
    records, err = svc.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if len(records) != 0 {
        t.Fatalf("expected 0 records after MarkSynced, got %d", len(records))
    }

    incoming := []RAGSyncRecord{
        {ID: "2", Context: "ctx2"},
    }
    err = svc.ProcessIncomingSync(ctx, incoming)
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
}
