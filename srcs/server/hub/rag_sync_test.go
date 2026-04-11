package hub

import (
	"context"
	"database/sql"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
	_ "modernc.org/sqlite"
	"go.opentelemetry.io/otel/metric/noop"
)

func setupTestDB(t *testing.T) db.Provider {
	sqliteDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite db: %v", err)
	}

	provider := db.NewSqliteProvider(sqliteDB)

	// Mock the table
	query := `CREATE TABLE swarm_memory_embeddings (
		memory_id TEXT PRIMARY KEY,
		context TEXT NOT NULL,
		vector_embedding BLOB,
		source_plugin TEXT,
		created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
		sync_status VARCHAR(50) DEFAULT 'pending',
		last_sync_at TIMESTAMP NULL
	)`
	if _, err := provider.Exec(context.Background(), query); err != nil {
		t.Fatalf("failed to create table: %v", err)
	}
	return provider
}

func TestRAGSyncService(t *testing.T) {
	ctx := context.Background()

	// Initialize dummy metrics
	meter := noop.NewMeterProvider().Meter("test")
	telemetry.RAGRecordsSyncedTotal, _ = meter.Int64Counter("rag_records_synced_total")
	telemetry.RAGSyncErrorsTotal, _ = meter.Int64Counter("rag_sync_errors_total")

	provider := setupTestDB(t)
	svc := NewRAGSyncService(provider)

	// 1. Test ProcessIncomingSync (creates new records)
	records := []RAGSyncRecord{
		{ID: "m1", Context: "test 1", Vector: []byte{1, 2, 3}},
	}
	if err := svc.ProcessIncomingSync(ctx, records); err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// Verify it was inserted as 'synced'
	var status string
	provider.QueryRow(ctx, "SELECT sync_status FROM swarm_memory_embeddings WHERE memory_id = 'm1'").Scan(&status)
	if status != "synced" {
		t.Errorf("expected synced, got %s", status)
	}

	// Insert a pending record manually
	provider.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status) VALUES ('m2', 'test 2', 'x', 'pending')")

	// 2. Test FetchPendingSyncs
	pending, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 1 || pending[0].ID != "m2" {
		t.Errorf("expected 1 pending record (m2), got %v", pending)
	}

	// 3. Test MarkSynced
	if err := svc.MarkSynced(ctx, []string{"m2"}); err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	pendingAfter, _ := svc.FetchPendingSyncs(ctx, 10)
	if len(pendingAfter) != 0 {
		t.Errorf("expected 0 pending records after MarkSynced, got %v", pendingAfter)
	}
}
