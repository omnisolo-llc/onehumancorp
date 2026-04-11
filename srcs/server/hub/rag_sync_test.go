package hub

import (
	"context"
	"database/sql"
	"os"
	"testing"
	"time"

	_ "modernc.org/sqlite"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"go.opentelemetry.io/otel/metric/noop"
)

func TestRAGSyncService(t *testing.T) {
	// Initialize dummy telemetry to avoid nil pointer panics
	telemetry.RagRecordsSyncedCounter, _ = noop.NewMeterProvider().Meter("test").Int64Counter("rag_records_synced_total")
	telemetry.RagSyncErrorsCounter, _ = noop.NewMeterProvider().Meter("test").Int64Counter("rag_sync_errors_total")

	tmpFile, err := os.CreateTemp("", "test_db_*.sqlite")
	if err != nil {
		t.Fatalf("failed to create temp file: %v", err)
	}
	defer os.Remove(tmpFile.Name())

	sqlDB, err := sql.Open("sqlite", tmpFile.Name())
	if err != nil {
		t.Fatalf("failed to open sqlite db: %v", err)
	}
	defer sqlDB.Close()

	provider := db.NewSqliteProvider(sqlDB)

	ctx := context.Background()
	_, err = provider.Exec(ctx, `
		CREATE TABLE swarm_memory_embeddings (
			memory_id TEXT PRIMARY KEY,
			context TEXT NOT NULL,
			vector_embedding BLOB,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at DATETIME NULL
		)
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	service := NewRAGSyncService(provider)

	// Test ProcessIncomingSync
	now := time.Now().UTC()
	rec := RAGSyncRecord{
		ID:         "mem1",
		Context:    "test context",
		Vector:     []byte{1, 2, 3},
		SyncStatus: SyncStatusSynced,
		LastSyncAt: now,
	}
	if err := service.ProcessIncomingSync(ctx, []RAGSyncRecord{rec}); err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// Test FetchPendingSyncs
	_, err = provider.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status) VALUES (?, ?, ?, ?)", "mem2", "pending ctx", []byte{4, 5}, "pending")
	if err != nil {
		t.Fatalf("failed to insert pending record: %v", err)
	}

	pending, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 1 {
		t.Fatalf("expected 1 pending record, got %d", len(pending))
	}
	if pending[0].ID != "mem2" {
		t.Errorf("expected mem2, got %s", pending[0].ID)
	}

	// Test MarkSynced
	if err := service.MarkSynced(ctx, []string{"mem2"}); err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	pendingAfter, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pendingAfter) != 0 {
		t.Fatalf("expected 0 pending records after MarkSynced, got %d", len(pendingAfter))
	}
}
