package hub_test

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/hub"
)

// MockRAGSyncService is a mock implementation for testing.
type MockRAGSyncService struct {
	PendingRecords []hub.RAGSyncRecord
	SyncedIDs      []string
	ProcessedRecs  []hub.RAGSyncRecord
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]hub.RAGSyncRecord, error) {
	if limit > len(m.PendingRecords) {
		return m.PendingRecords, nil
	}
	return m.PendingRecords[:limit], nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	m.SyncedIDs = append(m.SyncedIDs, ids...)
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []hub.RAGSyncRecord) error {
	m.ProcessedRecs = append(m.ProcessedRecs, records...)
	return nil
}

func TestRAGSyncServiceMock(t *testing.T) {
	ctx := context.Background()
	mockService := &MockRAGSyncService{
		PendingRecords: []hub.RAGSyncRecord{
			{ID: "1", Context: "Memory 1", SyncStatus: hub.SyncStatusPending},
			{ID: "2", Context: "Memory 2", SyncStatus: hub.SyncStatusPending},
		},
	}

	// Test FetchPendingSyncs
	pending, err := mockService.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(pending) != 2 {
		t.Fatalf("expected 2 pending records, got %d", len(pending))
	}

	// Test ProcessIncomingSync
	err = mockService.ProcessIncomingSync(ctx, pending)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(mockService.ProcessedRecs) != 2 {
		t.Fatalf("expected 2 processed records, got %d", len(mockService.ProcessedRecs))
	}

	// Test MarkSynced
	ids := []string{"1", "2"}
	err = mockService.MarkSynced(ctx, ids)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(mockService.SyncedIDs) != 2 {
		t.Fatalf("expected 2 synced IDs, got %d", len(mockService.SyncedIDs))
	}
}
