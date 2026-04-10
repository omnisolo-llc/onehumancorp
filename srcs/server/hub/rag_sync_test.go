package hub

import (
    "context"
    "database/sql"
    "testing"


    "github.com/onehumancorp/mono/srcs/server/db"
    _ "modernc.org/sqlite" // Requires SQLite driver
)

func TestRAGSyncServiceDB(t *testing.T) {
    // Setup in-memory sqlite provider using modernc.org/sqlite
    sqlDB, err := sql.Open("sqlite", ":memory:")
    if err != nil {
        t.Fatalf("failed to open sqlite: %v", err)
    }
    defer sqlDB.Close()

    provider := db.NewSqliteProvider(sqlDB)

    // Create necessary table since NewSqliteProvider doesn't run migrations directly here
    createTableQuery := `CREATE TABLE autodream_memories (
        id TEXT PRIMARY KEY,
        content TEXT NOT NULL,
        embedding TEXT,
        sync_status VARCHAR(50) DEFAULT 'pending',
        last_sync_at TIMESTAMP NULL
    );`

    _, err = provider.Exec(context.Background(), createTableQuery)
    if err != nil {
        t.Fatalf("failed to create table: %v", err)
    }

    service := NewRAGSyncService(provider)
    ctx := context.Background()

    // Test ProcessIncomingSync
    records := []RAGSyncRecord{
        {ID: "1", Context: "test 1", Vector: []float32{0.1, 0.2}},
        {ID: "2", Context: "test 2", Vector: []float32{0.3, 0.4}},
    }

    err = service.ProcessIncomingSync(ctx, records)
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }

    // Since ProcessIncomingSync marks as synced, let's insert a pending record manually
    _, err = provider.Exec(ctx, `INSERT INTO autodream_memories (id, content, embedding, sync_status) VALUES ('3', 'test 3', '[0.5, 0.6]', 'pending')`)
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }

    // Test FetchPendingSyncs
    pending, err := service.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }

    if len(pending) != 1 {
        t.Errorf("expected 1 pending record, got %d", len(pending))
    }

    if len(pending) == 1 {
        if pending[0].ID != "3" {
            t.Errorf("expected id 3, got %s", pending[0].ID)
        }
        if len(pending[0].Vector) != 2 || pending[0].Vector[0] != 0.5 {
            t.Errorf("expected vector to be parsed correctly, got %v", pending[0].Vector)
        }
    }

    // Test MarkSynced
    err = service.MarkSynced(ctx, []string{"3"})
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }

    pendingAfter, err := service.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }

    if len(pendingAfter) != 0 {
        t.Errorf("expected 0 pending records after MarkSynced, got %d", len(pendingAfter))
    }
}
