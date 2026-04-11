package hub

import (
    "context"
    "database/sql"
    "testing"

    _ "modernc.org/sqlite"
    "github.com/onehumancorp/mono/srcs/server/db"
    "github.com/onehumancorp/mono/srcs/server/telemetry"
    "go.opentelemetry.io/otel/metric/noop"
)

func setupTestProvider(t *testing.T) db.Provider {
    dbConn, err := sql.Open("sqlite", "file::memory:?cache=shared")
    if err != nil {
        t.Fatalf("Failed to open sqlite: %v", err)
    }
    provider := db.NewSqliteProvider(dbConn)

    // Use TEXT/BLOB for pgvector compat in sqlite
    createTable := `
    CREATE TABLE swarm_memory_embeddings (
        memory_id TEXT PRIMARY KEY,
        context TEXT NOT NULL,
        vector_embedding BLOB,
        sync_status TEXT DEFAULT 'pending',
        last_sync_at TIMESTAMPTZ NULL
    );`
    if _, err := provider.Exec(context.Background(), createTable); err != nil {
        t.Fatalf("Failed to create table: %v", err)
    }

    return provider
}

func TestRAGSyncService(t *testing.T) {
    provider := setupTestProvider(t)
    defer provider.Close()

    // Init dummy metrics to avoid nil panic
    meter := noop.NewMeterProvider().Meter("test")
    telemetry.RagRecordsSyncedTotal, _ = meter.Int64Counter("rag_records_synced_total")
    telemetry.RagSyncErrorsTotal, _ = meter.Int64Counter("rag_sync_errors_total")

    svc := NewRAGSyncService(provider)
    ctx := context.Background()

    // Insert mock pending data
    _, err := provider.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding) VALUES ($1, $2, $3)", "id1", "ctx1", []byte{1,2,3})
    if err != nil {
        t.Fatalf("Insert failed: %v", err)
    }

    // Test FetchPendingSyncs
    records, err := svc.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("FetchPendingSyncs failed: %v", err)
    }
    if len(records) != 1 || records[0].ID != "id1" {
        t.Fatalf("Expected 1 record, got %v", records)
    }

    // Test MarkSynced
    err = svc.MarkSynced(ctx, []string{"id1"})
    if err != nil {
        t.Fatalf("MarkSynced failed: %v", err)
    }

    // Test ProcessIncomingSync
    incoming := []RAGSyncRecord{
        {ID: "id2", Context: "ctx2", Vector: []byte{4,5,6}},
    }
    err = svc.ProcessIncomingSync(ctx, incoming)
    if err != nil {
        t.Fatalf("ProcessIncomingSync failed: %v", err)
    }
}
