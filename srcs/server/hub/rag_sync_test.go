package hub

import (
    "context"
    "database/sql"
    "testing"

    "github.com/onehumancorp/mono/srcs/server/db"
    _ "modernc.org/sqlite"
)

func TestRAGSyncService(t *testing.T) {
    sqlDB, err := sql.Open("sqlite", ":memory:")
    if err != nil {
        t.Fatalf("Failed to open sqlite db: %v", err)
    }
    defer sqlDB.Close()

    provider := db.NewSqliteProvider(sqlDB)

    ctx := context.Background()

    _, err = sqlDB.ExecContext(ctx, `
    CREATE TABLE autodream_memories (
        id TEXT PRIMARY KEY,
        content TEXT NOT NULL,
        sync_status VARCHAR(50) DEFAULT 'pending',
        last_sync_at TIMESTAMP NULL
    );`)
    if err != nil {
        t.Fatalf("Failed to create table: %v", err)
    }

    service := NewRAGSyncService(provider)

    // Test ProcessIncomingSync
    records := []RAGSyncRecord{
        {ID: "test-id-1", Context: "Test Context 1", SyncStatus: SyncStatusPending},
        {ID: "test-id-2", Context: "Test Context 2", SyncStatus: SyncStatusPending},
    }
    err = service.ProcessIncomingSync(ctx, records)
    if err != nil {
        t.Fatalf("ProcessIncomingSync failed: %v", err)
    }

    // Test FetchPendingSyncs
    pending, err := service.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("FetchPendingSyncs failed: %v", err)
    }
    if len(pending) != 2 {
        t.Errorf("Expected 2 pending records, got %d", len(pending))
    }

    // Test MarkSynced
    err = service.MarkSynced(ctx, []string{"test-id-1", "test-id-2"})
    if err != nil {
        t.Fatalf("MarkSynced failed: %v", err)
    }

    // Verify they are synced
    pending, err = service.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("FetchPendingSyncs failed: %v", err)
    }
    if len(pending) != 0 {
        t.Errorf("Expected 0 pending records, got %d", len(pending))
    }
}
