package hub

import (
	"context"
	"testing"
	"time"
)

type MockRAGSyncService struct {
	recordsToReturn []RAGSyncRecord
	markedIDs       []string
	processed       []RAGSyncRecord
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if limit < len(m.recordsToReturn) {
		return m.recordsToReturn[:limit], nil
	}
	return m.recordsToReturn, nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	m.markedIDs = append(m.markedIDs, ids...)
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	m.processed = append(m.processed, records...)
	return nil
}

func TestFetchPendingSyncs(t *testing.T) {
	ctx := context.Background()
	mockService := &MockRAGSyncService{
		recordsToReturn: []RAGSyncRecord{
			{ID: "1", Context: "test1", SyncStatus: SyncStatusPending},
			{ID: "2", Context: "test2", SyncStatus: SyncStatusPending},
		},
	}

	records, err := mockService.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(records) != 2 {
		t.Errorf("expected 2 records, got %d", len(records))
	}
	if records[0].ID != "1" {
		t.Errorf("expected ID '1', got '%s'", records[0].ID)
	}
}

func TestMarkSynced(t *testing.T) {
	ctx := context.Background()
	mockService := &MockRAGSyncService{}

	err := mockService.MarkSynced(ctx, []string{"1", "2"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(mockService.markedIDs) != 2 {
		t.Errorf("expected 2 marked IDs, got %d", len(mockService.markedIDs))
	}
	if mockService.markedIDs[0] != "1" || mockService.markedIDs[1] != "2" {
		t.Errorf("unexpected marked IDs: %v", mockService.markedIDs)
	}
}

func TestProcessIncomingSync(t *testing.T) {
	ctx := context.Background()
	mockService := &MockRAGSyncService{}
	records := []RAGSyncRecord{
		{ID: "1", Context: "test1", SyncStatus: SyncStatusSynced, LastSyncAt: time.Now()},
	}

	err := mockService.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(mockService.processed) != 1 {
		t.Errorf("expected 1 processed record, got %d", len(mockService.processed))
	}
	if mockService.processed[0].ID != "1" {
		t.Errorf("expected ID '1', got '%s'", mockService.processed[0].ID)
	}
}
