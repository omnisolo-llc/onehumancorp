package hub

import (
	"context"
	"testing"
	"time"
)

type MockRAGSyncService struct {
	PendingRecords []RAGSyncRecord
	MarkedSynced   []string
	IncomingSyncs  []RAGSyncRecord
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if len(m.PendingRecords) > limit {
		return m.PendingRecords[:limit], nil
	}
	return m.PendingRecords, nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	m.MarkedSynced = append(m.MarkedSynced, ids...)
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	m.IncomingSyncs = append(m.IncomingSyncs, records...)
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
		t.Errorf("expected 2 records, got %d", len(records))
	}

	// Test MarkSynced
	err = mockService.MarkSynced(ctx, []string{"1", "2"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(mockService.MarkedSynced) != 2 {
		t.Errorf("expected 2 marked synced, got %d", len(mockService.MarkedSynced))
	}

	// Test ProcessIncomingSync
	incoming := []RAGSyncRecord{
		{ID: "3", Context: "test 3", SyncStatus: SyncStatusPending, LastSyncAt: time.Now()},
	}
	err = mockService.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(mockService.IncomingSyncs) != 1 {
		t.Errorf("expected 1 incoming sync, got %d", len(mockService.IncomingSyncs))
	}
}
