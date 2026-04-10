package hub_test

import (
	"context"
	"database/sql"
	"testing"

	_ "modernc.org/sqlite"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/hub"
)

func TestRAGSyncProvider(t *testing.T) {
	ctx := context.Background()

	rawDB, err := sql.Open("sqlite", "file::memory:?mode=memory")
	if err != nil {
		t.Fatalf("Failed to open sqlite: %v", err)
	}
	defer rawDB.Close()

	database := db.NewSqliteProvider(rawDB)
	defer database.Close()

	// Manually create the table and migration columns
	createTableSQL := `
		CREATE TABLE swarm_memory_embeddings (
			memory_id        TEXT PRIMARY KEY,
			context          TEXT NOT NULL,
			vector_embedding BYTEA,
			source_plugin    TEXT,
			created_at       TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
			sync_status      VARCHAR(50) DEFAULT 'pending',
			last_sync_at     TIMESTAMPTZ NULL
		);
	`
	_, err = database.Exec(ctx, createTableSQL)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	provider := hub.NewRAGSyncProvider(database)

	// Test ProcessIncomingSync
	records := []hub.RAGSyncRecord{
		{
			ID:         "mem1",
			Context:    "Test context 1",
			Vector:     []float32{0.1, 0.2, 0.3},
			SyncStatus: hub.SyncStatusPending,
		},
		{
			ID:         "mem2",
			Context:    "Test context 2",
			Vector:     []float32{0.4, 0.5, 0.6},
			SyncStatus: hub.SyncStatusPending,
		},
	}

	err = provider.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("Failed to process incoming sync: %v", err)
	}

	// Test FetchPendingSyncs
	pending, err := provider.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("Failed to fetch pending syncs: %v", err)
	}
	if len(pending) != 2 {
		t.Fatalf("Expected 2 pending syncs, got %d", len(pending))
	}

	// Test MarkSynced
	err = provider.MarkSynced(ctx, []string{"mem1", "mem2"})
	if err != nil {
		t.Fatalf("Failed to mark synced: %v", err)
	}

	// Verify they are no longer pending
	pendingAfter, err := provider.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("Failed to fetch pending syncs after mark: %v", err)
	}
	if len(pendingAfter) != 0 {
		t.Fatalf("Expected 0 pending syncs after marking, got %d", len(pendingAfter))
	}
}
