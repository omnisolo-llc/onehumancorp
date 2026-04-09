package hub

import (
    "context"
    "testing"
    "database/sql"

    "github.com/onehumancorp/mono/srcs/server/db"
    _ "modernc.org/sqlite" // Import modernc/sqlite driver for testing
)

func TestHybridRAGSyncService(t *testing.T) {
    ctx := context.Background()

    sqliteDB, err := sql.Open("sqlite", ":memory:")
    if err != nil {
        t.Fatalf("failed to open test sqlite db: %v", err)
    }
    defer sqliteDB.Close()

    provider := db.NewSqliteProvider(sqliteDB)
    dbWrapper := &db.DB{Provider: provider}

    // Apply migrations
    _, err = dbWrapper.Exec(ctx, `
        CREATE TABLE IF NOT EXISTS consolidated_memory (
            id TEXT PRIMARY KEY,
            organization_id TEXT NOT NULL,
            agent_id TEXT,
            content TEXT NOT NULL,
            embedding TEXT,
            source_type TEXT NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            sync_status VARCHAR(50) DEFAULT 'pending',
            last_sync_at TIMESTAMP NULL
        );
    `)
    if err != nil {
        t.Fatalf("Failed to setup test db: %v", err)
    }

    service := NewHybridRAGSyncService(dbWrapper)

    // Test ProcessIncomingSync
    err = service.ProcessIncomingSync(ctx, []RAGSyncRecord{
        {ID: "record1", Context: "test context 1", Vector: []float32{0.1, 0.2}},
        {ID: "record2", Context: "test context 2", Vector: []float32{0.3, 0.4}},
    })
    if err != nil {
        t.Fatalf("Failed to process incoming: %v", err)
    }

    // Insert a pending record
    _, err = dbWrapper.Exec(ctx, `
        INSERT INTO consolidated_memory (id, organization_id, content, source_type, sync_status)
        VALUES ('record3', 'test-org', 'test context 3', 'local', 'pending')
    `)
    if err != nil {
        t.Fatalf("Failed to insert pending: %v", err)
    }

    // Test FetchPendingSyncs
    pending, err := service.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("Failed to fetch pending: %v", err)
    }
    if len(pending) != 1 {
        t.Fatalf("Expected 1 pending record, got %d", len(pending))
    }
    if pending[0].ID != "record3" {
        t.Errorf("Expected ID record3, got %s", pending[0].ID)
    }

    // Test MarkSynced
    err = service.MarkSynced(ctx, []string{"record3"})
    if err != nil {
        t.Fatalf("Failed to mark synced: %v", err)
    }

    pendingAfterSync, err := service.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("Failed to fetch pending after sync: %v", err)
    }
    if len(pendingAfterSync) != 0 {
        t.Fatalf("Expected 0 pending records after sync, got %d", len(pendingAfterSync))
    }
}
