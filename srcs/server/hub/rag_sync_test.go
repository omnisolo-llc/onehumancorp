package hub

import (
	"context"
	"errors"
	"testing"
	"time"
)

// MockRAGSyncService is a simple mock for testing the RAGSyncService interface
type MockRAGSyncService struct {
	PendingRecords []RAGSyncRecord
	MarkedIDs      []string
	ProcessedData  []RAGSyncRecord
	FetchErr       error
	MarkErr        error
	ProcessErr     error
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if m.FetchErr != nil {
		return nil, m.FetchErr
	}
	if limit > len(m.PendingRecords) {
		return m.PendingRecords, nil
	}
	return m.PendingRecords[:limit], nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if m.MarkErr != nil {
		return m.MarkErr
	}
	m.MarkedIDs = append(m.MarkedIDs, ids...)
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if m.ProcessErr != nil {
		return m.ProcessErr
	}
	m.ProcessedData = append(m.ProcessedData, records...)
	return nil
}

func TestRAGSyncService_FetchPendingSyncs(t *testing.T) {
	mockService := &MockRAGSyncService{
		PendingRecords: []RAGSyncRecord{
			{ID: "1", Context: "test context 1", SyncStatus: SyncStatusPending},
			{ID: "2", Context: "test context 2", SyncStatus: SyncStatusPending},
		},
	}

	ctx := context.Background()
	records, err := mockService.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if len(records) != 2 {
		t.Errorf("expected 2 records, got %d", len(records))
	}
}

func TestRAGSyncService_MarkSynced(t *testing.T) {
	mockService := &MockRAGSyncService{}

	ctx := context.Background()
	idsToMark := []string{"1", "2"}
	err := mockService.MarkSynced(ctx, idsToMark)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if len(mockService.MarkedIDs) != 2 {
		t.Errorf("expected 2 marked IDs, got %d", len(mockService.MarkedIDs))
	}
}

func TestRAGSyncService_ProcessIncomingSync(t *testing.T) {
	mockService := &MockRAGSyncService{}

	ctx := context.Background()
	incomingRecords := []RAGSyncRecord{
		{ID: "3", Context: "incoming context", SyncStatus: SyncStatusSynced, LastSyncAt: time.Now()},
	}

	err := mockService.ProcessIncomingSync(ctx, incomingRecords)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if len(mockService.ProcessedData) != 1 {
		t.Errorf("expected 1 processed record, got %d", len(mockService.ProcessedData))
	}
}

func TestRAGSyncService_Errors(t *testing.T) {
	mockService := &MockRAGSyncService{
		FetchErr:   errors.New("fetch failed"),
		MarkErr:    errors.New("mark failed"),
		ProcessErr: errors.New("process failed"),
	}

	ctx := context.Background()

	_, err := mockService.FetchPendingSyncs(ctx, 10)
	if err == nil || err.Error() != "fetch failed" {
		t.Errorf("expected 'fetch failed' error")
	}

	err = mockService.MarkSynced(ctx, []string{"1"})
	if err == nil || err.Error() != "mark failed" {
		t.Errorf("expected 'mark failed' error")
	}

	err = mockService.ProcessIncomingSync(ctx, []RAGSyncRecord{{}})
	if err == nil || err.Error() != "process failed" {
		t.Errorf("expected 'process failed' error")
	}
}
