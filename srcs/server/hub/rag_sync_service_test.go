package hub

import (
	"context"
	"database/sql"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func TestRAGSyncServiceImplementation(t *testing.T) {
	d, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}
	defer d.Close()

	provider := db.NewSqliteProvider(d)
	ctx := context.Background()

	// Setup schema
	_, err = provider.Exec(ctx, `
		CREATE TABLE autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL
		);
	`)
	if err != nil {
		t.Fatalf("failed to create schema: %v", err)
	}

	svc := NewRAGSyncService(provider)

	// Test 1: Insert pending and fetch
	_, err = provider.Exec(ctx, "INSERT INTO autodream_memories (id, content, sync_status) VALUES (?, ?, ?)", "id-1", "context 1", "pending")
	if err != nil {
		t.Fatalf("failed to insert: %v", err)
	}

	records, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error fetching pending: %v", err)
	}
	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}
	if records[0].ID != "id-1" {
		t.Errorf("expected id-1, got %s", records[0].ID)
	}

	// Test 2: Mark synced
	err = svc.MarkSynced(ctx, []string{"id-1"})
	if err != nil {
		t.Fatalf("unexpected error marking synced: %v", err)
	}

	// Verify it's no longer pending
	records, err = svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error fetching pending: %v", err)
	}
	if len(records) != 0 {
		t.Fatalf("expected 0 records, got %d", len(records))
	}

	// Test 3: Process incoming sync
	incoming := []RAGSyncRecord{
		{ID: "id-2", Context: "context 2", SyncStatus: SyncStatusSynced, LastSyncAt: time.Now()},
		{ID: "id-1", Context: "context 1 updated", SyncStatus: SyncStatusSynced, LastSyncAt: time.Now()},
	}

	err = svc.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("unexpected error processing incoming sync: %v", err)
	}

	// Verify upsert
	var count int
	err = provider.QueryRow(ctx, "SELECT COUNT(*) FROM autodream_memories WHERE sync_status = 'synced'").Scan(&count)
	if err != nil {
		t.Fatalf("unexpected error querying count: %v", err)
	}
	if count != 2 {
		t.Fatalf("expected 2 synced records, got %d", count)
	}
}
