package hub

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestRAGSyncServiceFlow_Standalone(t *testing.T) {
	provider := db.NewTestProvider(t)
	defer provider.Close()

	ctx := context.Background()

	// Need to manually create the autodream_memories table for testing
	// if SetupTestDB doesn't run the specific migration.
	_, err := provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS autodream_memories (
			id TEXT PRIMARY KEY,
			context TEXT,
			vector TEXT,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL
		)
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	// Insert initial test data
	_, err = provider.Exec(ctx, `
		INSERT INTO autodream_memories (id, context, sync_status)
		VALUES
			('1', 'Memory 1', 'pending'),
			('2', 'Memory 2', 'synced'),
			('3', 'Memory 3', 'pending')
	`)
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	svc := NewRAGSyncService(provider)

	// 1. Fetch pending syncs
	pending, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("Failed to fetch pending syncs: %v", err)
	}
	if len(pending) != 2 {
		t.Errorf("Expected 2 pending syncs, got %d", len(pending))
	}

	// 2. Mark synced
	var ids []string
	for _, r := range pending {
		ids = append(ids, r.ID)
	}
	err = svc.MarkSynced(ctx, ids)
	if err != nil {
		t.Fatalf("Failed to mark synced: %v", err)
	}

	// 3. Verify no pending syncs
	pending, err = svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("Failed to fetch pending syncs: %v", err)
	}
	if len(pending) != 0 {
		t.Errorf("Expected 0 pending syncs, got %d", len(pending))
	}
}

func TestRAGSyncServiceFlow_Cloud(t *testing.T) {
	// Simple mock check to ensure cloud operations fail appropriately on IsSQLite=true
	provider := db.NewTestProvider(t)
	defer provider.Close()

	ctx := context.Background()
	svc := NewRAGSyncService(provider)

	err := svc.ProcessIncomingSync(ctx, []RAGSyncRecord{
		{ID: "1", Context: "Memory 1", SyncStatus: SyncStatusPending},
	})

	if err == nil {
		t.Fatalf("Expected ProcessIncomingSync to fail on SQLite")
	}
	if err.Error() != "ProcessIncomingSync is only supported in Cloud mode" {
		t.Fatalf("Unexpected error: %v", err)
	}
}
