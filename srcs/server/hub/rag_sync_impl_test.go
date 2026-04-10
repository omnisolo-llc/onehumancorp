package hub

import (
	"context"
	"database/sql"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func setupTestDB(t *testing.T) db.Provider {
	dbConn, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite db: %v", err)
	}

	provider := db.NewSqliteProvider(dbConn)
	ctx := context.Background()

	_, err = provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS swarm_memory_embeddings (
			memory_id        TEXT PRIMARY KEY,
			context          TEXT NOT NULL,
			vector_embedding TEXT,
			source_plugin    TEXT,
			created_at       TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
			sync_status      VARCHAR(50) DEFAULT 'pending',
			last_sync_at     TIMESTAMPTZ NULL
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	return provider
}

func TestDefaultRAGSyncService(t *testing.T) {
	provider := setupTestDB(t)
	svc := NewDefaultRAGSyncService(provider)
	ctx := context.Background()

	// Test ProcessIncomingSync
	records := []RAGSyncRecord{
		{ID: "mem1", Context: "context 1", Vector: []float32{1.0, 2.0}},
		{ID: "mem2", Context: "context 2", Vector: []float32{3.0, 4.0}},
	}

	err := svc.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// Insert a pending record manually
	_, err = provider.Exec(ctx, `INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status) VALUES (?, ?, ?, ?)`, "mem3", "context 3", "[5.0, 6.0]", "pending")
	if err != nil {
		t.Fatalf("Failed to insert pending record: %v", err)
	}

	// Test FetchPendingSyncs
	pending, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}

	if len(pending) != 1 {
		t.Fatalf("Expected 1 pending record, got %d", len(pending))
	}

	if pending[0].ID != "mem3" {
		t.Errorf("Expected pending ID mem3, got %s", pending[0].ID)
	}

	// Test MarkSynced
	err = svc.MarkSynced(ctx, []string{"mem3"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	// Verify it's no longer pending
	pendingAfter, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}

	if len(pendingAfter) != 0 {
		t.Fatalf("Expected 0 pending records after MarkSynced, got %d", len(pendingAfter))
	}
}
