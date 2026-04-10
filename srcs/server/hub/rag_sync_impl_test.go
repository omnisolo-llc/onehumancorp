package hub

import (
	"context"
	"testing"

	_ "modernc.org/sqlite"
	"database/sql"
)

func TestRAGSyncServiceImpl(t *testing.T) {
	d, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open memory db: %v", err)
	}
	defer d.Close()

	// Create table and indexes for test. SQLite style.
	_, err = d.Exec(`
		CREATE TABLE IF NOT EXISTS swarm_memory_embeddings (
			memory_id        TEXT PRIMARY KEY,
			context          TEXT NOT NULL,
			vector_embedding BLOB,
			source_plugin    TEXT,
			created_at       TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
			sync_status      VARCHAR(50) DEFAULT 'pending',
			last_sync_at     TIMESTAMP NULL
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	svc := NewRAGSyncService(d)
	ctx := context.Background()

	// Insert some mock data
	_, err = d.Exec(`
		INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status)
		VALUES
			('mem1', 'Context 1', x'010203', 'pending'),
			('mem2', 'Context 2', x'040506', 'pending'),
			('mem3', 'Context 3', x'070809', 'synced');
	`)
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}

	// Test FetchPendingSyncs
	pending, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 2 {
		t.Errorf("expected 2 pending records, got %d", len(pending))
	}

	// Test MarkSynced
	err = svc.MarkSynced(ctx, []string{"mem1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	pending, err = svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 1 {
		t.Errorf("expected 1 pending record, got %d", len(pending))
	}

	// Test ProcessIncomingSync
	incoming := []RAGSyncRecord{
		{ID: "mem4", Context: "Context 4", Vector: []byte{10, 11, 12}},
		{ID: "mem2", Context: "Updated Context 2", Vector: []byte{4, 5, 6}},
	}
	err = svc.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// Verify incoming synced
	var syncStatus string
	var contextStr string
	err = d.QueryRow("SELECT sync_status, context FROM swarm_memory_embeddings WHERE memory_id = 'mem4'").Scan(&syncStatus, &contextStr)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}
	if syncStatus != "synced" {
		t.Errorf("expected status 'synced', got %s", syncStatus)
	}
	if contextStr != "Context 4" {
		t.Errorf("expected context 'Context 4', got %s", contextStr)
	}
}
