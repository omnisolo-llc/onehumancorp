package hub

import (
	"context"
	"database/sql"
	"testing"

	_ "modernc.org/sqlite"
)

func TestRAGSyncService(t *testing.T) {
	db, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("Failed to open sqlite db: %v", err)
	}
	defer db.Close()

	// Initialize schema
	schema := `
		CREATE TABLE swarm_memory_embeddings (
			memory_id TEXT PRIMARY KEY,
			context TEXT NOT NULL,
			vector_embedding BLOB,
			source_plugin TEXT,
			created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL
		);
	`
	if _, err := db.Exec(schema); err != nil {
		t.Fatalf("Failed to create schema: %v", err)
	}

	svc := NewRAGSyncService(db)
	ctx := context.Background()

	// Test ProcessIncomingSync
	records := []RAGSyncRecord{
		{ID: "mem1", Context: "test context 1", Vector: []float32{1.0, 2.0}},
		{ID: "mem2", Context: "test context 2", Vector: []float32{3.0, 4.0}},
	}
	err = svc.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// Insert a pending sync manually
	_, err = db.Exec(`INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES ('mem3', 'pending context', 'pending')`)
	if err != nil {
		t.Fatalf("Failed to insert pending sync: %v", err)
	}

	// Test FetchPendingSyncs
	pending, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 1 {
		t.Errorf("Expected 1 pending record, got %d", len(pending))
	}
	if pending[0].ID != "mem3" {
		t.Errorf("Expected pending ID 'mem3', got '%s'", pending[0].ID)
	}

	// Test MarkSynced
	err = svc.MarkSynced(ctx, []string{"mem3"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	// Verify it's synced
	pending, err = svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 0 {
		t.Errorf("Expected 0 pending records after MarkSynced, got %d", len(pending))
	}
}
