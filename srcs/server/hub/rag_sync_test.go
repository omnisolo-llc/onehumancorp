package hub

import (
    "context"
    "testing"
    "os"

    "github.com/onehumancorp/mono/srcs/server/db"
    "go.opentelemetry.io/otel/metric/noop"
    "github.com/onehumancorp/mono/srcs/server/telemetry"
)

func TestHubRAGSyncService(t *testing.T) {
    ctx := context.Background()

    meter := noop.NewMeterProvider().Meter("test")
    telemetry.RagRecordsSyncedTotal, _ = meter.Int64Counter("rag_records_synced_total")
    telemetry.RagSyncErrorsTotal, _ = meter.Int64Counter("rag_sync_errors_total")

    os.Setenv("DATABASE_URL", "sqlite://file::memory:?cache=shared")
    defer os.Unsetenv("DATABASE_URL")
    providerDB, err := db.New(ctx)
    if err != nil {
        t.Fatalf("Failed to create provider: %v", err)
    }
    err = providerDB.RunMigrations(ctx)
    if err != nil {
        t.Fatalf("Failed to apply migrations: %v", err)
    }

    provider := providerDB.Provider

    _, err = provider.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status) VALUES ('mem1', 'ctx1', X'010203', 'pending')")
    if err != nil {
        t.Fatalf("Insert failed: %v", err)
    }

    svc := NewHubRAGSyncService(provider)

    records, err := svc.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("FetchPendingSyncs failed: %v", err)
    }
    if len(records) != 1 {
        t.Fatalf("Expected 1 record, got %d", len(records))
    }

    err = svc.MarkSynced(ctx, []string{"mem1"})
    if err != nil {
        t.Fatalf("MarkSynced failed: %v", err)
    }

    records, err = svc.FetchPendingSyncs(ctx, 10)
    if len(records) != 0 {
        t.Fatalf("Expected 0 pending records after MarkSynced, got %d", len(records))
    }

    newRecords := []RAGSyncRecord{
        {ID: "mem2", Context: "ctx2", Vector: []byte{4,5,6}, SyncStatus: SyncStatusPending},
        {ID: "mem1", Context: "ctx1_updated", Vector: []byte{1,2,3}, SyncStatus: SyncStatusPending},
    }
    err = svc.ProcessIncomingSync(ctx, newRecords)
    if err != nil {
        t.Fatalf("ProcessIncomingSync failed: %v", err)
    }
}
