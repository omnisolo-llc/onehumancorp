package hub

import (
	"context"
	"database/sql"
	"os"
	"testing"
	"time"


	_ "modernc.org/sqlite"
)

func TestDBRAGSyncService(t *testing.T) {
	// Setup a temporary sqlite database
	tempFile, err := os.CreateTemp("", "testdb_*.sqlite")
	if err != nil {
		t.Fatalf("failed to create temp file: %v", err)
	}
	defer os.Remove(tempFile.Name())

	database, err := sql.Open("sqlite", tempFile.Name())
	if err != nil {
		t.Fatalf("failed to open database: %v", err)
	}
	defer database.Close()

	// Create tables
	_, err = database.Exec(`
		CREATE TABLE autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			embedding BLOB,
			sync_status TEXT DEFAULT 'pending',
			last_sync_at DATETIME
		);
		CREATE TABLE consolidated_memory (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			agent_id TEXT,
			content TEXT NOT NULL,
			embedding BLOB,
			source_type TEXT NOT NULL,
			sync_status TEXT DEFAULT 'pending',
			last_sync_at DATETIME
		);
	`)
	if err != nil {
		t.Fatalf("failed to create tables: %v", err)
	}

	// Insert test data
	_, err = database.Exec(`
		INSERT INTO autodream_memories (id, content, embedding, sync_status)
		VALUES ('test-1', 'test context', X'0102', 'pending')
	`)
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	service := NewDBRAGSyncService(database)
	ctx := context.Background()

	// Test FetchPendingSyncs
	records, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}
	if records[0].ID != "test-1" {
		t.Errorf("expected ID test-1, got %s", records[0].ID)
	}

	// Test MarkSynced
	err = service.MarkSynced(ctx, []string{"test-1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	records, err = service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(records) != 0 {
		t.Fatalf("expected 0 records after MarkSynced, got %d", len(records))
	}

	// Test ProcessIncomingSync
	incoming := []RAGSyncRecord{
		{
			ID:         "test-2",
			Context:    "incoming context",
			Vector:     []byte{3, 4},
			SyncStatus: SyncStatusSynced,
			LastSyncAt: time.Now(),
		},
	}
	err = service.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	var count int
	err = database.QueryRow(`SELECT count(*) FROM consolidated_memory WHERE id = 'test-2'`).Scan(&count)
	if err != nil {
		t.Fatalf("failed to query consolidated_memory: %v", err)
	}
	if count != 1 {
		t.Fatalf("expected 1 record in consolidated_memory, got %d", count)
	}
}
