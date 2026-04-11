package hub

import (
	"context"
	"testing"
)

type MockRAGSyncService struct {
	PendingRecords []RAGSyncRecord
	MarkedSynced   []string
	ProcessedRecords []RAGSyncRecord
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if limit > len(m.PendingRecords) {
		return m.PendingRecords, nil
	}
	return m.PendingRecords[:limit], nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	m.MarkedSynced = append(m.MarkedSynced, ids...)
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	m.ProcessedRecords = append(m.ProcessedRecords, records...)
	return nil
}

func TestRAGSyncService_FetchPendingSyncs(t *testing.T) {
	mockService := &MockRAGSyncService{
		PendingRecords: []RAGSyncRecord{
			{ID: "1", Context: "Test Context 1", SyncStatus: SyncStatusPending},
			{ID: "2", Context: "Test Context 2", SyncStatus: SyncStatusPending},
		},
	}

	records, err := mockService.FetchPendingSyncs(context.Background(), 2)
	if err != nil {
		t.Fatalf("FetchPendingSyncs returned error: %v", err)
	}

	if len(records) != 2 {
		t.Errorf("Expected 2 records, got %d", len(records))
	}
}

func TestRAGSyncService_MarkSynced(t *testing.T) {
	mockService := &MockRAGSyncService{}
	ids := []string{"1", "2"}

	err := mockService.MarkSynced(context.Background(), ids)
	if err != nil {
		t.Fatalf("MarkSynced returned error: %v", err)
	}

	if len(mockService.MarkedSynced) != 2 {
		t.Errorf("Expected 2 marked records, got %d", len(mockService.MarkedSynced))
	}
}

func TestRAGSyncService_ProcessIncomingSync(t *testing.T) {
	mockService := &MockRAGSyncService{}
	records := []RAGSyncRecord{
		{ID: "1", Context: "Test Context 1", SyncStatus: SyncStatusPending},
	}

	err := mockService.ProcessIncomingSync(context.Background(), records)
	if err != nil {
		t.Fatalf("ProcessIncomingSync returned error: %v", err)
	}

	if len(mockService.ProcessedRecords) != 1 {
		t.Errorf("Expected 1 processed record, got %d", len(mockService.ProcessedRecords))
	}
}
