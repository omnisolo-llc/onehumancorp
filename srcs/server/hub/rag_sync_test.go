package hub

import (
	"context"
	"database/sql"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func TestRAGSyncService(t *testing.T) {
	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}
	defer sqlDB.Close()

	provider := db.NewSqliteProvider(sqlDB)
	ctx := context.Background()

	// Setup schema
	_, err = provider.Exec(ctx, `
		CREATE TABLE swarm_memory_embeddings (
			memory_id        TEXT PRIMARY KEY,
			context          TEXT NOT NULL,
			vector_embedding BYTEA,
			sync_status      VARCHAR(50) DEFAULT 'pending',
			last_sync_at     TIMESTAMPTZ NULL
		)
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	svc := NewRAGSyncService(provider)

	// Test ProcessIncomingSync
	records := []RAGSyncRecord{
		{ID: "m1", Context: "test context 1", Vector: []float32{0.1, 0.2}},
	}
	if err := svc.ProcessIncomingSync(ctx, records); err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// Insert a pending record manually
	_, err = provider.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES (?, ?, ?)", "m2", "test context 2", "pending")
	if err != nil {
		t.Fatalf("failed to insert pending record: %v", err)
	}

	// Test FetchPendingSyncs
	pending, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 1 {
		t.Fatalf("expected 1 pending record, got %d", len(pending))
	}
	if pending[0].ID != "m2" {
		t.Fatalf("expected pending record ID m2, got %s", pending[0].ID)
	}

	// Test MarkSynced
	if err := svc.MarkSynced(ctx, []string{"m2"}); err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	pendingAfter, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed after mark synced: %v", err)
	}
	if len(pendingAfter) != 0 {
		t.Fatalf("expected 0 pending records, got %d", len(pendingAfter))
	}
}
