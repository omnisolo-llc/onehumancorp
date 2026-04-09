package hub

import (
	"context"
	"testing"
	"time"
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

func TestRAGSyncServiceFlow(t *testing.T) {
	mockService := &MockRAGSyncService{
		PendingRecords: []RAGSyncRecord{
			{
				ID:         "mem-1",
				Context:    "User requested privacy settings",
				Vector:     []float32{0.1, 0.2, 0.3},
				SyncStatus: SyncStatusPending,
				LastSyncAt: time.Time{},
			},
		},
	}

	ctx := context.Background()

	// 1. Fetch pending syncs
	pending, err := mockService.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 1 {
		t.Fatalf("Expected 1 pending record, got %d", len(pending))
	}

	// 2. Process incoming sync (Cloud side simulation)
	err = mockService.ProcessIncomingSync(ctx, pending)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}
	if len(mockService.ProcessedSyncs) != 1 {
		t.Fatalf("Expected 1 processed sync, got %d", len(mockService.ProcessedSyncs))
	}

	// 3. Mark synced (Local side simulation)
	idsToMark := []string{pending[0].ID}
	err = mockService.MarkSynced(ctx, idsToMark)
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}
	if len(mockService.MarkedSynced) != 1 {
		t.Fatalf("Expected 1 marked synced ID, got %d", len(mockService.MarkedSynced))
	}
	if mockService.MarkedSynced[0] != "mem-1" {
		t.Errorf("Expected marked ID to be 'mem-1', got '%s'", mockService.MarkedSynced[0])
	}
}
