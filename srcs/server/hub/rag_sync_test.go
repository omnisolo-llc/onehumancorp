package hub

import (
    "context"
    "database/sql"
    "testing"

    "github.com/onehumancorp/mono/srcs/server/db"
    _ "modernc.org/sqlite"
)

func TestRAGSyncService(t *testing.T) {
    ctx := context.Background()
    sqliteDB, err := sql.Open("sqlite", ":memory:")
    if err != nil {
        t.Fatalf("Failed to open sqlite: %v", err)
    }
    provider := db.NewSqliteProvider(sqliteDB)

    _, err = provider.Exec(ctx, `
        CREATE TABLE swarm_memory_embeddings (
            memory_id TEXT PRIMARY KEY,
            context TEXT NOT NULL,
            vector_embedding BLOB,
            source_plugin TEXT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            sync_status VARCHAR(50) DEFAULT 'pending',
            last_sync_at TIMESTAMP NULL
        )
    `)
    if err != nil {
        t.Fatalf("Failed to create table: %v", err)
    }

    svc := NewRAGSyncService(provider)

    records := []RAGSyncRecord{
        {ID: "mem1", Context: "context1", Vector: []float32{0.1, 0.2}},
    }

    if err := svc.ProcessIncomingSync(ctx, records); err != nil {
        t.Fatalf("ProcessIncomingSync failed: %v", err)
    }

    _, err = provider.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES ('mem2', 'context2', 'pending')")
    if err != nil {
        t.Fatalf("Failed to insert pending record: %v", err)
    }

    pending, err := svc.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("FetchPendingSyncs failed: %v", err)
    }
    if len(pending) != 1 || pending[0].ID != "mem2" {
        t.Fatalf("Expected 1 pending record mem2, got: %+v", pending)
    }

    if err := svc.MarkSynced(ctx, []string{"mem2"}); err != nil {
        t.Fatalf("MarkSynced failed: %v", err)
    }

    pendingAfter, _ := svc.FetchPendingSyncs(ctx, 10)
    if len(pendingAfter) != 0 {
        t.Fatalf("Expected 0 pending records after MarkSynced, got: %d", len(pendingAfter))
    }
}
