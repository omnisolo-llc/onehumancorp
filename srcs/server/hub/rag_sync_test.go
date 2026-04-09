package hub

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func TestRAGSyncService(t *testing.T) {
	ctx := context.Background()

	// 1. Setup InMemory SQLite DB for hybrid testing
	t.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory")
	dbWrapper, err := db.New(ctx)
	if err != nil {
		t.Fatalf("failed to init db: %v", err)
	}
	defer dbWrapper.Close()

	// 2. Setup schema
	if err := dbWrapper.RunMigrations(ctx); err != nil {
		t.Fatalf("failed to run migrations: %v", err)
	}

	service := NewRAGSyncService(dbWrapper)

	// Clear out any pre-existing data from migrations to have a clean slate for assertions
	_, err = dbWrapper.Exec(ctx, "DELETE FROM swarm_memory_embeddings")
	if err != nil {
		t.Fatalf("failed to clear swarm_memory_embeddings: %v", err)
	}

	// 3. Test ProcessIncomingSync (Upsert)
	now := time.Now().Round(time.Second) // Rounding for SQLite precision
	recordsToSync := []RAGSyncRecord{
		{
			ID:         "mem_1",
			Context:    "Test insight 1",
			SyncStatus: SyncStatusPending,
			LastSyncAt: now,
		},
		{
			ID:         "mem_2",
			Context:    "Test insight 2",
			SyncStatus: SyncStatusSynced,
			LastSyncAt: now,
		},
	}

	err = service.ProcessIncomingSync(ctx, recordsToSync)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// 4. Test FetchPendingSyncs
	pending, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}

	if len(pending) != 1 {
		t.Fatalf("expected 1 pending record, got %d", len(pending))
	}

	if pending[0].ID != "mem_1" {
		t.Errorf("expected pending record ID 'mem_1', got '%s'", pending[0].ID)
	}

	// 5. Test MarkSynced
	err = service.MarkSynced(ctx, []string{"mem_1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	// Verify it's no longer pending
	pendingAfter, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs after MarkSynced failed: %v", err)
	}

	if len(pendingAfter) != 0 {
		t.Fatalf("expected 0 pending records after MarkSynced, got %d", len(pendingAfter))
	}
}
