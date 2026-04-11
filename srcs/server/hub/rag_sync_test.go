package hub

import (
	"context"
	"database/sql"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	"go.opentelemetry.io/otel/metric/noop"
	_ "modernc.org/sqlite"
)

func TestRAGSyncService(t *testing.T) {
	ctx := context.Background()

	sqlDB, err := sql.Open("sqlite", "file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}

	provider := db.NewSqliteProvider(sqlDB)
	defer provider.Close()

	// Initialize tables
	_, err = provider.Exec(ctx, `
		CREATE TABLE swarm_memory_embeddings (
			memory_id        TEXT PRIMARY KEY,
			context          TEXT NOT NULL,
			vector_embedding BLOB,
			source_plugin    TEXT,
			created_at       DATETIME DEFAULT CURRENT_TIMESTAMP,
			sync_status      VARCHAR(50) DEFAULT 'pending',
			last_sync_at     DATETIME NULL
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	meter := noop.NewMeterProvider().Meter("test")
	service, err := NewRAGSyncService(provider, meter)
	if err != nil {
		t.Fatalf("failed to create sync service: %v", err)
	}

	// Insert pending
	_, err = provider.Exec(ctx, `INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status) VALUES ('test-id-1', 'test context 1', ?, 'pending')`, []byte{1,2,3})
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	// Test FetchPendingSyncs
	pending, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 1 {
		t.Fatalf("expected 1 pending record, got %d", len(pending))
	}
	if pending[0].ID != "test-id-1" {
		t.Errorf("expected ID 'test-id-1', got '%s'", pending[0].ID)
	}

	// Test MarkSynced
	err = service.MarkSynced(ctx, []string{"test-id-1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	pending2, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending2) != 0 {
		t.Fatalf("expected 0 pending records after MarkSynced, got %d", len(pending2))
	}

	// Test ProcessIncomingSync
	incoming := []RAGSyncRecord{
		{
			ID:         "test-id-2",
			Context:    "test context 2",
			Vector:     []byte{4,5,6},
			SyncStatus: SyncStatusSynced,
		},
		{
			ID:         "test-id-1", // update existing
			Context:    "updated context 1",
			Vector:     []byte{7,8,9},
			SyncStatus: SyncStatusSynced,
		},
	}

	err = service.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// Verify updates
	row := provider.QueryRow(ctx, "SELECT context FROM swarm_memory_embeddings WHERE memory_id = 'test-id-1'")
	var ctx1 string
	if err := row.Scan(&ctx1); err != nil {
		t.Fatalf("failed to query updated record: %v", err)
	}
	if ctx1 != "updated context 1" {
		t.Errorf("expected 'updated context 1', got '%s'", ctx1)
	}

	row = provider.QueryRow(ctx, "SELECT context FROM swarm_memory_embeddings WHERE memory_id = 'test-id-2'")
	var ctx2 string
	if err := row.Scan(&ctx2); err != nil {
		t.Fatalf("failed to query new record: %v", err)
	}
	if ctx2 != "test context 2" {
		t.Errorf("expected 'test context 2', got '%s'", ctx2)
	}
}
