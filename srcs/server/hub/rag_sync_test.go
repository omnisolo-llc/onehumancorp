package hub

import (
	"context"
	"database/sql"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite" // use modernc sqlite driver
)

func TestDefaultRAGSyncService(t *testing.T) {
	// Create an in-memory db directly
	sqlDB, err := sql.Open("sqlite", "file::memory:?mode=memory")
	if err != nil {
		t.Fatalf("failed to open sqlite db: %v", err)
	}
	defer sqlDB.Close()

	// Apply migrations using db package methods or manual schema for our test
	_, err = sqlDB.Exec(`
	CREATE TABLE IF NOT EXISTS autodream_memories (
		id TEXT PRIMARY KEY,
		content TEXT NOT NULL,
		sync_status VARCHAR(50) DEFAULT 'pending',
		last_sync_at TIMESTAMP NULL
	);
	`)
	if err != nil {
		t.Fatalf("failed to create schema: %v", err)
	}

	provider := db.NewSqliteProvider(sqlDB)
	ctx := context.Background()

	// Setup initial data
	_, err = provider.Exec(ctx, "INSERT INTO autodream_memories (id, content, sync_status) VALUES ('1', 'hello', 'pending')")
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}
	_, err = provider.Exec(ctx, "INSERT INTO autodream_memories (id, content, sync_status) VALUES ('2', 'world', 'pending')")
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	svc := NewRAGSyncService(provider)

	pending, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(pending) != 2 {
		t.Fatalf("expected 2 pending records, got %d", len(pending))
	}

	err = svc.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	pending, err = svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(pending) != 1 {
		t.Fatalf("expected 1 pending record, got %d", len(pending))
	}

	err = svc.ProcessIncomingSync(ctx, []RAGSyncRecord{
		{ID: "3", Context: "foo"},
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	// Verify the new record was inserted
	var count int
	err = provider.QueryRow(ctx, "SELECT COUNT(*) FROM autodream_memories WHERE id = '3' AND sync_status = 'synced'").Scan(&count)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if count != 1 {
		t.Fatalf("expected 1 synced record for id 3, got %d", count)
	}
}
