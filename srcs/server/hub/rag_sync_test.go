package hub

import (
	"context"
	"database/sql"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func setupTestDB(t *testing.T) *db.DB {
	sqliteDB, err := sql.Open("sqlite", "file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}

	dbWrapper := &db.DB{Provider: db.NewSqliteProvider(sqliteDB)}

	// Create table
	_, err = dbWrapper.Exec(context.Background(), `
		DROP TABLE IF EXISTS autodream_memories;
		CREATE TABLE autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			sync_status TEXT DEFAULT 'pending',
			last_sync_at DATETIME
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	return dbWrapper
}

func TestRAGSyncService_Flow(t *testing.T) {
	dbWrapper := setupTestDB(t)
	svc := NewRAGSyncService(dbWrapper.Provider)
	ctx := context.Background()

	// 1. Process incoming sync
	incoming := []RAGSyncRecord{
		{ID: "record-1", Context: "test context 1"},
	}
	err := svc.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// 2. Add a pending record
	_, err = dbWrapper.Exec(ctx, "INSERT INTO autodream_memories (id, content, sync_status) VALUES ('record-2', 'test context 2', 'pending')")
	if err != nil {
		t.Fatalf("failed to insert pending record: %v", err)
	}

	// 3. Fetch pending
	pending, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}

	if len(pending) != 1 {
		t.Fatalf("expected 1 pending record, got %d", len(pending))
	}

	if pending[0].ID != "record-2" {
		t.Fatalf("expected pending record ID 'record-2', got '%s'", pending[0].ID)
	}

	// 4. Mark synced
	err = svc.MarkSynced(ctx, []string{"record-2"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	// 5. Fetch again to verify
	pending2, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}

	if len(pending2) != 0 {
		t.Fatalf("expected 0 pending records, got %d", len(pending2))
	}
}
