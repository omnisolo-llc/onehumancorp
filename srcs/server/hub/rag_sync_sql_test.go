package hub

import (
	"context"
	"database/sql"
	"testing"

    _ "github.com/mattn/go-sqlite3"
)

func TestSQLRAGSyncService(t *testing.T) {
	db, err := sql.Open("sqlite3", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite db: %v", err)
	}
	defer db.Close()

	// Setup schema
	_, err = db.Exec(`
		CREATE TABLE autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	svc := NewSQLRAGSyncService(db)
	ctx := context.Background()

	// Insert test data
	_, err = db.Exec(`INSERT INTO autodream_memories (id, content, sync_status) VALUES ('1', 'test context 1', 'pending')`)
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}
	_, err = db.Exec(`INSERT INTO autodream_memories (id, content, sync_status) VALUES ('2', 'test context 2', 'pending')`)
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}

	// Test FetchPendingSyncs
	records, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error fetching pending syncs: %v", err)
	}
	if len(records) != 2 {
		t.Fatalf("expected 2 pending records, got %d", len(records))
	}

	// Test MarkSynced
	err = svc.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("unexpected error marking synced: %v", err)
	}

	var status string
	err = db.QueryRow("SELECT sync_status FROM autodream_memories WHERE id = '1'").Scan(&status)
	if err != nil {
		t.Fatalf("failed to check status: %v", err)
	}
	if status != "synced" {
		t.Fatalf("expected status synced, got %s", status)
	}

	// Test ProcessIncomingSync
	incoming := []RAGSyncRecord{
		{ID: "3", Context: "test context 3"},
	}
	err = svc.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("unexpected error processing incoming sync: %v", err)
	}

	var count int
	err = db.QueryRow("SELECT COUNT(*) FROM autodream_memories WHERE id = '3'").Scan(&count)
	if err != nil {
		t.Fatalf("failed to check incoming sync count: %v", err)
	}
	if count != 1 {
		t.Fatalf("expected 1 incoming sync, got %d", count)
	}

	// Test ProcessIncomingSync - Update
	incomingUpdate := []RAGSyncRecord{
		{ID: "3", Context: "test context 3 updated"},
	}
	err = svc.ProcessIncomingSync(ctx, incomingUpdate)
	if err != nil {
		t.Fatalf("unexpected error processing incoming sync update: %v", err)
	}

	var content string
	err = db.QueryRow("SELECT content FROM autodream_memories WHERE id = '3'").Scan(&content)
	if err != nil {
		t.Fatalf("failed to check incoming sync content: %v", err)
	}
	if content != "test context 3 updated" {
		t.Fatalf("expected 'test context 3 updated', got '%s'", content)
	}
}
