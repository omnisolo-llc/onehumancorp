package hub

import (
	"context"
	"testing"
)

func TestRAGSyncService_Flow(t *testing.T) {
	service := NewRAGSyncService()

	ctx := context.Background()

	// 1. Fetch pending syncs
	records, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error fetching pending syncs: %v", err)
	}

	// 2. Process incoming syncs
	err = service.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("unexpected error processing incoming syncs: %v", err)
	}

	// 3. Mark as synced
	var idsToMark []string
	err = service.MarkSynced(ctx, idsToMark)
	if err != nil {
		t.Fatalf("unexpected error marking records synced: %v", err)
	}
}
