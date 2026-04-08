package hub

import (
	"context"
	"database/sql"
	"testing"
	"time"

	_ "modernc.org/sqlite"
)

func setupTestDB(t *testing.T) *sql.DB {
	db, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite db: %v", err)
	}

	_, err = db.Exec(`
		CREATE TABLE autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			sync_status TEXT DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL,
			created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
		)
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	return db
}

func TestDefaultRAGSyncService(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()

	// Use our service which expects a DBProvider
	svc := NewDefaultRAGSyncService(db)
	ctx := context.Background()

	// 1. Insert pending records manually
	_, err := db.Exec(`INSERT INTO autodream_memories (id, content, sync_status) VALUES ('1', 'test1', 'pending'), ('2', 'test2', 'pending')`)
	if err != nil {
		t.Fatalf("insert failed: %v", err)
	}

	// 2. FetchPendingSyncs
	pending, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs error: %v", err)
	}
	if len(pending) != 2 {
		t.Fatalf("expected 2 pending, got %d", len(pending))
	}

	// 3. MarkSynced
	err = svc.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("MarkSynced error: %v", err)
	}

	pendingAfterMark, _ := svc.FetchPendingSyncs(ctx, 10)
	if len(pendingAfterMark) != 1 {
		t.Fatalf("expected 1 pending after mark, got %d", len(pendingAfterMark))
	}
	if pendingAfterMark[0].ID != "2" {
		t.Fatalf("expected ID 2 to still be pending, got %s", pendingAfterMark[0].ID)
	}
}

func TestDefaultRAGSyncService_ProcessIncoming(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()

	svc := NewDefaultRAGSyncService(db)
	ctx := context.Background()

	// Create new record
	now := time.Now()
	recs := []RAGSyncRecord{
		{
			ID: "3", Context: "test3", SyncStatus: SyncStatusSynced, LastSyncAt: now,
		},
	}

	err := svc.ProcessIncomingSync(ctx, recs)
	if err != nil {
		t.Fatalf("ProcessIncomingSync error: %v", err)
	}

	// Verify insertion
	var content, status string
	err = db.QueryRow("SELECT content, sync_status FROM autodream_memories WHERE id = '3'").Scan(&content, &status)
	if err != nil {
		t.Fatalf("failed to query processed record: %v", err)
	}
	if content != "test3" || status != "synced" {
		t.Fatalf("unexpected values: %s %s", content, status)
	}

	// Update existing record
	recs[0].Context = "test3-updated"
	err = svc.ProcessIncomingSync(ctx, recs)
	if err != nil {
		t.Fatalf("ProcessIncomingSync update error: %v", err)
	}
	err = db.QueryRow("SELECT content FROM autodream_memories WHERE id = '3'").Scan(&content)
	if err != nil {
		t.Fatalf("failed to query updated record: %v", err)
	}
	if content != "test3-updated" {
		t.Fatalf("unexpected updated value: %s", content)
	}
}
