package hybrid_rag

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/hub"
)

func TestRAGSyncManager(t *testing.T) {
	ctx := context.Background()
	provider := db.NewTestProvider(t)
	manager := NewRAGSyncManager(provider, "test-org")

	// Insert test data
	_, err := provider.Exec(ctx, `INSERT INTO swarm_memory_embeddings (memory_id, context, organization_id, sync_status)
	                              VALUES ($1, $2, $3, $4)`, "mem-1", "test context", "test-org", hub.SyncStatusPending)
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	// 1. Test FetchPendingSyncs
	pending, err := manager.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 1 {
		t.Fatalf("expected 1 pending record, got %d", len(pending))
	}
	if pending[0].ID != "mem-1" {
		t.Errorf("expected ID mem-1, got %s", pending[0].ID)
	}

	// 2. Test MarkSynced
	err = manager.MarkSynced(ctx, []string{"mem-1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	// Verify update
	var status hub.SyncStatus
	err = provider.QueryRow(ctx, "SELECT sync_status FROM swarm_memory_embeddings WHERE memory_id = $1", "mem-1").Scan(&status)
	if err != nil {
		t.Fatalf("failed to query status: %v", err)
	}
	if status != hub.SyncStatusSynced {
		t.Errorf("expected status synced, got %s", status)
	}

	// 3. Test ProcessIncomingSync
	incoming := []hub.RAGSyncRecord{
		{ID: "mem-2", Context: "incoming context", SyncStatus: hub.SyncStatusPending},
	}
	err = manager.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// Verify insertion
	var contextStr string
	err = provider.QueryRow(ctx, "SELECT context FROM swarm_memory_embeddings WHERE memory_id = $1", "mem-2").Scan(&contextStr)
	if err != nil {
		t.Fatalf("failed to query context: %v", err)
	}
	if contextStr != "incoming context" {
		t.Errorf("expected context 'incoming context', got %s", contextStr)
	}
}
