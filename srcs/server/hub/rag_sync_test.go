package hub

import (
	"context"
	"testing"
)

type MockRAGSyncService struct {
	PendingRecords []RAGSyncRecord
	MarkedSynced   []string
	ProcessedSyncs []RAGSyncRecord
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if limit > len(m.PendingRecords) {
		limit = len(m.PendingRecords)
	}
	return m.PendingRecords[:limit], nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	m.MarkedSynced = append(m.MarkedSynced, ids...)
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	m.ProcessedSyncs = append(m.ProcessedSyncs, records...)
	return nil
}

func TestRAGSyncService_Mock(t *testing.T) {
	mockService := &MockRAGSyncService{
		PendingRecords: []RAGSyncRecord{
			{ID: "1", Context: "Test Context 1", SyncStatus: SyncStatusPending},
			{ID: "2", Context: "Test Context 2", SyncStatus: SyncStatusPending},
		},
	}

	ctx := context.Background()

	// Test FetchPendingSyncs
	pending, err := mockService.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("Failed to fetch pending syncs: %v", err)
	}
	if len(pending) != 2 {
		t.Fatalf("Expected 2 pending records, got %d", len(pending))
	}

	// Test ProcessIncomingSync
	err = mockService.ProcessIncomingSync(ctx, pending)
	if err != nil {
		t.Fatalf("Failed to process incoming sync: %v", err)
	}
	if len(mockService.ProcessedSyncs) != 2 {
		t.Fatalf("Expected 2 processed records, got %d", len(mockService.ProcessedSyncs))
	}

	// Test MarkSynced
	ids := []string{pending[0].ID, pending[1].ID}
	err = mockService.MarkSynced(ctx, ids)
	if err != nil {
		t.Fatalf("Failed to mark synced: %v", err)
	}
	if len(mockService.MarkedSynced) != 2 {
		t.Fatalf("Expected 2 marked records, got %d", len(mockService.MarkedSynced))
	}

	// Test Metrics
	RecordSyncSuccess(ctx, 2)
	RecordSyncError(ctx)
}
