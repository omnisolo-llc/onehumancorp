package hub

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestRAGSyncService(t *testing.T) {
	InitRAGSyncMetrics(nil)

	t.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory")
	ctx := context.Background()
	provider, err := db.New(ctx)
	if err != nil {
		t.Fatalf("failed to init db: %v", err)
	}
	defer provider.Close()

	// create table manually for tests
	_, err = provider.Exec(ctx, `
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

	svc := NewRAGSyncService(provider)
	now := time.Now()

	// Process an incoming sync to setup data
	records := []RAGSyncRecord{
		{
			ID:         "test-mem-1",
			Context:    "some context info",
			Vector:     []byte{1, 2, 3},
			SyncStatus: SyncStatusPending,
			LastSyncAt: &now,
		},
	}
	err = svc.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// Fetch pending syncs
	pending, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 1 {
		t.Fatalf("expected 1 pending record, got %d", len(pending))
	}
	if pending[0].ID != "test-mem-1" {
		t.Fatalf("expected ID test-mem-1, got %s", pending[0].ID)
	}
	if len(pending[0].Vector) != 3 {
		t.Fatalf("expected vector of length 3, got %d", len(pending[0].Vector))
	}

	// Mark as synced
	err = svc.MarkSynced(ctx, []string{"test-mem-1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	// Fetch pending syncs again
	pending, err = svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 0 {
		t.Fatalf("expected 0 pending records after MarkSynced, got %d", len(pending))
	}
}
