package hub

import (
	"context"
	"testing"
)

type MockRAGSyncService struct {
	PendingRecords []RAGSyncRecord
	MarkError      error
	ProcessError   error
	Processed      []RAGSyncRecord
	MarkedIDs      []string
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if limit > len(m.PendingRecords) {
		return m.PendingRecords, nil
	}
	return m.PendingRecords[:limit], nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if m.MarkError != nil {
		return m.MarkError
	}
	m.MarkedIDs = append(m.MarkedIDs, ids...)
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if m.ProcessError != nil {
		return m.ProcessError
	}
	m.Processed = append(m.Processed, records...)
	return nil
}

func TestMockRAGSyncService(t *testing.T) {
	ctx := context.Background()
	mockSvc := &MockRAGSyncService{
		PendingRecords: []RAGSyncRecord{
			{ID: "1", Context: "test context 1", SyncStatus: SyncStatusPending},
			{ID: "2", Context: "test context 2", SyncStatus: SyncStatusPending},
		},
	}

	records, err := mockSvc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(records) != 2 {
		t.Errorf("expected 2 records, got %d", len(records))
	}

	err = mockSvc.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(mockSvc.Processed) != 2 {
		t.Errorf("expected 2 processed records, got %d", len(mockSvc.Processed))
	}

	err = mockSvc.MarkSynced(ctx, []string{"1", "2"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(mockSvc.MarkedIDs) != 2 {
		t.Errorf("expected 2 marked records, got %d", len(mockSvc.MarkedIDs))
	}
}
