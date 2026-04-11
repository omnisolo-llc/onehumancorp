package hub

import (
    "context"
    "testing"
    "database/sql"

    "github.com/onehumancorp/mono/srcs/server/db"
    _ "modernc.org/sqlite"
)

func TestRAGSyncServiceImpl(t *testing.T) {
    sqliteDB, err := sql.Open("sqlite", "file::memory:?cache=shared")
    if err != nil {
        t.Fatalf("failed to open sqlite: %v", err)
    }
    defer sqliteDB.Close()

    provider := db.NewSqliteProvider(sqliteDB)

    // Create schema manually
    createSchema := `
        DROP TABLE IF EXISTS autodream_memories;
        CREATE TABLE autodream_memories (
            id TEXT PRIMARY KEY,
            content TEXT NOT NULL,
            embedding TEXT,
            source_mission_id TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            organization_id TEXT,
            agent_id TEXT,
            source_type TEXT,
            sync_status VARCHAR(50) DEFAULT 'pending',
            last_sync_at DATETIME NULL
        );
    `
    _, err = sqliteDB.Exec(createSchema)
    if err != nil {
        t.Fatalf("failed to create schema: %v", err)
    }

    // Insert dummy data
    insertQuery := `INSERT INTO autodream_memories (id, content, sync_status) VALUES ('1', 'test content 1', 'pending'), ('2', 'test content 2', 'pending')`
    _, err = sqliteDB.Exec(insertQuery)
    if err != nil {
        t.Fatalf("failed to insert data: %v", err)
    }

    svc := NewRAGSyncService(provider)
    ctx := context.Background()

    t.Run("FetchPendingSyncs", func(t *testing.T) {
        records, err := svc.FetchPendingSyncs(ctx, 10)
        if err != nil {
            t.Fatalf("unexpected error: %v", err)
        }
        if len(records) != 2 {
            t.Fatalf("expected 2 records, got %d", len(records))
        }
    })

    t.Run("MarkSynced", func(t *testing.T) {
        err := svc.MarkSynced(ctx, []string{"1"})
        if err != nil {
            t.Fatalf("unexpected error: %v", err)
        }

        var status string
        err = sqliteDB.QueryRow("SELECT sync_status FROM autodream_memories WHERE id = '1'").Scan(&status)
        if err != nil {
            t.Fatalf("failed to check status: %v", err)
        }
        if status != "synced" {
            t.Fatalf("expected status 'synced', got %s", status)
        }
    })

    t.Run("ProcessIncomingSync", func(t *testing.T) {
        records := []RAGSyncRecord{
            {ID: "3", Context: "new incoming sync"},
            {ID: "2", Context: "updated incoming sync"},
        }
        err := svc.ProcessIncomingSync(ctx, records)
        if err != nil {
            t.Fatalf("unexpected error: %v", err)
        }

        var count int
        err = sqliteDB.QueryRow("SELECT COUNT(*) FROM autodream_memories").Scan(&count)
        if err != nil {
            t.Fatalf("failed to count records: %v", err)
        }
        if count != 3 {
             t.Fatalf("expected 3 records, got %d", count)
        }

        var status string
        err = sqliteDB.QueryRow("SELECT sync_status FROM autodream_memories WHERE id = '3'").Scan(&status)
        if err != nil {
            t.Fatalf("failed to get status: %v", err)
        }
        if status != "synced" {
             t.Fatalf("expected synced, got %s", status)
        }
        err = sqliteDB.QueryRow("SELECT sync_status FROM autodream_memories WHERE id = '2'").Scan(&status)
        if err != nil {
            t.Fatalf("failed to get status: %v", err)
        }
        if status != "synced" {
             t.Fatalf("expected synced, got %s", status)
        }
    })
}
