package hub

import (
	"context"
	"testing"
	"time"
)

// MockRAGSyncService is a mock implementation of RAGSyncService
type MockRAGSyncService struct {
	PendingRecords []RAGSyncRecord
	MarkedIDs      []string
	ProcessedData  []RAGSyncRecord
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if limit < len(m.PendingRecords) {
		return m.PendingRecords[:limit], nil
	}
	return m.PendingRecords, nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	m.MarkedIDs = append(m.MarkedIDs, ids...)
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	m.ProcessedData = append(m.ProcessedData, records...)
	return nil
}

func TestRAGSyncService(t *testing.T) {
	mockService := &MockRAGSyncService{
		PendingRecords: []RAGSyncRecord{
			{ID: "1", Context: "test1", SyncStatus: SyncStatusPending},
			{ID: "2", Context: "test2", SyncStatus: SyncStatusPending},
		},
	}

	ctx := context.Background()

	// Test FetchPendingSyncs
	records, err := mockService.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(records) != 2 {
		t.Errorf("expected 2 records, got %d", len(records))
	}

	// Test MarkSynced
	err = mockService.MarkSynced(ctx, []string{"1", "2"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(mockService.MarkedIDs) != 2 {
		t.Errorf("expected 2 marked IDs, got %d", len(mockService.MarkedIDs))
	}

	// Test ProcessIncomingSync
	incomingRecords := []RAGSyncRecord{
		{ID: "3", Context: "test3", SyncStatus: SyncStatusSynced, LastSyncAt: time.Now()},
	}
	err = mockService.ProcessIncomingSync(ctx, incomingRecords)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(mockService.ProcessedData) != 1 {
		t.Errorf("expected 1 processed record, got %d", len(mockService.ProcessedData))
	}
}
