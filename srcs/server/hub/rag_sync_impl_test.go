package hub

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestRAGSyncServiceImpl(t *testing.T) {
	ctx := context.Background()
	provider := db.NewTestProvider(t)

	// Ensure table exists for test
	_, err := provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS swarm_memory_embeddings (
			memory_id        TEXT PRIMARY KEY,
			context          TEXT NOT NULL,
			vector_embedding BLOB,
			source_plugin    TEXT,
			created_at       DATETIME DEFAULT CURRENT_TIMESTAMP,
			sync_status      VARCHAR(50) DEFAULT 'pending',
			last_sync_at     DATETIME NULL
		);
	`)
	if err != nil {
		t.Fatalf("failed to create test table: %v", err)
	}

	svc := NewRAGSyncService(provider)

	// Seed data
	_, err = provider.Exec(ctx, `
		INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status)
		VALUES ('id-1', 'context-1', 'pending'), ('id-2', 'context-2', 'synced')
	`)
	if err != nil {
		t.Fatalf("failed to seed data: %v", err)
	}

	// Test FetchPendingSyncs
	pending, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 1 {
		t.Errorf("expected 1 pending record, got %d", len(pending))
	}
	if pending[0].ID != "id-1" {
		t.Errorf("expected id-1, got %s", pending[0].ID)
	}

	// Test MarkSynced
	err = svc.MarkSynced(ctx, []string{"id-1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	// Verify MarkSynced
	pending2, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs2 failed: %v", err)
	}
	if len(pending2) != 0 {
		t.Errorf("expected 0 pending records after MarkSynced, got %d", len(pending2))
	}

	// Test ProcessIncomingSync
	incoming := []RAGSyncRecord{
		{
			ID:         "id-3",
			Context:    "context-3",
			SyncStatus: SyncStatusSynced,
			LastSyncAt: time.Now(),
		},
	}
	err = svc.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// Verify ProcessIncomingSync
	var ctxVal string
	row := provider.QueryRow(ctx, "SELECT context FROM swarm_memory_embeddings WHERE memory_id = 'id-3'")
	if err := row.Scan(&ctxVal); err != nil {
		t.Fatalf("failed to query processed record: %v", err)
	}
	if ctxVal != "context-3" {
		t.Errorf("expected context-3, got %s", ctxVal)
	}
}
