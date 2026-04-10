package hybrid_sync

import (
    "context"
    "database/sql"
    "testing"
    "time"

    "github.com/onehumancorp/mono/srcs/server/db"
    _ "modernc.org/sqlite"
)

func TestRAGSyncServiceImpl_FetchPendingSyncs(t *testing.T) {
    dbInstance, err := sql.Open("sqlite", ":memory:")
    if err != nil {
        t.Fatalf("failed to open sqlite: %v", err)
    }
    defer dbInstance.Close()

    provider := db.NewSqliteProvider(dbInstance)
    ctx := context.Background()

    _, err = provider.Exec(ctx, `
        CREATE TABLE autodream_memories (
            id TEXT PRIMARY KEY,
            content TEXT NOT NULL,
            embedding BLOB,
            sync_status VARCHAR(50) DEFAULT 'pending',
            last_sync_at TIMESTAMP NULL
        )
    `)
    if err != nil {
        t.Fatalf("failed to create table: %v", err)
    }

    _, err = provider.Exec(ctx, `
        INSERT INTO autodream_memories (id, content, sync_status)
        VALUES ('1', 'test content 1', 'pending'),
               ('2', 'test content 2', 'pending'),
               ('3', 'test content 3', 'synced')
    `)
    if err != nil {
        t.Fatalf("failed to insert data: %v", err)
    }

    service := NewRAGSyncService(provider)

    records, err := service.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("FetchPendingSyncs failed: %v", err)
    }

    if len(records) != 2 {
        t.Errorf("expected 2 records, got %d", len(records))
    }
}

func TestRAGSyncServiceImpl_MarkSynced(t *testing.T) {
    dbInstance, err := sql.Open("sqlite", ":memory:")
    if err != nil {
        t.Fatalf("failed to open sqlite: %v", err)
    }
    defer dbInstance.Close()

    provider := db.NewSqliteProvider(dbInstance)
    ctx := context.Background()

    _, err = provider.Exec(ctx, `
        CREATE TABLE autodream_memories (
            id TEXT PRIMARY KEY,
            content TEXT NOT NULL,
            embedding BLOB,
            sync_status VARCHAR(50) DEFAULT 'pending',
            last_sync_at TIMESTAMP NULL
        )
    `)
    if err != nil {
        t.Fatalf("failed to create table: %v", err)
    }

    _, err = provider.Exec(ctx, `
        INSERT INTO autodream_memories (id, content, sync_status)
        VALUES ('1', 'test content 1', 'pending'),
               ('2', 'test content 2', 'pending')
    `)
    if err != nil {
        t.Fatalf("failed to insert data: %v", err)
    }

    service := NewRAGSyncService(provider)

    err = service.MarkSynced(ctx, []string{"1"})
    if err != nil {
        t.Fatalf("MarkSynced failed: %v", err)
    }

    var status string
    err = provider.QueryRow(ctx, "SELECT sync_status FROM autodream_memories WHERE id = '1'").Scan(&status)
    if err != nil {
        t.Fatalf("failed to query status: %v", err)
    }

    if status != "synced" {
        t.Errorf("expected synced status, got %s", status)
    }
}

func TestRAGSyncServiceImpl_ProcessIncomingSync(t *testing.T) {
    dbInstance, err := sql.Open("sqlite", ":memory:")
    if err != nil {
        t.Fatalf("failed to open sqlite: %v", err)
    }
    defer dbInstance.Close()

    provider := db.NewSqliteProvider(dbInstance)
    ctx := context.Background()

    _, err = provider.Exec(ctx, `
        CREATE TABLE autodream_memories (
            id TEXT PRIMARY KEY,
            content TEXT NOT NULL,
            embedding BLOB,
            sync_status VARCHAR(50) DEFAULT 'pending',
            last_sync_at TIMESTAMP NULL
        )
    `)
    if err != nil {
        t.Fatalf("failed to create table: %v", err)
    }

    service := NewRAGSyncService(provider)

    now := time.Now()
    records := []RAGSyncRecord{
        {ID: "1", Context: "test content", SyncStatus: "synced", LastSyncAt: &now},
    }

    err = service.ProcessIncomingSync(ctx, records)
    if err != nil {
        t.Fatalf("ProcessIncomingSync failed: %v", err)
    }

    var count int
    err = provider.QueryRow(ctx, "SELECT COUNT(*) FROM autodream_memories").Scan(&count)
    if err != nil {
        t.Fatalf("failed to query count: %v", err)
    }

    if count != 1 {
        t.Errorf("expected 1 record, got %d", count)
    }
}
