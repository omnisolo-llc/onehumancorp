package hub

import (
	"context"
	"testing"

	_ "modernc.org/sqlite"
)

// MockRAGSyncService is a mock implementation for testing
type MockRAGSyncService struct {
	PendingRecords []RAGSyncRecord
	MarkedSynced   []string
	Processed      []RAGSyncRecord
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
	m.Processed = append(m.Processed, records...)
	return nil
}

func TestRAGSyncService_Interface(t *testing.T) {
	mockService := &MockRAGSyncService{
		PendingRecords: []RAGSyncRecord{
			{ID: "test-1", Context: "context", Vector: []float32{0.1, 0.2}, SyncStatus: SyncStatusPending},
		},
	}

	ctx := context.Background()
	records, err := mockService.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(records) != 1 {
		t.Errorf("expected 1 record, got %d", len(records))
	}

	err = mockService.MarkSynced(ctx, []string{"test-1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(mockService.MarkedSynced) != 1 || mockService.MarkedSynced[0] != "test-1" {
		t.Errorf("expected test-1 to be marked synced")
	}

	err = mockService.ProcessIncomingSync(ctx, []RAGSyncRecord{{ID: "test-2"}})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(mockService.Processed) != 1 || mockService.Processed[0].ID != "test-2" {
		t.Errorf("expected test-2 to be processed")
	}
}
