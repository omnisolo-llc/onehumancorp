package hub

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func TestRAGSyncService(t *testing.T) {
	t.Setenv("DATABASE_URL", "sqlite://file::memory:?cache=shared")
	ctx := context.Background()
	dbWrapper, err := db.New(ctx)
	if err != nil {
		t.Fatalf("failed to init db: %v", err)
	}
	defer dbWrapper.Close()

	// Manually run migration-like setup since memory db is fresh
	_, err = dbWrapper.Exec(ctx, `CREATE TABLE agent_memories (
		id TEXT PRIMARY KEY,
		organization_id TEXT NOT NULL,
		content TEXT NOT NULL,
		embedding TEXT,
		sync_status TEXT DEFAULT 'pending',
		last_sync_at TIMESTAMP
	)`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	service := NewSQLRAGSyncService(dbWrapper)

	// 1. ProcessIncomingSync
	recordsToIncoming := []RAGSyncRecord{
		{ID: "mem1", Context: "Context 1", Vector: []float32{0.1, 0.2}, SyncStatus: SyncStatusPending},
		{ID: "mem2", Context: "Context 2", Vector: []float32{0.3, 0.4}, SyncStatus: SyncStatusPending},
	}
	err = service.ProcessIncomingSync(ctx, recordsToIncoming)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// 2. FetchPendingSyncs
	pending, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 2 {
		t.Fatalf("expected 2 pending records, got %d", len(pending))
	}

	// Verify contents
	if pending[0].Context != "Context 1" && pending[1].Context != "Context 1" {
		t.Errorf("missing Context 1")
	}
	if len(pending[0].Vector) != 2 {
		t.Errorf("missing Vector")
	}

	// 3. MarkSynced
	err = service.MarkSynced(ctx, []string{"mem1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	// Re-fetch pending, should be 1
	pendingAfter, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pendingAfter) != 1 {
		t.Fatalf("expected 1 pending record after MarkSynced, got %d", len(pendingAfter))
	}
	if pendingAfter[0].ID != "mem2" {
		t.Errorf("expected mem2 to still be pending")
	}
}
