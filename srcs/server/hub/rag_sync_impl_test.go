package hub_test

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/hub"
	"database/sql"
	_ "modernc.org/sqlite"
)

func setupTestDB(t *testing.T) *sql.DB {
	db, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open db: %v", err)
	}

	// Ensure the base table and new columns exist
	_, err = db.ExecContext(context.Background(), `
		CREATE TABLE IF NOT EXISTS swarm_memory_embeddings (
			memory_id TEXT PRIMARY KEY,
			context TEXT NOT NULL,
			vector_embedding BLOB,
			source_plugin TEXT,
			created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL
		)
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}
	return db
}

func TestRAGSyncServiceImpl(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()

	service := hub.NewRAGSyncService(db)
	ctx := context.Background()

	// Insert some test data
	_, err := db.ExecContext(ctx, `
		INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status)
		VALUES ('m1', 'ctx1', 'pending'), ('m2', 'ctx2', 'pending')
	`)
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}

	// Test FetchPendingSyncs
	pending, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("failed to fetch pending syncs: %v", err)
	}
	if len(pending) != 2 {
		t.Fatalf("expected 2 pending records, got %d", len(pending))
	}

	// Test MarkSynced
	err = service.MarkSynced(ctx, []string{"m1"})
	if err != nil {
		t.Fatalf("failed to mark synced: %v", err)
	}

	pending, err = service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("failed to fetch pending syncs: %v", err)
	}
	if len(pending) != 1 {
		t.Fatalf("expected 1 pending record, got %d", len(pending))
	}

	// Test ProcessIncomingSync
	newRecords := []hub.RAGSyncRecord{
		{ID: "m3", Context: "ctx3", SyncStatus: hub.SyncStatusSynced, LastSyncAt: time.Now()},
		{ID: "m1", Context: "ctx1_updated", SyncStatus: hub.SyncStatusSynced, LastSyncAt: time.Now()}, // This will test upsert
	}
	err = service.ProcessIncomingSync(ctx, newRecords)
	if err != nil {
		t.Fatalf("failed to process incoming sync: %v", err)
	}

	var count int
	err = db.QueryRowContext(ctx, "SELECT COUNT(*) FROM swarm_memory_embeddings").Scan(&count)
	if err != nil {
		t.Fatalf("failed to get count: %v", err)
	}
	if count != 3 {
		t.Fatalf("expected 3 total records, got %d", count)
	}
}
