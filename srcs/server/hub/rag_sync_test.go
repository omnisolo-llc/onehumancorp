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

func TestRAGSyncService(t *testing.T) {
	// Initialize dummy metrics to avoid nil pointer panics
	meter := noop.NewMeterProvider().Meter("test")
	telemetry.RagRecordsSyncedTotal, _ = meter.Int64Counter("rag_records_synced_total")
	telemetry.RagSyncErrorsTotal, _ = meter.Int64Counter("rag_sync_errors_total")

	// Initialize an in-memory SQLite database provider for testing
	sqliteDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to create sqlite provider: %v", err)
	}
	provider := db.NewSqliteProvider(sqliteDB)
	ctx := context.Background()

	// Create table schema
	_, err = provider.Exec(ctx, `
		CREATE TABLE swarm_memory_embeddings (
			memory_id TEXT PRIMARY KEY,
			context TEXT NOT NULL,
			vector_embedding BLOB,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	service := NewRAGSyncService(provider)

	// Test 1: Empty state
	records, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(records) != 0 {
		t.Fatalf("Expected 0 records, got %d", len(records))
	}

	// Test 2: Process Incoming Sync
	incomingRecords := []RAGSyncRecord{
		{ID: "mem1", Context: "context 1", Vector: []byte{1, 2, 3}},
		{ID: "mem2", Context: "context 2", Vector: []byte{4, 5, 6}},
	}
	if err := service.ProcessIncomingSync(ctx, incomingRecords); err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// Verify they are synced
	rows, err := provider.Query(ctx, `SELECT memory_id, sync_status FROM swarm_memory_embeddings`)
	if err != nil {
		t.Fatalf("Query failed: %v", err)
	}
	defer rows.Close()

	count := 0
	for rows.Next() {
		var id, status string
		if err := rows.Scan(&id, &status); err != nil {
			t.Fatalf("Scan failed: %v", err)
		}
		if status != string(SyncStatusSynced) {
			t.Errorf("Expected status synced, got %s for id %s", status, id)
		}
		count++
	}
	if count != 2 {
		t.Fatalf("Expected 2 records, got %d", count)
	}

	// Test 3: Fetch pending syncs (insert some pending)
	_, err = provider.Exec(ctx, `
		INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status)
		VALUES ('mem3', 'context 3', 'pending')
	`)
	if err != nil {
		t.Fatalf("Insert failed: %v", err)
	}

	pending, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 1 {
		t.Fatalf("Expected 1 pending record, got %d", len(pending))
	}
	if pending[0].ID != "mem3" {
		t.Errorf("Expected id mem3, got %s", pending[0].ID)
	}

	// Test 4: Mark Synced
	if err := service.MarkSynced(ctx, []string{"mem3"}); err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	pendingAfter, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pendingAfter) != 0 {
		t.Fatalf("Expected 0 pending records after MarkSynced, got %d", len(pendingAfter))
	}
}
