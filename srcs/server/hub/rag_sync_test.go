package hub

import (
	"context"
	"database/sql"
	"testing"

	_ "modernc.org/sqlite"
)

func TestDefaultRAGSyncService(t *testing.T) {
	db, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open memory db: %v", err)
	}
	defer db.Close()

	_, err = db.Exec(`
		CREATE TABLE autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			sync_status TEXT DEFAULT 'pending',
			last_sync_at DATETIME
		)
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	service := NewDefaultRAGSyncService(db)
	ctx := context.Background()

	// Seed data
	_, err = db.Exec("INSERT INTO autodream_memories (id, content, sync_status) VALUES ('1', 'hello', 'pending')")
	if err != nil {
		t.Fatalf("failed to seed: %v", err)
	}

	// Fetch
	pending, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("failed to fetch: %v", err)
	}
	if len(pending) != 1 {
		t.Fatalf("expected 1, got %d", len(pending))
	}

	// Mark synced
	err = service.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("failed to mark synced: %v", err)
	}

	// Check fetch again
	pending2, _ := service.FetchPendingSyncs(ctx, 10)
	if len(pending2) != 0 {
		t.Fatalf("expected 0, got %d", len(pending2))
	}

	// Process incoming
	incoming := []RAGSyncRecord{
		{ID: "2", Context: "world"},
	}

	err = service.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("failed to process incoming: %v", err)
	}

	var count int
	err = db.QueryRow("SELECT COUNT(*) FROM autodream_memories WHERE id = '2'").Scan(&count)
	if err != nil {
		t.Fatalf("failed to query count: %v", err)
	}
	if count != 1 {
		t.Fatalf("expected 1 inserted record, got %d", count)
	}
}
