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

func TestRagSyncService(t *testing.T) {
	dbConn, err := sql.Open("sqlite", "file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}
	defer dbConn.Close()

	_, err = dbConn.Exec(`
		CREATE TABLE IF NOT EXISTS swarm_memory_embeddings (
			memory_id        TEXT PRIMARY KEY,
			context          TEXT NOT NULL,
			vector_embedding BLOB,
			source_plugin    TEXT,
			created_at       DATETIME DEFAULT CURRENT_TIMESTAMP,
			sync_status      VARCHAR(50) DEFAULT 'pending',
			last_sync_at     DATETIME
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	provider := db.NewSqliteProvider(dbConn)
	service := NewRAGSyncService(provider)

	meter := noop.NewMeterProvider().Meter("test")
	telemetry.RagRecordsSyncedTotal, _ = meter.Int64Counter("rag_records_synced_total")
	telemetry.RagSyncErrorsTotal, _ = meter.Int64Counter("rag_sync_errors_total")

	ctx := context.Background()

	// Test ProcessIncomingSync
	records := []RAGSyncRecord{
		{
			ID:         "mem1",
			Context:    "test context 1",
			Vector:     []byte{1, 2, 3},
			SyncStatus: SyncStatusPending,
		},
		{
			ID:         "mem2",
			Context:    "test context 2",
			Vector:     []byte{4, 5, 6},
			SyncStatus: SyncStatusPending,
		},
	}
	if err := service.ProcessIncomingSync(ctx, records); err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// Test FetchPendingSyncs
	pending, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 2 {
		t.Fatalf("expected 2 pending records, got %d", len(pending))
	}

	// Test MarkSynced
	if err := service.MarkSynced(ctx, []string{"mem1"}); err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	pending, err = service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 1 || pending[0].ID != "mem2" {
		t.Fatalf("expected 1 pending record (mem2), got %v", pending)
	}
}
