package hub

import (
	"context"
	"errors"
	"testing"
	"time"
)

type MockRAGSyncService struct {
	PendingRecords []RAGSyncRecord
	SyncedIDs      []string
	ProcessedRecords []RAGSyncRecord
	ProcessError   error
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if len(m.PendingRecords) > limit {
		return m.PendingRecords[:limit], nil
	}
	return m.PendingRecords, nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	m.SyncedIDs = append(m.SyncedIDs, ids...)
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if m.ProcessError != nil {
		return m.ProcessError
	}
	m.ProcessedRecords = append(m.ProcessedRecords, records...)
	return nil
}

func TestFetchPendingSyncs(t *testing.T) {
	mockService := &MockRAGSyncService{
		PendingRecords: []RAGSyncRecord{
			{ID: "1", Context: "test context 1", SyncStatus: SyncStatusPending, LastSyncAt: time.Now()},
			{ID: "2", Context: "test context 2", SyncStatus: SyncStatusPending, LastSyncAt: time.Now()},
		},
	}

	records, err := mockService.FetchPendingSyncs(context.Background(), 1)
	if err != nil {
		t.Fatalf("Expected no error, got: %v", err)
	}

	if len(records) != 1 {
		t.Errorf("Expected 1 record, got %d", len(records))
	}

	if records[0].ID != "1" {
		t.Errorf("Expected ID '1', got '%s'", records[0].ID)
	}
}

func TestMarkSynced(t *testing.T) {
	mockService := &MockRAGSyncService{}
	idsToSync := []string{"1", "2"}

	err := mockService.MarkSynced(context.Background(), idsToSync)
	if err != nil {
		t.Fatalf("Expected no error, got: %v", err)
	}

	if len(mockService.SyncedIDs) != 2 {
		t.Errorf("Expected 2 synced IDs, got %d", len(mockService.SyncedIDs))
	}
}

func TestProcessIncomingSync(t *testing.T) {
	t.Run("Success", func(t *testing.T) {
		mockService := &MockRAGSyncService{}
		recordsToProcess := []RAGSyncRecord{
			{ID: "1", Context: "test context 1"},
		}

		err := mockService.ProcessIncomingSync(context.Background(), recordsToProcess)
		if err != nil {
			t.Fatalf("Expected no error, got: %v", err)
		}

		if len(mockService.ProcessedRecords) != 1 {
			t.Errorf("Expected 1 processed record, got %d", len(mockService.ProcessedRecords))
		}
	})

	t.Run("Error", func(t *testing.T) {
		expectedErr := errors.New("processing error")
		mockService := &MockRAGSyncService{
			ProcessError: expectedErr,
		}
		recordsToProcess := []RAGSyncRecord{
			{ID: "1", Context: "test context 1"},
		}

		err := mockService.ProcessIncomingSync(context.Background(), recordsToProcess)
		if err == nil {
			t.Fatal("Expected error, got nil")
		}
		if err.Error() != expectedErr.Error() {
			t.Errorf("Expected error '%v', got '%v'", expectedErr, err)
		}
	})
}
