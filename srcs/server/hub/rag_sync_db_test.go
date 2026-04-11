package hub

import (
    "context"
    "database/sql"
    "os"
    "testing"

    "github.com/onehumancorp/mono/srcs/server/db"
    _ "modernc.org/sqlite"
)

func TestDBAGSyncService_RealDB(t *testing.T) {
    tmpFile, err := os.CreateTemp("", "testdb-*.sqlite")
    if err != nil {
        t.Fatalf("failed to create temp db: %v", err)
    }
    defer os.Remove(tmpFile.Name())

    sqlDB, err := sql.Open("sqlite", tmpFile.Name())
    if err != nil {
        t.Fatalf("failed to open sqlite: %v", err)
    }
    defer sqlDB.Close()

    provider := db.NewSqliteProvider(sqlDB)

    // Setup schema
    ctx := context.Background()
    _, err = provider.Exec(ctx, `
        CREATE TABLE swarm_memory_embeddings (
            memory_id        TEXT PRIMARY KEY,
            context          TEXT NOT NULL,
            vector_embedding BLOB,
            source_plugin    TEXT,
            sync_status      VARCHAR(50) DEFAULT 'pending',
            last_sync_at     TIMESTAMP NULL,
            created_at       TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );
    `)
    if err != nil {
        t.Fatalf("failed to create table: %v", err)
    }

    svc := NewDBAGSyncService(provider)

    // 1. ProcessIncomingSync
    records := []RAGSyncRecord{
        {ID: "mem1", Context: "some context", Vector: []byte{1, 2, 3}},
        {ID: "mem2", Context: "other context", Vector: []byte{4, 5, 6}},
    }
    if err := svc.ProcessIncomingSync(ctx, records); err != nil {
        t.Fatalf("ProcessIncomingSync failed: %v", err)
    }

    // Insert pending records
    _, err = provider.Exec(ctx, `INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES ('mem3', 'pending ctx', 'pending')`)
    if err != nil {
        t.Fatalf("failed to insert pending record: %v", err)
    }

    // 2. FetchPendingSyncs
    fetched, err := svc.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("FetchPendingSyncs failed: %v", err)
    }
    if len(fetched) != 1 {
        t.Fatalf("expected 1 pending record, got %d", len(fetched))
    }
    if fetched[0].ID != "mem3" {
        t.Errorf("expected mem3, got %s", fetched[0].ID)
    }

    // Verify it was marked in_progress
    row := provider.QueryRow(ctx, "SELECT sync_status FROM swarm_memory_embeddings WHERE memory_id = 'mem3'")
    var status string
    if err := row.Scan(&status); err != nil {
        t.Fatalf("failed to scan status: %v", err)
    }
    if status != string(SyncStatusInProgress) {
        t.Errorf("expected status in_progress, got %s", status)
    }

    // 3. MarkSynced
    if err := svc.MarkSynced(ctx, []string{"mem3"}); err != nil {
        t.Fatalf("MarkSynced failed: %v", err)
    }

    row = provider.QueryRow(ctx, "SELECT sync_status FROM swarm_memory_embeddings WHERE memory_id = 'mem3'")
    if err := row.Scan(&status); err != nil {
        t.Fatalf("failed to scan status: %v", err)
    }
    if status != string(SyncStatusSynced) {
        t.Errorf("expected status synced, got %s", status)
    }
}
