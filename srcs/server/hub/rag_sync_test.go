package hub

import (
    "context"
    "testing"
    "time"

    _ "modernc.org/sqlite"
    "database/sql"
    "github.com/onehumancorp/mono/srcs/server/db"
)

func TestRAGSyncService(t *testing.T) {
    ctx := context.Background()
    sqldb, err := sql.Open("sqlite", ":memory:")
    if err != nil {
        t.Fatalf("Failed to open sqlite: %v", err)
    }
    provider := db.NewSqliteProvider(sqldb)
    defer provider.Close()

    // Create tables
    _, err = provider.Exec(ctx, `
        CREATE TABLE swarm_memory_embeddings (
            memory_id TEXT PRIMARY KEY,
            context TEXT NOT NULL,
            vector_embedding BLOB,
            sync_status VARCHAR(50) DEFAULT 'pending',
            last_sync_at TIMESTAMP NULL
        )
    `)
    if err != nil {
        t.Fatalf("Failed to create table: %v", err)
    }

    // Insert data
    _, err = provider.Exec(ctx, `INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding) VALUES ('1', 'test context', X'010203')`)
    if err != nil {
        t.Fatalf("Failed to insert data: %v", err)
    }

    service := NewRAGSyncService(provider)

    // Test FetchPendingSyncs
    records, err := service.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("FetchPendingSyncs failed: %v", err)
    }
    if len(records) != 1 {
        t.Fatalf("Expected 1 pending record, got %d", len(records))
    }
    if records[0].SyncStatus != SyncStatusInProgress {
        t.Fatalf("Expected status in_progress, got %v", records[0].SyncStatus)
    }

    // Test MarkSynced
    err = service.MarkSynced(ctx, []string{"1"})
    if err != nil {
        t.Fatalf("MarkSynced failed: %v", err)
    }

    // Verify MarkSynced
    var status string
    err = provider.QueryRow(ctx, "SELECT sync_status FROM swarm_memory_embeddings WHERE memory_id = '1'").Scan(&status)
    if err != nil {
        t.Fatalf("Query failed: %v", err)
    }
    if status != "synced" {
        t.Fatalf("Expected status synced, got %s", status)
    }

    // Test ProcessIncomingSync
    incoming := []RAGSyncRecord{
        {
            ID:         "2",
            Context:    "new context",
            Vector:     []byte{4, 5, 6},
            SyncStatus: SyncStatusSynced,
            LastSyncAt: time.Now(),
        },
        {
            ID:         "1",
            Context:    "updated context",
            Vector:     []byte{7, 8, 9},
            SyncStatus: SyncStatusSynced,
            LastSyncAt: time.Now(),
        },
    }
    err = service.ProcessIncomingSync(ctx, incoming)
    if err != nil {
        t.Fatalf("ProcessIncomingSync failed: %v", err)
    }

    // Verify ProcessIncomingSync
    var count int
    err = provider.QueryRow(ctx, "SELECT COUNT(*) FROM swarm_memory_embeddings").Scan(&count)
    if err != nil {
        t.Fatalf("Query failed: %v", err)
    }
    if count != 2 {
        t.Fatalf("Expected 2 records, got %d", count)
    }

    var ctx1 string
    err = provider.QueryRow(ctx, "SELECT context FROM swarm_memory_embeddings WHERE memory_id = '1'").Scan(&ctx1)
    if err != nil {
        t.Fatalf("Query failed: %v", err)
    }
    if ctx1 != "updated context" {
        t.Fatalf("Expected 'updated context', got %s", ctx1)
    }
}
