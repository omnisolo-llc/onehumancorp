package hub

import (
	"context"
	"testing"
	"time"
)

type MockRAGSyncService struct {
	PendingRecords []RAGSyncRecord
	MarkedSynced   []string
	Incoming       []RAGSyncRecord
	ErrFetch       error
	ErrMark        error
	ErrProcess     error
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if m.ErrFetch != nil {
		return nil, m.ErrFetch
	}
	if limit > len(m.PendingRecords) {
		limit = len(m.PendingRecords)
	}
	return m.PendingRecords[:limit], nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if m.ErrMark != nil {
		return m.ErrMark
	}
	m.MarkedSynced = append(m.MarkedSynced, ids...)
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if m.ErrProcess != nil {
		return m.ErrProcess
	}
	m.Incoming = append(m.Incoming, records...)
	return nil
}

func TestRAGSyncService(t *testing.T) {
	mockService := &MockRAGSyncService{
		PendingRecords: []RAGSyncRecord{
			{ID: "1", Context: "test 1", SyncStatus: SyncStatusPending},
			{ID: "2", Context: "test 2", SyncStatus: SyncStatusPending},
		},
	}

	ctx := context.Background()

	// Test FetchPendingSyncs
	records, err := mockService.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(records) != 2 {
		t.Fatalf("expected 2 records, got %d", len(records))
	}

	// Test MarkSynced
	ids := []string{"1", "2"}
	err = mockService.MarkSynced(ctx, ids)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(mockService.MarkedSynced) != 2 {
		t.Fatalf("expected 2 marked synced, got %d", len(mockService.MarkedSynced))
	}

	// Test ProcessIncomingSync
	incoming := []RAGSyncRecord{
		{ID: "3", Context: "test 3", SyncStatus: SyncStatusPending, LastSyncAt: time.Now()},
	}
	err = mockService.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(mockService.Incoming) != 1 {
		t.Fatalf("expected 1 incoming, got %d", len(mockService.Incoming))
	}
}
