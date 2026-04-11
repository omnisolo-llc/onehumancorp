package hub_test

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/hub"
)

func TestBasicRAGSyncService(t *testing.T) {
	service := hub.NewBasicRAGSyncService()

	records := []hub.RAGSyncRecord{
		{
			ID:         "mem_1",
			Context:    "User prefers dark mode",
			Vector:     []byte{1, 2, 3},
			SyncStatus: hub.SyncStatusPending,
		},
	}

	// Test ProcessIncomingSync
	err := service.ProcessIncomingSync(context.Background(), records)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	// Test FetchPendingSyncs (should be none since processed items are synced)
	pending, err := service.FetchPendingSyncs(context.Background(), 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(pending) != 0 {
		t.Fatalf("expected 0 pending, got %d", len(pending))
	}
}
