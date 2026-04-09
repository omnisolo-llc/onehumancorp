package hub

import (
    "context"
    "database/sql"
    "testing"
    "time"

    "github.com/onehumancorp/mono/srcs/server/db"
)

func TestRAGSyncService(t *testing.T) {
    t.Setenv("DATABASE_URL", "sqlite://file::memory:?cache=shared")
    ctx := context.Background()

    database, err := db.New(ctx)
    if err != nil {
        t.Fatalf("failed to create db: %v", err)
    }
    defer database.Close()

    _, err = database.Exec(ctx, `
        CREATE TABLE swarm_memory_embeddings (
            memory_id        TEXT PRIMARY KEY,
            context          TEXT NOT NULL,
            vector_embedding BLOB,
            source_plugin    TEXT,
            sync_status      VARCHAR(50) DEFAULT 'pending',
            last_sync_at     TIMESTAMP NULL
        )
    `)
    if err != nil {
        t.Fatalf("failed to create schema: %v", err)
    }

    service := NewRAGSyncService(database.Provider)

    // Setup initial data
    _, err = database.Exec(ctx, `
        INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status)
        VALUES ('mem1', 'ctx1', 'pending')
    `)
    if err != nil {
        t.Fatalf("failed to insert data: %v", err)
    }

    records, err := service.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("expected no error, got %v", err)
    }
    if len(records) != 1 {
        t.Fatalf("expected 1 record, got %d", len(records))
    }
    if records[0].MemoryID != "mem1" {
        t.Errorf("expected MemoryID 'mem1', got '%s'", records[0].MemoryID)
    }

    err = service.MarkSynced(ctx, []string{"mem1"})
    if err != nil {
        t.Fatalf("expected no error, got %v", err)
    }

    records, err = service.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("expected no error, got %v", err)
    }
    if len(records) != 0 {
        t.Fatalf("expected 0 pending records after MarkSynced, got %d", len(records))
    }

    // Process incoming sync
    incoming := []RAGSyncRecord{
        {
            MemoryID: "mem2",
            Context: "ctx2",
            VectorEmbedding: []byte("vec2"),
            SourcePlugin: sql.NullString{String: "plugin1", Valid: true},
            SyncStatus: SyncStatusSynced,
            LastSyncAt: sql.NullTime{Time: time.Now(), Valid: true},
        },
    }
    err = service.ProcessIncomingSync(ctx, incoming)
    if err != nil {
        t.Fatalf("expected no error processing incoming sync, got %v", err)
    }

    row := database.QueryRow(ctx, "SELECT COUNT(*) FROM swarm_memory_embeddings WHERE memory_id = 'mem2'")
    var count int
    err = row.Scan(&count)
    if err != nil || count != 1 {
        t.Fatalf("expected 1 record for mem2, got %d, err: %v", count, err)
    }
}
