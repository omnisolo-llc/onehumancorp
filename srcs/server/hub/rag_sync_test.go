package hub

import (
	"context"
	"testing"
)

func TestMockRAGSyncService(t *testing.T) {
	mock := &MockRAGSyncService{
		PendingRecords: []RAGSyncRecord{
			{ID: "test1", Context: "test context 1", SyncStatus: SyncStatusPending},
			{ID: "test2", Context: "test context 2", SyncStatus: SyncStatusPending},
		},
	}

	ctx := context.Background()

	// Test FetchPendingSyncs
	pending, err := mock.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("Unexpected error: %v", err)
	}
	if len(pending) != 2 {
		t.Fatalf("Expected 2 pending records, got %d", len(pending))
	}

	// Test MarkSynced
	ids := []string{"test1", "test2"}
	err = mock.MarkSynced(ctx, ids)
	if err != nil {
		t.Fatalf("Unexpected error: %v", err)
	}
	if len(mock.SyncedIDs) != 2 {
		t.Fatalf("Expected 2 synced IDs, got %d", len(mock.SyncedIDs))
	}

	// Test ProcessIncomingSync
	err = mock.ProcessIncomingSync(ctx, pending)
	if err != nil {
		t.Fatalf("Unexpected error: %v", err)
	}
	if len(mock.Incoming) != 2 {
		t.Fatalf("Expected 2 incoming records processed, got %d", len(mock.Incoming))
	}

	// Test metrics incrementing (basic panics check)
	RagRecordsSynced.Add(ctx, 1)
	RagSyncErrors.Add(ctx, 1)
}
