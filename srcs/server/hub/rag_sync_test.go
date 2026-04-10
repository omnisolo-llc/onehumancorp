package hub

import (
	"context"
	"database/sql"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func setupTestDB(t *testing.T) db.Provider {
	sqliteDB, err := sql.Open("sqlite", "file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("Failed to open sqlite: %v", err)
	}

	// Create table for tests (migrations are not auto-run in SQLite memory tests)
	_, err = sqliteDB.Exec(`
		CREATE TABLE autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			embedding TEXT,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_timestamp DATETIME NULL
		);
	`)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	return &db.DB{Provider: db.NewSqliteProvider(sqliteDB)}
}

func TestRAGSyncService_Implementation(t *testing.T) {
	provider := setupTestDB(t)
	defer provider.Close()

	svc := NewRAGSyncService(provider)
	ctx := context.Background()

	// 1. Initial State: Empty
	pending, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs error: %v", err)
	}
	if len(pending) != 0 {
		t.Errorf("Expected 0 pending records, got %d", len(pending))
	}

	// 2. Insert records via ProcessIncomingSync
	newRecords := []RAGSyncRecord{
		{ID: "1", Context: "test 1", Vector: []float32{0.1, 0.2}},
		{ID: "2", Context: "test 2", Vector: []float32{0.3, 0.4}},
	}
	err = svc.ProcessIncomingSync(ctx, newRecords)
	if err != nil {
		t.Fatalf("ProcessIncomingSync error: %v", err)
	}

	// Records inserted by ProcessIncomingSync are marked 'synced'
	pending, err = svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs error: %v", err)
	}
	if len(pending) != 0 {
		t.Errorf("Expected 0 pending records after ProcessIncomingSync, got %d", len(pending))
	}

	// 3. Manually insert pending records to simulate local standalone writes
	_, err = provider.Exec(ctx, `
		INSERT INTO autodream_memories (id, content, sync_status)
		VALUES ('3', 'test 3', 'pending'), ('4', 'test 4', 'pending')
	`)
	if err != nil {
		t.Fatalf("Failed to insert pending records: %v", err)
	}

	// Fetch should now return 2
	pending, err = svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs error: %v", err)
	}
	if len(pending) != 2 {
		t.Errorf("Expected 2 pending records, got %d", len(pending))
	}

	// 4. Mark one as synced
	err = svc.MarkSynced(ctx, []string{"3"})
	if err != nil {
		t.Fatalf("MarkSynced error: %v", err)
	}

	// Fetch should now return 1
	pending, err = svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs error: %v", err)
	}
	if len(pending) != 1 {
		t.Errorf("Expected 1 pending record, got %d", len(pending))
	}
	if pending[0].ID != "4" {
		t.Errorf("Expected pending record ID 4, got %s", pending[0].ID)
	}
}
