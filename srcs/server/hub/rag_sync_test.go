package hub

import (
    "context"
    "testing"
    "database/sql"
    _ "modernc.org/sqlite"

    "github.com/onehumancorp/mono/srcs/server/db"
)

func setupTestDB(t *testing.T) db.Provider {
    sqliteDB, err := sql.Open("sqlite", ":memory:")
    if err != nil {
        t.Fatalf("failed to open sqlite: %v", err)
    }

    // Set up the schema
    _, err = sqliteDB.Exec(`
        CREATE TABLE autodream_memories (
            id TEXT PRIMARY KEY,
            content TEXT NOT NULL,
            embedding TEXT,
            sync_status VARCHAR(50) DEFAULT 'pending',
            last_sync_at TIMESTAMP NULL
        );
    `)
    if err != nil {
        t.Fatalf("failed to create table: %v", err)
    }

    return &db.DB{Provider: db.NewSqliteProvider(sqliteDB)}
}

func TestSyncServiceDataFlow(t *testing.T) {
    provider := setupTestDB(t)
    svc := NewSyncService(provider)
    ctx := context.Background()

    // 1. Initial State: Insert some pending records simulating extraction
    _, err := provider.Exec(ctx, `
        INSERT INTO autodream_memories (id, content, sync_status)
        VALUES
            ('1', 'context A', 'pending'),
            ('2', 'context B', 'pending'),
            ('3', 'context C', 'synced')
    `)
    if err != nil {
        t.Fatalf("failed to insert initial records: %v", err)
    }

    // 2. FetchPendingSyncs
    pending, err := svc.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("unexpected error fetching pending: %v", err)
    }
    if len(pending) != 2 {
        t.Errorf("expected 2 pending records, got %d", len(pending))
    }

    // 3. MarkSynced
    err = svc.MarkSynced(ctx, []string{"1"})
    if err != nil {
        t.Fatalf("unexpected error marking synced: %v", err)
    }

    // Verify MarkSynced worked by fetching again
    pendingAfterMark, _ := svc.FetchPendingSyncs(ctx, 10)
    if len(pendingAfterMark) != 1 {
        t.Errorf("expected 1 pending record after mark, got %d", len(pendingAfterMark))
    }

    // 4. ProcessIncomingSync
    incoming := []RAGSyncRecord{
        {
            ID: "4",
            Context: "context D (cloud)",
            SyncStatus: SyncStatusSynced,
            Vector: []float32{1.0, 2.0, 3.0},
        },
    }
    err = svc.ProcessIncomingSync(ctx, incoming)
    if err != nil {
        t.Fatalf("unexpected error processing incoming: %v", err)
    }

    // Verify record was inserted
    row := provider.QueryRow(ctx, "SELECT sync_status, embedding FROM autodream_memories WHERE id = '4'")
    var status string
    var embedding *string
    if err := row.Scan(&status, &embedding); err != nil {
        t.Fatalf("failed to query new record: %v", err)
    }
    if status != "synced" {
        t.Errorf("expected status synced, got %s", status)
    }
    if embedding == nil || *embedding != "[1.000000,2.000000,3.000000]" {
        t.Errorf("expected embedding to match, got %v", embedding)
    }
}
