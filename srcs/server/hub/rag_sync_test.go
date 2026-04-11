package hub

import (
    "context"
    "database/sql"
    "os"
    "testing"

    "github.com/onehumancorp/mono/srcs/server/db"
    _ "modernc.org/sqlite"
)

func TestDBRAGSyncService(t *testing.T) {
    tmpFile, err := os.CreateTemp("", "test_db_*.sqlite")
    if err != nil {
        t.Fatalf("failed to create temp db: %v", err)
    }
    defer os.Remove(tmpFile.Name())

    sqlDB, err := sql.Open("sqlite", tmpFile.Name())
    if err != nil {
        t.Fatalf("failed to open sqlite db: %v", err)
    }
    defer sqlDB.Close()

    _, err = sqlDB.Exec(`
        CREATE TABLE swarm_memory_embeddings (
            memory_id TEXT PRIMARY KEY,
            context TEXT NOT NULL,
            vector_embedding BLOB,
            sync_status VARCHAR(50) DEFAULT 'pending',
            last_sync_at TIMESTAMP NULL
        )
    `)
    if err != nil {
        t.Fatalf("failed to create swarm_memory_embeddings table: %v", err)
    }

    _, err = sqlDB.Exec(`
        INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding)
        VALUES
            ('m1', 'context1', NULL),
            ('m2', 'context2', NULL)
    `)
    if err != nil {
        t.Fatalf("failed to insert test data: %v", err)
    }

    sqliteProv := db.NewSqliteProvider(sqlDB)
    dbWrapper := &db.DB{Provider: sqliteProv}
    svc := NewRAGSyncService(dbWrapper)

    ctx := context.Background()

    // Test FetchPendingSyncs
    records, err := svc.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("FetchPendingSyncs failed: %v", err)
    }
    if len(records) != 2 {
        t.Errorf("expected 2 pending records, got %d", len(records))
    }

    // Test MarkSynced
    err = svc.MarkSynced(ctx, []string{"m1"})
    if err != nil {
        t.Fatalf("MarkSynced failed: %v", err)
    }

    records, err = svc.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("FetchPendingSyncs failed: %v", err)
    }
    if len(records) != 1 {
        t.Errorf("expected 1 pending record after marking m1 synced, got %d", len(records))
    }

    // Test ProcessIncomingSync
    incoming := []RAGSyncRecord{
        {ID: "m3", Context: "context3", SyncStatus: SyncStatusSynced},
        {ID: "m1", Context: "context1_updated", SyncStatus: SyncStatusSynced},
    }
    err = svc.ProcessIncomingSync(ctx, incoming)
    if err != nil {
        t.Fatalf("ProcessIncomingSync failed: %v", err)
    }
}
