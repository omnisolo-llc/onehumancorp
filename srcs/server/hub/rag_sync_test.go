package hub

import (
    "context"
    "testing"
    "github.com/onehumancorp/mono/srcs/server/db"
    _ "modernc.org/sqlite"
    "database/sql"
)

func setupDB(t *testing.T) db.Provider {
    sqlDB, err := sql.Open("sqlite", ":memory:")
    if err != nil {
        t.Fatalf("failed to open sqlite db: %v", err)
    }

    _, err = sqlDB.Exec(`
        CREATE TABLE swarm_memory_embeddings (
            memory_id TEXT PRIMARY KEY,
            context TEXT NOT NULL,
            sync_status VARCHAR(50) DEFAULT 'pending',
            last_sync_at TIMESTAMP NULL
        );
    `)
    if err != nil {
        t.Fatalf("failed to create table: %v", err)
    }

    return db.NewSqliteProvider(sqlDB)
}

func TestDefaultRAGSyncService_FetchPendingSyncs(t *testing.T) {
    provider := setupDB(t)
    svc := NewDefaultRAGSyncService(provider)
    ctx := context.Background()

    // Insert test data
    _, err := provider.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES ('1', 'ctx1', 'pending')")
    if err != nil {
        t.Fatalf("failed to insert data: %v", err)
    }

    records, err := svc.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if len(records) != 1 {
        t.Fatalf("expected 1 record, got %d", len(records))
    }
    if records[0].ID != "1" || records[0].SyncStatus != "pending" {
        t.Errorf("unexpected record data: %+v", records[0])
    }
}

func TestDefaultRAGSyncService_MarkSynced(t *testing.T) {
    provider := setupDB(t)
    svc := NewDefaultRAGSyncService(provider)
    ctx := context.Background()

    // Insert test data
    _, err := provider.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES ('1', 'ctx1', 'pending')")
    if err != nil {
        t.Fatalf("failed to insert data: %v", err)
    }

    err = svc.MarkSynced(ctx, []string{"1"})
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }

    // Verify
    rows, _ := provider.Query(ctx, "SELECT sync_status FROM swarm_memory_embeddings WHERE memory_id = '1'")
    defer rows.Close()
    var status string
    if rows.Next() {
        rows.Scan(&status)
    }
    if status != "synced" {
        t.Errorf("expected status 'synced', got %s", status)
    }
}

func TestDefaultRAGSyncService_ProcessIncomingSync(t *testing.T) {
    provider := setupDB(t)
    svc := NewDefaultRAGSyncService(provider)
    ctx := context.Background()

    // Insert test data
    _, err := provider.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES ('1', 'ctx1', 'pending')")
    if err != nil {
        t.Fatalf("failed to insert data: %v", err)
    }

    err = svc.ProcessIncomingSync(ctx, []RAGSyncRecord{
        {ID: "1", Context: "updated_ctx"},
    })
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }

    // Verify
    rows, _ := provider.Query(ctx, "SELECT context, sync_status FROM swarm_memory_embeddings WHERE memory_id = '1'")
    defer rows.Close()
    var ctxStr, status string
    if rows.Next() {
        rows.Scan(&ctxStr, &status)
    }
    if ctxStr != "updated_ctx" {
        t.Errorf("expected context 'updated_ctx', got %s", ctxStr)
    }
    if status != "synced" {
        t.Errorf("expected status 'synced', got %s", status)
    }
}
