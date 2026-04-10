package hub

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestRAGSyncService_Implementation(t *testing.T) {
	t.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory")
	ctx := context.Background()
	provider, err := db.New(ctx)
	if err != nil {
		t.Fatalf("failed to connect to db: %v", err)
	}
	defer provider.Close()

	// Initial table setup for tests since migrations might not all run correctly in this isolated test
	_, err = provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS swarm_memory_embeddings (
			memory_id TEXT PRIMARY KEY,
			context TEXT NOT NULL,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMPTZ NULL
		)
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	InitRAGSyncMetrics(nil)

	service := NewRAGSyncService(provider)

	// Seed some data
	_, err = provider.Exec(ctx, `INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES ('1', 'ctx1', 'pending'), ('2', 'ctx2', 'pending'), ('3', 'ctx3', 'synced')`)
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}

	// Test FetchPendingSyncs
	pending, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 2 {
		t.Errorf("expected 2 pending records, got %d", len(pending))
	}

	// Test MarkSynced
	err = service.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	pending, err = service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 1 {
		t.Errorf("expected 1 pending record after MarkSynced, got %d", len(pending))
	}

	// Test ProcessIncomingSync
	incoming := []RAGSyncRecord{
		{ID: "4", Context: "ctx4"},
	}
	err = service.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// Verify incoming sync was added
	row := provider.QueryRow(ctx, `SELECT context, sync_status FROM swarm_memory_embeddings WHERE memory_id = '4'`)
	var ctxVal, status string
	err = row.Scan(&ctxVal, &status)
	if err != nil {
		t.Fatalf("failed to scan row: %v", err)
	}
	if ctxVal != "ctx4" || status != "synced" {
		t.Errorf("unexpected row values: %s, %s", ctxVal, status)
	}
}
