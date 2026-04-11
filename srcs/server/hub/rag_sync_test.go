package hub

import (
	"context"
	"testing"
)

type MockRAGSyncService struct {
	PendingRecords []RAGSyncRecord
	MarkedIDs      []string
	ProcessedData  []RAGSyncRecord
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if limit > len(m.PendingRecords) {
		limit = len(m.PendingRecords)
	}
	return m.PendingRecords[:limit], nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	m.MarkedIDs = append(m.MarkedIDs, ids...)
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	m.ProcessedData = append(m.ProcessedData, records...)
	return nil
}

func TestRAGSyncServiceInterface(t *testing.T) {
	mockService := &MockRAGSyncService{
		PendingRecords: []RAGSyncRecord{
			{ID: "record-1", Context: "test context", Vector: []byte{1, 2, 3}, SyncStatus: SyncStatusPending},
		},
	}

	ctx := context.Background()

	// Test FetchPendingSyncs
	pending, err := mockService.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 1 {
		t.Errorf("Expected 1 pending record, got %d", len(pending))
	}

	// Test ProcessIncomingSync
	err = mockService.ProcessIncomingSync(ctx, pending)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}
	if len(mockService.ProcessedData) != 1 {
		t.Errorf("Expected 1 processed record, got %d", len(mockService.ProcessedData))
	}

	// Test MarkSynced
	idsToMark := []string{pending[0].ID}
	err = mockService.MarkSynced(ctx, idsToMark)
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}
	if len(mockService.MarkedIDs) != 1 || mockService.MarkedIDs[0] != "record-1" {
		t.Errorf("Expected record-1 to be marked as synced, got %v", mockService.MarkedIDs)
	}
}
