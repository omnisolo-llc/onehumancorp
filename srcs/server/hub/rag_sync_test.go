package hub

import (
	"context"
	"encoding/json"
	"os"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestRAGSyncService(t *testing.T) {
	ctx := context.Background()

	// Use SQLite for testing
	os.Setenv("DATABASE_URL", "sqlite://:memory:")
	provider, err := db.New(ctx)
	if err != nil {
		t.Fatalf("Failed to create db provider: %v", err)
	}
	defer provider.Close()

	// Ensure the table and new columns exist
	_, err = provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS swarm_memory_embeddings (
			memory_id TEXT PRIMARY KEY,
			context TEXT NOT NULL,
			vector_embedding BLOB,
			sync_status TEXT DEFAULT 'pending',
			last_sync_at TEXT NULL
		)
	`)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	service := NewRAGSyncService(provider)

	// Test 1: FetchPendingSyncs when empty
	records, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(records) != 0 {
		t.Fatalf("Expected 0 records, got %d", len(records))
	}

	// Insert test data
	vecData, _ := json.Marshal([]float32{1.0, 2.0, 3.0})
	_, err = provider.Exec(ctx, `
		INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status)
		VALUES ('test-1', 'Test context', $1, 'pending')
	`, vecData)
	if err != nil {
		t.Fatalf("Failed to insert test data: %v", err)
	}

	// Test 2: FetchPendingSyncs
	records, err = service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(records) != 1 {
		t.Fatalf("Expected 1 record, got %d", len(records))
	}
	if records[0].ID != "test-1" {
		t.Errorf("Expected ID test-1, got %s", records[0].ID)
	}
	if len(records[0].Vector) != 3 {
		t.Errorf("Expected vector of length 3, got %v", records[0].Vector)
	}

	// Test 3: MarkSynced
	err = service.MarkSynced(ctx, []string{"test-1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	// Verify sync_status is updated
	records, err = service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(records) != 0 {
		t.Fatalf("Expected 0 pending records after MarkSynced, got %d", len(records))
	}

	// Test 4: ProcessIncomingSync
	incoming := []RAGSyncRecord{
		{
			ID:      "test-2",
			Context: "Incoming context",
			Vector:  []float32{4.0, 5.0},
		},
		{
			ID:      "test-1", // Existing
			Context: "Updated context",
			Vector:  []float32{1.1, 2.2, 3.3},
		},
	}

	err = service.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// Verify data was processed
	var ctxStr string
	err = provider.QueryRow(ctx, "SELECT context FROM swarm_memory_embeddings WHERE memory_id = 'test-2'").Scan(&ctxStr)
	if err != nil {
		t.Fatalf("Failed to verify test-2: %v", err)
	}
	if ctxStr != "Incoming context" {
		t.Errorf("Expected 'Incoming context', got '%s'", ctxStr)
	}

	err = provider.QueryRow(ctx, "SELECT context FROM swarm_memory_embeddings WHERE memory_id = 'test-1'").Scan(&ctxStr)
	if err != nil {
		t.Fatalf("Failed to verify test-1: %v", err)
	}
	if ctxStr != "Updated context" {
		t.Errorf("Expected 'Updated context', got '%s'", ctxStr)
	}
}
