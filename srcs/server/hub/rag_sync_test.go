package hub

import (
    "context"
    "testing"
    "database/sql"

    _ "modernc.org/sqlite"
    "go.opentelemetry.io/otel"
    "github.com/onehumancorp/mono/srcs/server/db"
)

func TestRAGSyncService(t *testing.T) {
    sqliteDB, err := sql.Open("sqlite", ":memory:")
    if err != nil {
        t.Fatalf("failed to open sqlite: %v", err)
    }
    defer sqliteDB.Close()

    provider := &db.DB{Provider: db.NewSqliteProvider(sqliteDB)}
    defer provider.Close()

    _, err = provider.Exec(context.Background(), "CREATE TABLE autodream_memories (id TEXT PRIMARY KEY, content TEXT, embedding TEXT, sync_status TEXT, last_sync_at TIMESTAMP)")
    if err != nil {
        t.Fatalf("failed to create table: %v", err)
    }

    meter := otel.Meter("test")
    svc, err := NewDefaultRAGSyncService(provider, meter)
    if err != nil {
        t.Fatalf("expected no error, got %v", err)
    }

    ctx := context.Background()
    records := []RAGSyncRecord{{ID: "1", Context: "test context", Vector: []float32{1.0, 2.0}, SyncStatus: SyncStatusPending}}

    err = svc.ProcessIncomingSync(ctx, records)
    if err != nil {
        t.Fatalf("expected no error, got %v", err)
    }

    err = svc.MarkSynced(ctx, []string{"1"})
    if err != nil {
        t.Fatalf("expected no error, got %v", err)
    }

    pending, err := svc.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("expected no error, got %v", err)
    }
    if len(pending) != 0 {
        t.Errorf("expected 0 pending, got %d", len(pending))
    }
}
