package hub

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestRAGSyncService(t *testing.T) {
	provider := db.NewTestProvider(t)

	// Create table manually since NewTestProvider might not run migrations
	_, err := provider.Exec(context.Background(), `
		CREATE TABLE IF NOT EXISTS autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			sync_status TEXT DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL
		)
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	_, err = provider.Exec(context.Background(), `
		INSERT INTO autodream_memories (id, content, sync_status) VALUES ('1', 'hello world', 'pending'), ('2', 'hello there', 'synced')
	`)
	if err != nil {
		t.Fatalf("failed to insert records: %v", err)
	}

	svc := NewRAGSyncService(provider)

	pending, err := svc.FetchPendingSyncs(context.Background(), 10)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(pending) != 1 {
		t.Fatalf("expected 1 pending record, got %d", len(pending))
	}
	if pending[0].ID != "1" {
		t.Fatalf("expected pending record ID 1, got %s", pending[0].ID)
	}

	err = svc.MarkSynced(context.Background(), []string{"1"})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	pendingAfter, err := svc.FetchPendingSyncs(context.Background(), 10)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(pendingAfter) != 0 {
		t.Fatalf("expected 0 pending records after mark, got %d", len(pendingAfter))
	}

	err = svc.ProcessIncomingSync(context.Background(), []RAGSyncRecord{
		{ID: "3", Context: "newly pushed sync"},
	})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	var count int
	row := provider.QueryRow(context.Background(), `SELECT COUNT(*) FROM autodream_memories WHERE id = '3' AND sync_status = 'synced'`)
	if err := row.Scan(&count); err != nil {
		t.Fatalf("failed to query processed count: %v", err)
	}
	if count != 1 {
		t.Fatalf("expected to find 1 synced record, got %d", count)
	}
}
