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
		t.Fatalf("failed to open sqlite db: %v", err)
	}

	prov := db.NewSqliteProvider(sqliteDB)

	ctx := context.Background()

	// Create table
	_, err = prov.Exec(ctx, "DROP TABLE IF EXISTS autodream_memories;")
	if err != nil {
		t.Fatalf("failed to drop table: %v", err)
	}

	createTableSQL := `
	CREATE TABLE autodream_memories (
		id TEXT PRIMARY KEY,
		content TEXT NOT NULL,
		embedding TEXT,
		source_mission_id TEXT,
		created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
		organization_id TEXT,
		agent_id TEXT,
		source_type TEXT,
		sync_status TEXT DEFAULT 'pending',
		last_sync_at DATETIME
	);
	`
	_, err = prov.Exec(ctx, createTableSQL)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	return prov
}

func TestSQLRAGSyncService(t *testing.T) {
	prov := setupTestDB(t)
	defer prov.Close()

	srv := NewSQLRAGSyncService(prov)
	ctx := context.Background()

	// Insert some dummy data
	_, err := prov.Exec(ctx, "INSERT INTO autodream_memories (id, content, sync_status) VALUES ('1', 'test1', 'pending'), ('2', 'test2', 'pending'), ('3', 'test3', 'synced')")
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	pending, err := srv.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(pending) != 2 {
		t.Fatalf("expected 2 pending records, got %d", len(pending))
	}

	err = srv.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	pending, err = srv.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(pending) != 1 {
		t.Fatalf("expected 1 pending record, got %d", len(pending))
	}

	err = srv.ProcessIncomingSync(ctx, []RAGSyncRecord{
		{ID: "4", Context: "test4", SyncStatus: SyncStatusSynced},
		{ID: "1", Context: "test1 updated", SyncStatus: SyncStatusSynced},
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	row := prov.QueryRow(ctx, "SELECT COUNT(*) FROM autodream_memories")
	var count int
	err = row.Scan(&count)
	if err != nil {
		t.Fatalf("unexpected error scanning count: %v", err)
	}
	if count != 4 {
		t.Fatalf("expected 4 records, got %d", count)
	}

	row = prov.QueryRow(ctx, "SELECT content FROM autodream_memories WHERE id = '1'")
	var content string
	err = row.Scan(&content)
	if err != nil {
		t.Fatalf("unexpected error scanning content: %v", err)
	}
	if content != "test1 updated" {
		t.Fatalf("expected content 'test1 updated', got '%s'", content)
	}
}
