package hub

import (
	"context"
	"testing"
)

func TestRAGSyncServiceImpl(t *testing.T) {
	service := NewRAGSyncService()

	records, err := service.FetchPendingSyncs(context.Background(), 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(records) != 0 {
		t.Fatalf("expected 0 records, got %d", len(records))
	}

	err = service.MarkSynced(context.Background(), []string{"test"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	err = service.ProcessIncomingSync(context.Background(), []RAGSyncRecord{})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
}
