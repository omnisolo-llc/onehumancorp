package hub

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
	"database/sql"
)

func setupTestDB(t *testing.T) *db.DB {
	sqliteDB, err := sql.Open("sqlite", "file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}

	_, err = sqliteDB.Exec(`
		DROP TABLE IF EXISTS autodream_memories;
		CREATE TABLE autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			embedding TEXT,
			source_mission_id TEXT,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			organization_id TEXT,
			agent_id TEXT,
			source_type TEXT,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	return &db.DB{Provider: db.NewSqliteProvider(sqliteDB)}
}

func TestRAGSyncService_Implementation(t *testing.T) {
	testDB := setupTestDB(t)
	service := NewRAGSyncService(testDB)
	ctx := context.Background()

	// Insert pending record
	_, err := testDB.Exec(ctx, `
		INSERT INTO autodream_memories (id, content, embedding, sync_status)
		VALUES ('1', 'test content', '[1.0, 2.0]', 'pending')
	`)
	if err != nil {
		t.Fatalf("failed to insert pending record: %v", err)
	}

	// Fetch pending
	records, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(records) != 1 {
		t.Errorf("expected 1 record, got %d", len(records))
	}

	// Mark synced
	err = service.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	// Verify it was marked synced
	var status string
	err = testDB.QueryRow(ctx, "SELECT sync_status FROM autodream_memories WHERE id = '1'").Scan(&status)
	if err != nil {
		t.Fatalf("failed to query status: %v", err)
	}
	if status != "synced" {
		t.Errorf("expected status 'synced', got '%s'", status)
	}

	// Process incoming sync
	incoming := []RAGSyncRecord{
		{
			ID:         "2",
			Context:    "incoming context",
			Vector:     []float32{3.0, 4.0},
			SyncStatus: SyncStatusPending, // Will be forced to synced in DB
		},
	}
	err = service.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	// Verify incoming was inserted
	var content string
	err = testDB.QueryRow(ctx, "SELECT content FROM autodream_memories WHERE id = '2'").Scan(&content)
	if err != nil {
		t.Fatalf("failed to query incoming: %v", err)
	}
	if content != "incoming context" {
		t.Errorf("expected context 'incoming context', got '%s'", content)
	}
}
