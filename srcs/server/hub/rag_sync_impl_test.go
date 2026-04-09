package hub

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestRAGSyncServiceImpl(t *testing.T) {
	provider := db.NewTestProvider(t)
	database := &db.DB{Provider: provider}

	// Run migrations
	ctx := context.Background()
	if err := database.RunMigrations(ctx); err != nil {
		t.Fatalf("failed to run migrations: %v", err)
	}

	service := NewRAGSyncService(database)

	// Insert test data
	_, err := database.Exec(ctx, "INSERT INTO autodream_memories (id, content, sync_status) VALUES ('test-id-1', 'test content 1', 'pending')")
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	// Test FetchPendingSyncs
	pending, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 1 {
		t.Fatalf("expected 1 pending record, got %d", len(pending))
	}
	if pending[0].ID != "test-id-1" {
		t.Fatalf("expected id 'test-id-1', got '%s'", pending[0].ID)
	}

	// Test MarkSynced
	err = service.MarkSynced(ctx, []string{"test-id-1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	// Verify MarkSynced
	pending, err = service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 0 {
		t.Fatalf("expected 0 pending records after MarkSynced, got %d", len(pending))
	}

	// Test ProcessIncomingSync
	incoming := []RAGSyncRecord{
		{
			ID:         "test-id-2",
			Context:    "test content 2",
			Vector:     []float32{0.1, 0.2, 0.3},
			SyncStatus: SyncStatusSynced,
			LastSyncAt: time.Now(),
		},
	}
	err = service.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// Verify ProcessIncomingSync
	var count int
	err = database.QueryRow(ctx, "SELECT COUNT(*) FROM autodream_memories WHERE id = 'test-id-2' AND sync_status = 'synced'").Scan(&count)
	if err != nil {
		t.Fatalf("failed to query database: %v", err)
	}
	if count != 1 {
		t.Fatalf("expected incoming record to be inserted, got count %d", count)
	}
}
