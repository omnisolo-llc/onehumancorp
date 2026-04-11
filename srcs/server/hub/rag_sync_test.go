package hub

import (
    "context"
    "testing"
    "database/sql"

    "github.com/onehumancorp/mono/srcs/server/db"
    _ "modernc.org/sqlite"
)

func setupTestDB(t *testing.T) db.Provider {
    sqlDB, err := sql.Open("sqlite", ":memory:")
    if err != nil {
        t.Fatalf("failed to open sqlite db: %v", err)
    }

    provider := db.NewSqliteProvider(sqlDB)
    ctx := context.Background()

    _, err = provider.Exec(ctx, `
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
    provider := setupTestDB(t)
    ctx := context.Background()
    service := NewRAGSyncService(provider)

    _, err := provider.Exec(ctx, `
        INSERT INTO autodream_memories (id, content, embedding, sync_status)
        VALUES ('1', 'context 1', '[0.1, 0.2]', 'pending'),
               ('2', 'context 2', '[0.3, 0.4]', 'synced')
    `)
    if err != nil {
        t.Fatalf("failed to insert mock data: %v", err)
    }

    records, err := service.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("FetchPendingSyncs failed: %v", err)
    }

    if len(records) != 1 {
        t.Fatalf("expected 1 record, got %d", len(records))
    }

    if records[0].ID != "1" {
        t.Errorf("expected ID '1', got '%s'", records[0].ID)
    }
}

func TestMarkSynced(t *testing.T) {
    provider := setupTestDB(t)
    ctx := context.Background()
    service := NewRAGSyncService(provider)

    _, err := provider.Exec(ctx, `
        INSERT INTO autodream_memories (id, content, sync_status)
        VALUES ('1', 'context 1', 'pending')
    `)
    if err != nil {
        t.Fatalf("failed to insert mock data: %v", err)
    }

    err = service.MarkSynced(ctx, []string{"1"})
    if err != nil {
        t.Fatalf("MarkSynced failed: %v", err)
    }

    rows, err := provider.Query(ctx, "SELECT sync_status, last_sync_at FROM autodream_memories WHERE id = '1'")
    if err != nil {
        t.Fatalf("Query failed: %v", err)
    }
    defer rows.Close()

    if !rows.Next() {
        t.Fatal("expected 1 row")
    }

    var status string
    var lastSync sql.NullTime
    if err := rows.Scan(&status, &lastSync); err != nil {
        var str string
        if err := rows.Scan(&status, &str); err != nil {
            t.Fatalf("Scan failed: %v", err)
        }
    } else {
         if status != "synced" {
             t.Errorf("expected status 'synced', got '%s'", status)
         }
         if !lastSync.Valid {
             t.Error("expected last_sync_at to be valid")
         }
    }
}

func TestProcessIncomingSync(t *testing.T) {
    provider := setupTestDB(t)
    ctx := context.Background()
    service := NewRAGSyncService(provider)

    records := []RAGSyncRecord{
        {
            ID:      "1",
            Context: "incoming context",
            Vector:  []float32{0.5, 0.6},
        },
    }

    err := service.ProcessIncomingSync(ctx, records)
    if err != nil {
        t.Fatalf("ProcessIncomingSync failed: %v", err)
    }

    rows, err := provider.Query(ctx, "SELECT content, sync_status FROM autodream_memories WHERE id = '1'")
    if err != nil {
        t.Fatalf("Query failed: %v", err)
    }
    defer rows.Close()

    if !rows.Next() {
        t.Fatal("expected 1 row")
    }

    var content, status string
    if err := rows.Scan(&content, &status); err != nil {
        t.Fatalf("Scan failed: %v", err)
    }

    if content != "incoming context" {
        t.Errorf("expected content 'incoming context', got '%s'", content)
    }
    if status != "synced" {
        t.Errorf("expected status 'synced', got '%s'", status)
    }
}
