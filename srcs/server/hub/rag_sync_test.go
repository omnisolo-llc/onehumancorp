package hub

import (
	"context"
	"database/sql"
	"testing"
	"time"

	_ "modernc.org/sqlite"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestRAGSyncService(t *testing.T) {
	sqliteDB, err := sql.Open("sqlite", "file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}
	defer sqliteDB.Close()

	ctx := context.Background()

	// Create table for testing
	_, err = sqliteDB.Exec(`
		DROP TABLE IF EXISTS autodream_memories;
		CREATE TABLE autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			embedding TEXT,
			sync_status TEXT DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	provider := db.NewSqliteProvider(sqliteDB)
	service := NewRAGSyncService(provider)

	// Insert test data
	_, err = sqliteDB.Exec(`
		INSERT INTO autodream_memories (id, content, embedding, sync_status)
		VALUES ('test-1', 'test content 1', '[0.1, 0.2, 0.3]', 'pending');
	`)
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}

	// Test FetchPendingSyncs
	records, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("failed to fetch pending syncs: %v", err)
	}
	if len(records) != 1 {
		t.Errorf("expected 1 record, got %d", len(records))
	}
	if records[0].ID != "test-1" {
		t.Errorf("expected id test-1, got %s", records[0].ID)
	}
	if records[0].Vector == nil || len(records[0].Vector) != 3 || records[0].Vector[0] != 0.1 {
		t.Errorf("expected vector [0.1, 0.2, 0.3], got %v", records[0].Vector)
	}

	// Test MarkSynced
	err = service.MarkSynced(ctx, []string{"test-1"})
	if err != nil {
		t.Fatalf("failed to mark synced: %v", err)
	}

	// Verify it was marked
	var status string
	err = sqliteDB.QueryRow("SELECT sync_status FROM autodream_memories WHERE id = 'test-1'").Scan(&status)
	if err != nil {
		t.Fatalf("failed to query status: %v", err)
	}
	if status != "synced" {
		t.Errorf("expected status 'synced', got %s", status)
	}

	// Test ProcessIncomingSync
	incoming := []RAGSyncRecord{
		{
			ID:         "test-2",
			Context:    "test content 2",
			Vector:     nil,
			SyncStatus: "synced",
			LastSyncAt: time.Now(),
		},
	}
	err = service.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("failed to process incoming sync: %v", err)
	}

	// Verify it was inserted
	var count int
	err = sqliteDB.QueryRow("SELECT COUNT(*) FROM autodream_memories WHERE id = 'test-2'").Scan(&count)
	if err != nil {
		t.Fatalf("failed to query count: %v", err)
	}
	if count != 1 {
		t.Errorf("expected 1 record, got %d", count)
	}
}
