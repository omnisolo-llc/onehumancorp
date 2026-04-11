package hub

import (
    "context"
    "database/sql"
    "os"
    "testing"

    "github.com/onehumancorp/mono/srcs/server/db"
    _ "modernc.org/sqlite"
)

func setupTestDB(t *testing.T) (db.Provider, func()) {
    tmpFile, err := os.CreateTemp("", "testdb-*.sqlite")
    if err != nil {
        t.Fatalf("failed to create temp file: %v", err)
    }

    sqlDB, err := sql.Open("sqlite", tmpFile.Name())
    if err != nil {
        t.Fatalf("failed to open sqlite: %v", err)
    }

    provider := db.NewSqliteProvider(sqlDB)

    // Setup schema
    schema := `
    CREATE TABLE IF NOT EXISTS swarm_memory_embeddings (
        memory_id        TEXT PRIMARY KEY,
        context          TEXT NOT NULL,
        vector_embedding BLOB,
        source_plugin    TEXT,
        created_at       TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        sync_status      VARCHAR(50) DEFAULT 'pending',
        last_sync_at     TIMESTAMP NULL
    );
    `
    if _, err := sqlDB.Exec(schema); err != nil {
        t.Fatalf("failed to create schema: %v", err)
    }

    cleanup := func() {
        sqlDB.Close()
        os.Remove(tmpFile.Name())
    }

    return provider, cleanup
}

func TestRAGSyncService(t *testing.T) {
    provider, cleanup := setupTestDB(t)
    defer cleanup()

    svc := NewRAGSyncService(provider)
    ctx := context.Background()

    // Test ProcessIncomingSync
    records := []RAGSyncRecord{
        {ID: "mem1", Context: "test context 1", Vector: []byte{1, 2, 3}, SyncStatus: SyncStatusPending},
        {ID: "mem2", Context: "test context 2", Vector: []byte{4, 5, 6}, SyncStatus: SyncStatusSynced},
    }
    if err := svc.ProcessIncomingSync(ctx, records); err != nil {
        t.Fatalf("ProcessIncomingSync failed: %v", err)
    }

    // Test FetchPendingSyncs
    pending, err := svc.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("FetchPendingSyncs failed: %v", err)
    }
    if len(pending) != 1 {
        t.Fatalf("expected 1 pending record, got %d", len(pending))
    }
    if pending[0].ID != "mem1" {
        t.Errorf("expected mem1, got %s", pending[0].ID)
    }

    // Test MarkSynced
    if err := svc.MarkSynced(ctx, []string{"mem1"}); err != nil {
        t.Fatalf("MarkSynced failed: %v", err)
    }

    // Verify mem1 is no longer pending
    pendingAfter, err := svc.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("FetchPendingSyncs failed: %v", err)
    }
    if len(pendingAfter) != 0 {
        t.Fatalf("expected 0 pending records, got %d", len(pendingAfter))
    }
}
