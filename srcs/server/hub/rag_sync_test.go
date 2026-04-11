package hub

import (
	"context"
	"database/sql"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"go.opentelemetry.io/otel/metric/noop"
	_ "modernc.org/sqlite"
)

func TestRAGSyncService(t *testing.T) {
	// Initialize telemetry to avoid nil pointer panic
	meter := noop.NewMeterProvider().Meter("test")
	telemetry.RagRecordsSyncedTotal, _ = meter.Int64Counter("rag_records_synced_total")
	telemetry.RagSyncErrorsTotal, _ = meter.Int64Counter("rag_sync_errors_total")

    sqliteDB, err := sql.Open("sqlite", ":memory:")
    if err != nil {
        t.Fatalf("failed to open sqlite: %v", err)
    }
    defer sqliteDB.Close()

	pool := db.NewSqliteProvider(sqliteDB)

	ctx := context.Background()
	_, err = pool.Exec(ctx, `
		CREATE TABLE swarm_memory_embeddings (
			memory_id TEXT PRIMARY KEY,
			context TEXT NOT NULL,
			vector_embedding BLOB,
			source_plugin TEXT,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			sync_status TEXT DEFAULT 'pending',
			last_sync_at DATETIME NULL
		)
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	svc := NewRAGSyncService(pool)

	// Insert initial data
	_, err = pool.Exec(ctx, `INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status) VALUES ('1', 'ctx1', 'vec1', 'pending')`)
	if err != nil {
		t.Fatalf("failed to insert: %v", err)
	}

	// Test FetchPendingSyncs
	records, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs err: %v", err)
	}
	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}
	if records[0].ID != "1" {
		t.Fatalf("expected id 1, got %s", records[0].ID)
	}

	// Test MarkSynced
	err = svc.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("MarkSynced err: %v", err)
	}

	records2, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs err: %v", err)
	}
	if len(records2) != 0 {
		t.Fatalf("expected 0 records after MarkSynced, got %d", len(records2))
	}

	// Test ProcessIncomingSync
	incoming := []RAGSyncRecord{
		{
			ID:         "2",
			Context:    "ctx2",
			Vector:     []byte("vec2"),
			SyncStatus: SyncStatusSynced,
			LastSyncAt: time.Now(),
		},
	}
	err = svc.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("ProcessIncomingSync err: %v", err)
	}

	// Verify insert
	var count int
	err = pool.QueryRow(ctx, `SELECT COUNT(*) FROM swarm_memory_embeddings WHERE memory_id = '2'`).Scan(&count)
	if err != nil || count != 1 {
		t.Fatalf("expected 1 row for id '2', err=%v count=%d", err, count)
	}
}
