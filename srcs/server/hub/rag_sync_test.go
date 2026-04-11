package hub

import (
    "context"
    "testing"
    "time"
    "database/sql"

    _db "github.com/onehumancorp/mono/srcs/server/db"
    "github.com/onehumancorp/mono/srcs/server/telemetry"
    "go.opentelemetry.io/otel/metric/noop"
    _ "modernc.org/sqlite"
)

func setupDB(t *testing.T) _db.Provider {
    sqliteDB, err := sql.Open("sqlite", ":memory:")
    if err != nil {
        t.Fatalf("Failed to open sqlite: %v", err)
    }
    provider := _db.NewSqliteProvider(sqliteDB)

    ctx := context.Background()

    tx, err := provider.Begin(ctx)
    if err != nil {
        t.Fatalf("Failed to begin tx: %v", err)
    }
    defer tx.Rollback(ctx)

    _, err = tx.Exec(ctx, `
        CREATE TABLE IF NOT EXISTS swarm_memory_embeddings (
            memory_id        TEXT PRIMARY KEY,
            context          TEXT NOT NULL,
            vector_embedding BLOB,
            source_plugin    TEXT,
            created_at       TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            sync_status      VARCHAR(50) DEFAULT 'pending',
            last_sync_at     TIMESTAMP NULL
        );
    `)
    if err != nil {
        t.Fatalf("Failed to create table: %v", err)
    }
    if err := tx.Commit(ctx); err != nil {
        t.Fatalf("Failed to commit table creation: %v", err)
    }

    return provider
}

func TestRAGSyncService(t *testing.T) {
    meter := noop.NewMeterProvider().Meter("test")
    telemetry.RagRecordsSyncedTotal, _ = meter.Int64Counter("rag_records_synced_total")
    telemetry.RagSyncErrorsTotal, _ = meter.Int64Counter("rag_sync_errors_total")

    provider := setupDB(t)
    service := NewSQLRAGSyncService(provider)
    ctx := context.Background()

    // Insert dummy data
    tx, err := provider.Begin(ctx)
    if err != nil {
        t.Fatalf("Failed to begin tx: %v", err)
    }
    _, err = tx.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding) VALUES ('1', 'ctx1', 'vector1')")
    if err != nil {
        t.Fatalf("Failed to insert dummy data: %v", err)
    }
    if err := tx.Commit(ctx); err != nil {
        t.Fatalf("Failed to commit dummy data: %v", err)
    }

    records, err := service.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("Failed to fetch pending syncs: %v", err)
    }
    if len(records) != 1 {
        t.Errorf("Expected 1 pending record, got %d", len(records))
    }
    if len(records) > 0 && records[0].ID != "1" {
        t.Errorf("Expected record ID '1', got '%s'", records[0].ID)
    }

    err = service.MarkSynced(ctx, []string{"1"})
    if err != nil {
        t.Fatalf("Failed to mark synced: %v", err)
    }

    records, err = service.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("Failed to fetch pending syncs: %v", err)
    }
    if len(records) != 0 {
        t.Errorf("Expected 0 pending records after MarkSynced, got %d", len(records))
    }

    newRecords := []RAGSyncRecord{
        {
            ID: "2",
            Context: "ctx2",
            Vector: []byte("vector2"),
            LastSyncAt: time.Now(),
        },
    }
    err = service.ProcessIncomingSync(ctx, newRecords)
    if err != nil {
        t.Fatalf("Failed to process incoming sync: %v", err)
    }

    tx2, err := provider.Begin(ctx)
    if err != nil {
        t.Fatalf("Failed to begin verify tx: %v", err)
    }
    var count int
    err = tx2.QueryRow(ctx, "SELECT COUNT(*) FROM swarm_memory_embeddings WHERE memory_id = '2' AND sync_status = 'synced'").Scan(&count)
    if err != nil {
        t.Fatalf("Failed to verify data: %v", err)
    }
    if count != 1 {
        t.Errorf("Expected 1 synced record for ID '2', got %d", count)
    }
    tx2.Rollback(ctx)
}
