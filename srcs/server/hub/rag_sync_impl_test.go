package hub

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestRAGSyncServiceImpl_SQLiteCoverage(t *testing.T) {
	t.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory")
	ctx := context.Background()

	database, err := db.New(ctx)
	if err != nil {
		t.Fatalf("failed to init db: %v", err)
	}
	defer database.Close()

	// Ensure the table exists in our in-memory SQLite DB
	initQueries := []string{
		`CREATE TABLE IF NOT EXISTS swarm_memory_embeddings (
			memory_id TEXT PRIMARY KEY,
			context TEXT NOT NULL,
			vector_embedding BLOB,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL
		)`,
		`INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status)
		 VALUES ('mem1', 'test context', 'test_vector', 'pending')`,
	}

	tx, err := database.Begin(ctx)
	if err != nil {
		t.Fatalf("failed to begin tx: %v", err)
	}
	for _, q := range initQueries {
		if _, err := tx.Exec(ctx, q); err != nil {
			t.Fatalf("failed to exec init query: %v", err)
		}
	}
	tx.Commit(ctx)

	service := NewRAGSyncService(database)

	// Test FetchPendingSyncs
	pending, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(pending) != 1 {
		t.Errorf("expected 1 pending record, got %d", len(pending))
	}
	if pending[0].ID != "mem1" {
		t.Errorf("expected ID mem1, got %s", pending[0].ID)
	}

	// Test MarkSynced
	err = service.MarkSynced(ctx, []string{"mem1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	// Verify it was marked synced
	pendingAgain, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(pendingAgain) != 0 {
		t.Errorf("expected 0 pending records after MarkSynced, got %d", len(pendingAgain))
	}

	// Test ProcessIncomingSync
	incoming := []RAGSyncRecord{
		{
			ID: "mem2",
			Context: "incoming context",
			Vector: []byte("incoming_vector"),
			SyncStatus: SyncStatusSynced,
			LastSyncAt: time.Now(),
		},
	}
	err = service.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
}
