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
        t.Fatalf("Failed to open sqlite memory db: %v", err)
    }

    _, err = sqliteDB.Exec(`
        CREATE TABLE swarm_memory_embeddings (
            memory_id        TEXT PRIMARY KEY,
            context          TEXT NOT NULL,
            vector_embedding BLOB,
            sync_status      VARCHAR(50) DEFAULT 'pending',
            last_sync_at     TIMESTAMP NULL
        );
    `)
    if err != nil {
        t.Fatalf("Failed to create table: %v", err)
    }

    return db.NewSqliteProvider(sqliteDB)
}

func TestDBRAGSyncService_ProcessIncomingSync(t *testing.T) {
    provider := setupTestDB(t)
    service := NewDBRAGSyncService(provider)
    ctx := context.Background()

    records := []RAGSyncRecord{
        {
            ID:         "1",
            Context:    "Context 1",
            Vector:     []byte{1, 2, 3},
            SyncStatus: SyncStatusPending,
        },
        {
            ID:         "2",
            Context:    "Context 2",
            Vector:     []byte{4, 5, 6},
            SyncStatus: SyncStatusPending,
        },
    }

    err := service.ProcessIncomingSync(ctx, records)
    if err != nil {
        t.Fatalf("Failed to process incoming sync: %v", err)
    }

    // verify records
    var count int
    tx, _ := provider.Begin(ctx)
    row := tx.QueryRow(ctx, "SELECT COUNT(*) FROM swarm_memory_embeddings")
    err = row.Scan(&count)
    if err != nil {
        t.Fatalf("Failed to count: %v", err)
    }
    if count != 2 {
        t.Errorf("Expected 2 records, got %d", count)
    }
    tx.Commit(ctx)
}

func TestDBRAGSyncService_FetchPendingSyncs(t *testing.T) {
    provider := setupTestDB(t)
    service := NewDBRAGSyncService(provider)
    ctx := context.Background()

    // Insert pending syncs directly
    tx, _ := provider.Begin(ctx)
    _, err := tx.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status) VALUES (?, ?, ?, 'pending')", "1", "Ctx 1", []byte{1})
    if err != nil {
        t.Fatalf("Insert failed: %v", err)
    }
    _, err = tx.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status) VALUES (?, ?, ?, 'pending')", "2", "Ctx 2", []byte{2})
    if err != nil {
        t.Fatalf("Insert failed: %v", err)
    }
    tx.Commit(ctx)

    pending, err := service.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("Failed to fetch pending syncs: %v", err)
    }
    if len(pending) != 2 {
        t.Errorf("Expected 2 pending syncs, got %d", len(pending))
    }
}

func TestDBRAGSyncService_MarkSynced(t *testing.T) {
    provider := setupTestDB(t)
    service := NewDBRAGSyncService(provider)
    ctx := context.Background()

    // Insert pending sync
    tx, _ := provider.Begin(ctx)
    _, err := tx.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status) VALUES (?, ?, ?, 'pending')", "1", "Ctx 1", []byte{1})
    if err != nil {
        t.Fatalf("Insert failed: %v", err)
    }
    tx.Commit(ctx)

    err = service.MarkSynced(ctx, []string{"1"})
    if err != nil {
        t.Fatalf("Failed to mark synced: %v", err)
    }

    pending, err := service.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("Failed to fetch pending syncs: %v", err)
    }
    if len(pending) != 0 {
        t.Errorf("Expected 0 pending syncs, got %d", len(pending))
    }
}
