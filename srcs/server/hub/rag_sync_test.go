package hub_test

import (
    "context"
    "database/sql"
    "testing"
    "time"

    _ "modernc.org/sqlite"

    "github.com/onehumancorp/mono/srcs/server/db"
    "github.com/onehumancorp/mono/srcs/server/hub"
)

func setupDB(t *testing.T) db.Provider {
    sqlDB, err := sql.Open("sqlite", ":memory:")
    if err != nil {
        t.Fatalf("failed to open sqlite: %v", err)
    }

    provider := db.NewSqliteProvider(sqlDB)

    _, err = provider.Exec(context.Background(), `
        CREATE TABLE autodream_memories (
            id TEXT PRIMARY KEY,
            content TEXT NOT NULL,
            embedding TEXT,
            sync_status VARCHAR(50) DEFAULT 'pending',
            last_sync_at TIMESTAMPTZ NULL
        )
    `)

    if err != nil {
        t.Fatalf("failed to create table: %v", err)
    }

    return provider
}

func TestFetchPendingSyncs(t *testing.T) {
    provider := setupDB(t)
    defer provider.Close()
    ctx := context.Background()

    svc := hub.NewRAGSyncService(provider)

    // insert mock data
    _, err := provider.Exec(ctx, "INSERT INTO autodream_memories (id, content, embedding, sync_status) VALUES ($1, $2, $3, $4)", "1", "test_content", "[0.1, 0.2, 0.3]", "pending")
    if err != nil {
        t.Fatalf("failed to insert: %v", err)
    }

    records, err := svc.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("failed to fetch pending syncs: %v", err)
    }

    if len(records) != 1 {
        t.Fatalf("expected 1 record, got %d", len(records))
    }
    if records[0].ID != "1" {
        t.Fatalf("expected ID 1, got %s", records[0].ID)
    }
}

func TestMarkSynced(t *testing.T) {
    provider := setupDB(t)
    defer provider.Close()
    ctx := context.Background()

    svc := hub.NewRAGSyncService(provider)

    // insert mock data
    _, err := provider.Exec(ctx, "INSERT INTO autodream_memories (id, content, embedding, sync_status) VALUES ($1, $2, $3, $4)", "1", "test_content", "[0.1, 0.2, 0.3]", "pending")
    if err != nil {
        t.Fatalf("failed to insert: %v", err)
    }

    err = svc.MarkSynced(ctx, []string{"1"})
    if err != nil {
        t.Fatalf("failed to mark synced: %v", err)
    }

    // Verify
    row := provider.QueryRow(ctx, "SELECT sync_status FROM autodream_memories WHERE id = $1", "1")
    var status string
    if err := row.Scan(&status); err != nil {
        t.Fatalf("failed to scan: %v", err)
    }
    if status != "synced" {
        t.Fatalf("expected status synced, got %s", status)
    }
}

func TestProcessIncomingSync(t *testing.T) {
    provider := setupDB(t)
    defer provider.Close()
    ctx := context.Background()

    svc := hub.NewRAGSyncService(provider)

    records := []hub.RAGSyncRecord{
        {
            ID: "1",
            Context: "incoming test",
            SyncStatus: hub.SyncStatusSynced,
            LastSyncAt: time.Now(),
        },
    }

    err := svc.ProcessIncomingSync(ctx, records)
    if err != nil {
        t.Fatalf("failed to process incoming sync: %v", err)
    }

    row := provider.QueryRow(ctx, "SELECT content FROM autodream_memories WHERE id = $1", "1")
    var content string
    if err := row.Scan(&content); err != nil {
        t.Fatalf("failed to scan: %v", err)
    }
    if content != "incoming test" {
        t.Fatalf("expected 'incoming test', got %s", content)
    }
}
