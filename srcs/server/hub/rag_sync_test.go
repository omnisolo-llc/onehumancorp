package hub

import (
	"context"
	"database/sql"
	_ "modernc.org/sqlite"
	"testing"
	"time"
)

func setupTestDB(t *testing.T) *sql.DB {
	db, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("Failed to open sqlite db: %v", err)
	}

	schema := `
    CREATE TABLE swarm_memory_embeddings (
        memory_id TEXT PRIMARY KEY,
        context TEXT,
        sync_status VARCHAR(50) DEFAULT 'pending',
        last_sync_at TIMESTAMP NULL
    );`

	if _, err := db.Exec(schema); err != nil {
		t.Fatalf("Failed to create schema: %v", err)
	}
	return db
}

func TestRAGSyncProvider(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()

	provider := NewRAGSyncProvider(db)
	ctx := context.Background()

	// 1. Insert initial pending data
	_, err := db.Exec(`INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES ('1', 'context 1', 'pending'), ('2', 'context 2', 'pending')`)
	if err != nil {
		t.Fatalf("Failed to insert initial data: %v", err)
	}

	// 2. Fetch Pending Syncs
	pending, err := provider.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 2 {
		t.Fatalf("Expected 2 pending records, got %d", len(pending))
	}

	// 3. Mark Synced
	err = provider.MarkSynced(ctx, []string{"1", "2"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	// Verify marked
	var count int
	db.QueryRow(`SELECT count(*) FROM swarm_memory_embeddings WHERE sync_status = 'pending'`).Scan(&count)
	if count != 0 {
		t.Fatalf("Expected 0 pending records after MarkSynced, got %d", count)
	}

	// 4. Process Incoming Sync
	incoming := []RAGSyncRecord{
		{ID: "3", Context: "incoming context 3", SyncStatus: SyncStatusSynced, LastSyncAt: time.Now()},
		{ID: "1", Context: "updated context 1", SyncStatus: SyncStatusSynced, LastSyncAt: time.Now()},
	}

	err = provider.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// Verify incoming
	db.QueryRow(`SELECT count(*) FROM swarm_memory_embeddings WHERE memory_id = '3'`).Scan(&count)
	if count != 1 {
		t.Fatalf("Expected record '3' to be inserted")
	}

	var updatedContext string
	db.QueryRow(`SELECT context FROM swarm_memory_embeddings WHERE memory_id = '1'`).Scan(&updatedContext)
	if updatedContext != "updated context 1" {
		t.Fatalf("Expected context to be updated, got: %s", updatedContext)
	}
}
