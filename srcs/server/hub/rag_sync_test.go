package hub

import (
	"context"
	"testing"
	"time"
)

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
	if limit < len(m.PendingRecords) {
		return m.PendingRecords[:limit], nil
	}
	return m.PendingRecords, nil
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

func TestMockRAGSyncService(t *testing.T) {
	mock := &MockRAGSyncService{
		PendingRecords: []RAGSyncRecord{
			{ID: "1", Context: "test1", SyncStatus: SyncStatusPending},
			{ID: "2", Context: "test2", SyncStatus: SyncStatusPending},
		},
	}

	ctx := context.Background()

	// Test FetchPendingSyncs
	records, err := mock.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(records) != 2 {
		t.Errorf("expected 2 records, got %d", len(records))
	}

	// Test MarkSynced
	err = mock.MarkSynced(ctx, []string{"1", "2"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(mock.MarkedIDs) != 2 {
		t.Errorf("expected 2 marked IDs, got %d", len(mock.MarkedIDs))
	}

	// Test ProcessIncomingSync
	incoming := []RAGSyncRecord{
		{ID: "3", Context: "test3", SyncStatus: SyncStatusSynced, LastSyncAt: time.Now()},
	}
	err = mock.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(mock.ProcessedData) != 1 {
		t.Errorf("expected 1 processed record, got %d", len(mock.ProcessedData))
	}
}
