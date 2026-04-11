package hub

import (
	"context"
	"database/sql"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func setupTestDB(t *testing.T) db.Provider {
	sqliteDB, err := sql.Open("sqlite", "file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}

	provider := db.NewSqliteProvider(sqliteDB)

	// Create table manually since Goose migrations aren't run automatically in db.Provider test setup
	query := `
		DROP TABLE IF EXISTS autodream_memories;
		CREATE TABLE autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			embedding TEXT,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL
		);
	`
	ctx := context.Background()
	_, err = provider.Exec(ctx, query)
	if err != nil {
		t.Fatalf("failed to create autodream_memories table: %v", err)
	}

	return provider
}

func TestRAGSyncService_DB(t *testing.T) {
	provider := setupTestDB(t)
	service := NewRAGSyncService(provider)
	ctx := context.Background()

	// 1. ProcessIncomingSync
	now := time.Now()
	err := service.ProcessIncomingSync(ctx, []RAGSyncRecord{
		{ID: "1", Context: "test incoming 1", Vector: []float32{0.1, 0.2}, LastSyncAt: now},
		{ID: "2", Context: "test incoming 2", Vector: []float32{0.3, 0.4}, LastSyncAt: now},
	})
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// Manually set record 2 back to pending to test fetch
	_, err = provider.Exec(ctx, "UPDATE autodream_memories SET sync_status = 'pending' WHERE id = '2'")
	if err != nil {
		t.Fatalf("failed to set pending status: %v", err)
	}

	// 2. FetchPendingSyncs
	pending, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 1 {
		t.Fatalf("Expected 1 pending record, got %d", len(pending))
	}
	if pending[0].ID != "2" {
		t.Fatalf("Expected pending record ID 2, got %s", pending[0].ID)
	}
	if pending[0].Vector[0] != 0.3 {
		t.Fatalf("Expected vector[0] to be 0.3, got %f", pending[0].Vector[0])
	}

	// 3. MarkSynced
	err = service.MarkSynced(ctx, []string{"2"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	pendingAgain, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs again failed: %v", err)
	}
	if len(pendingAgain) != 0 {
		t.Fatalf("Expected 0 pending records after MarkSynced, got %d", len(pendingAgain))
	}
}
