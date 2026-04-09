package hub

import (
	"context"

	"testing"
)

type MockRAGSyncService struct {
	PendingRecords   []RAGSyncRecord
	SyncedIDs        []string
	ProcessedRecords []RAGSyncRecord
	FetchErr         error
	MarkErr          error
	ProcessErr       error
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if m.FetchErr != nil {
		return nil, m.FetchErr
	}
	if limit < len(m.PendingRecords) {
		return m.PendingRecords[:limit], nil
	}
	return m.PendingRecords, nil
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
	m.ProcessedRecords = append(m.ProcessedRecords, records...)
	return nil
}

func TestFetchPendingSyncs(t *testing.T) {
	mockService := &MockRAGSyncService{
		PendingRecords: []RAGSyncRecord{
			{ID: "1", Context: "test 1", SyncStatus: SyncStatusPending},
			{ID: "2", Context: "test 2", SyncStatus: SyncStatusPending},
		},
	}

	ctx := context.Background()
	records, err := mockService.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(records) != 2 {
		t.Fatalf("expected 2 records, got %d", len(records))
	}
}

func TestMarkSynced(t *testing.T) {
	mockService := &MockRAGSyncService{}

	ctx := context.Background()
	err := mockService.MarkSynced(ctx, []string{"1", "2"})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(mockService.SyncedIDs) != 2 {
		t.Fatalf("expected 2 synced IDs, got %d", len(mockService.SyncedIDs))
	}
}

func TestProcessIncomingSync(t *testing.T) {
	mockService := &MockRAGSyncService{}

	records := []RAGSyncRecord{
		{ID: "1", Context: "test 1", SyncStatus: SyncStatusPending},
	}

	ctx := context.Background()
	err := mockService.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(mockService.ProcessedRecords) != 1 {
		t.Fatalf("expected 1 processed record, got %d", len(mockService.ProcessedRecords))
	}
}
