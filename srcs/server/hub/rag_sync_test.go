package hub

import (
	"context"
	"errors"
	"testing"
	"time"
)

// MockRAGSyncService is a simple mock implementation of RAGSyncService for testing
type MockRAGSyncService struct {
	PendingRecords []RAGSyncRecord
	SyncedIDs      []string
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
		limit = len(m.PendingRecords)
	}
	return m.PendingRecords[:limit], nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if m.MarkErr != nil {
		return m.MarkErr
	}
	m.SyncedIDs = append(m.SyncedIDs, ids...)
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
	mock := &MockRAGSyncService{
		PendingRecords: []RAGSyncRecord{
			{ID: "1", SyncStatus: SyncStatusPending},
			{ID: "2", SyncStatus: SyncStatusPending},
		},
	}

	records, err := mock.FetchPendingSyncs(context.Background(), 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(records) != 2 {
		t.Errorf("expected 2 records, got %d", len(records))
	}
}

func TestRAGSyncService_MarkSynced(t *testing.T) {
	mock := &MockRAGSyncService{}

	ids := []string{"1", "2"}
	err := mock.MarkSynced(context.Background(), ids)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(mock.SyncedIDs) != 2 {
		t.Errorf("expected 2 synced IDs, got %d", len(mock.SyncedIDs))
	}
}

func TestRAGSyncService_ProcessIncomingSync(t *testing.T) {
	mock := &MockRAGSyncService{}

	records := []RAGSyncRecord{
		{ID: "1", Context: "test data", LastSyncAt: time.Now()},
	}

	err := mock.ProcessIncomingSync(context.Background(), records)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(mock.ProcessedData) != 1 {
		t.Errorf("expected 1 processed record, got %d", len(mock.ProcessedData))
	}
}

func TestRAGSyncService_Errors(t *testing.T) {
	expectedErr := errors.New("test error")
	mock := &MockRAGSyncService{
		FetchErr:   expectedErr,
		MarkErr:    expectedErr,
		ProcessErr: expectedErr,
	}

	_, err := mock.FetchPendingSyncs(context.Background(), 10)
	if !errors.Is(err, expectedErr) {
		t.Errorf("expected %v, got %v", expectedErr, err)
	}

	err = mock.MarkSynced(context.Background(), []string{"1"})
	if !errors.Is(err, expectedErr) {
		t.Errorf("expected %v, got %v", expectedErr, err)
	}

	err = mock.ProcessIncomingSync(context.Background(), []RAGSyncRecord{{ID: "1"}})
	if !errors.Is(err, expectedErr) {
		t.Errorf("expected %v, got %v", expectedErr, err)
	}
}
