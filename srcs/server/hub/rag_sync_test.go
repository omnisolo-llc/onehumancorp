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

func TestRAGSyncService(t *testing.T) {
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
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 2 {
		t.Fatalf("Expected 2 pending records, got %d", len(pending))
	}

	// Test MarkSynced
	err = mockService.MarkSynced(ctx, []string{"1", "2"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}
	if len(mockService.MarkedSynced) != 2 {
		t.Fatalf("Expected 2 marked synced records, got %d", len(mockService.MarkedSynced))
	}

	// Test ProcessIncomingSync
	incoming := []RAGSyncRecord{
		{ID: "3", Context: "Incoming 1", SyncStatus: SyncStatusSynced, LastSyncAt: time.Now()},
	}
	err = mockService.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}
	if len(mockService.ProcessedSyncs) != 1 {
		t.Fatalf("Expected 1 processed sync record, got %d", len(mockService.ProcessedSyncs))
	}
	if mockService.ProcessedSyncs[0].ID != "3" {
		t.Fatalf("Expected ID 3, got %s", mockService.ProcessedSyncs[0].ID)
	}
}
