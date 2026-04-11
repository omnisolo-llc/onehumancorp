package hub

import (
    "context"
    "database/sql"
    "testing"

    "github.com/onehumancorp/mono/srcs/server/db"
    "github.com/onehumancorp/mono/srcs/server/telemetry"
    "go.opentelemetry.io/otel/metric/noop"
    _ "modernc.org/sqlite"
)

func init() {
    // Mock telemetry
    telemetry.RagRecordsSyncedTotal, _ = noop.NewMeterProvider().Meter("test").Int64Counter("rag_records_synced_total")
    telemetry.RagSyncErrorsTotal, _ = noop.NewMeterProvider().Meter("test").Int64Counter("rag_sync_errors_total")
}

func TestRAGSyncService(t *testing.T) {
    ctx := context.Background()

    // Setup dummy db
    dbConn, err := sql.Open("sqlite", "file::memory:?cache=shared")
    if err != nil {
        t.Fatalf("failed to open sqlite: %v", err)
    }
    defer dbConn.Close()

    provider := db.NewSqliteProvider(dbConn)
    defer provider.Close()

    // Run schema setup for testing
    setupQuery := `
    CREATE TABLE IF NOT EXISTS swarm_memory_embeddings (
        memory_id        TEXT PRIMARY KEY,
        context          TEXT NOT NULL,
        vector_embedding BLOB,
        source_plugin    TEXT,
        created_at       TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
        sync_status      VARCHAR(50) DEFAULT 'pending',
        last_sync_at     TIMESTAMPTZ NULL
    );
    `
    _, err = provider.Exec(ctx, setupQuery)
    if err != nil {
        t.Fatalf("failed to setup schema: %v", err)
    }

    service := NewRAGSyncService(provider)

    // Create some pending records
    _, err = provider.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding) VALUES ('1', 'ctx1', 'vec1'), ('2', 'ctx2', 'vec2')")
    if err != nil {
        t.Fatalf("insert failed: %v", err)
    }

    // Test FetchPendingSyncs
    records, err := service.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("fetch failed: %v", err)
    }
    if len(records) != 2 {
        t.Errorf("expected 2 records, got %d", len(records))
    }

    // Test MarkSynced
    err = service.MarkSynced(ctx, []string{"1"})
    if err != nil {
        t.Fatalf("marksynced failed: %v", err)
    }

    // Verify only 1 pending remains
    records, err = service.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("fetch failed: %v", err)
    }
    if len(records) != 1 || records[0].ID != "2" {
        t.Errorf("expected record 2, got %v", records)
    }

    // Test ProcessIncomingSync
    incoming := []RAGSyncRecord{
        {ID: "3", Context: "ctx3", Vector: []byte("vec3")},
        {ID: "1", Context: "ctx1_updated", Vector: []byte("vec1_updated")},
    }
    err = service.ProcessIncomingSync(ctx, incoming)
    if err != nil {
        t.Fatalf("processincomingsync failed: %v", err)
    }

    var ctx1 string
    row := provider.QueryRow(ctx, "SELECT context FROM swarm_memory_embeddings WHERE memory_id = '1'")
    err = row.Scan(&ctx1)
    if err != nil || ctx1 != "ctx1_updated" {
        t.Errorf("expected ctx1_updated, got %v (err: %v)", ctx1, err)
    }
}
