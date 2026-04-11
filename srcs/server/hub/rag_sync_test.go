package hub

import (
	"context"
	"database/sql"
	"os"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func TestHybridRAGSyncService(t *testing.T) {
	// Create temp file for SQLite DB
	tmpFile, err := os.CreateTemp("", "rag_sync_test_*.db")
	if err != nil {
		t.Fatalf("failed to create temp db file: %v", err)
	}
	defer os.Remove(tmpFile.Name())

	sqlDB, err := sql.Open("sqlite", tmpFile.Name())
	if err != nil {
		t.Fatalf("failed to open sqlite db: %v", err)
	}
	defer sqlDB.Close()

	database := db.NewSqliteProvider(sqlDB)

	ctx := context.Background()

	// Initialize schema manually
	schema := `
		CREATE TABLE IF NOT EXISTS swarm_memory_embeddings (
			memory_id TEXT PRIMARY KEY,
			context TEXT NOT NULL,
			vector_embedding BYTEA,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMPTZ NULL
		);
	`
	if _, err := database.Exec(ctx, schema); err != nil {
		t.Fatalf("failed to create schema: %v", err)
	}

	service := NewHybridRAGSyncService(database)

	// 1. Process incoming sync
	incoming := []RAGSyncRecord{
		{ID: "mem-1", Context: "context 1", Vector: []byte{1, 2, 3}},
	}
	err = service.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// 2. Insert a pending record manually
	_, err = database.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status) VALUES ($1, $2, $3, $4)", "mem-2", "context 2", []byte{4, 5, 6}, SyncStatusPending)
	if err != nil {
		t.Fatalf("failed to insert pending record: %v", err)
	}

	// 3. Fetch pending syncs
	pending, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 1 {
		t.Errorf("expected 1 pending record, got %d", len(pending))
	}
	if pending[0].ID != "mem-2" {
		t.Errorf("expected pending record ID mem-2, got %s", pending[0].ID)
	}

	// 4. Mark synced
	err = service.MarkSynced(ctx, []string{"mem-2"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	// Verify it's synced
	pendingAgain, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pendingAgain) != 0 {
		t.Errorf("expected 0 pending records after MarkSynced, got %d", len(pendingAgain))
	}
}
