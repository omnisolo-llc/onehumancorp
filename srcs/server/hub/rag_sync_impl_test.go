package hub

import (
	"context"
	"database/sql"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func setupTestDB(t *testing.T) db.Provider {
	dbConn, err := sql.Open("sqlite", "file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}

	provider := db.NewSqliteProvider(dbConn)

	_, err = provider.Exec(context.Background(), `
		CREATE TABLE swarm_memory_embeddings (
			memory_id TEXT PRIMARY KEY,
			context TEXT NOT NULL,
			vector_embedding BLOB,
			source_plugin TEXT,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at DATETIME NULL
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	return provider
}

func TestRAGSyncServiceImpl(t *testing.T) {
	provider := setupTestDB(t)
	service := NewRAGSyncService(provider)

	ctx := context.Background()

	// Insert initial data via ProcessIncomingSync (simulating an incoming sync to initialize data)
	// We use "pending" initially to see if Fetch Pending picks it up
	records := []RAGSyncRecord{
		{ID: "1", Context: "test 1", Vector: []byte{1, 2}, SyncStatus: SyncStatusPending},
		{ID: "2", Context: "test 2", Vector: []byte{3, 4}, SyncStatus: SyncStatusPending},
	}

	err := service.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// Fetch pending
	pending, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 2 {
		t.Fatalf("Expected 2 pending records, got %d", len(pending))
	}

	// Mark Synced
	err = service.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	// Fetch pending again
	pending, err = service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 1 {
		t.Fatalf("Expected 1 pending record, got %d", len(pending))
	}
	if pending[0].ID != "2" {
		t.Fatalf("Expected pending record ID 2, got %s", pending[0].ID)
	}

	// Update existing record
	records = []RAGSyncRecord{
		{ID: "2", Context: "updated test 2", Vector: []byte{3, 4}, SyncStatus: SyncStatusSynced},
	}
	err = service.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed on update: %v", err)
	}

	pending, err = service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 0 {
		t.Fatalf("Expected 0 pending records, got %d", len(pending))
	}
}
