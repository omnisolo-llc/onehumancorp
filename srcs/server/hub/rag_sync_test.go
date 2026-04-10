package hub

import (
	"context"
	"testing"
	"time"
)

// MockRAGSyncService is a mock implementation of RAGSyncService for testing
type MockRAGSyncService struct {
	PendingRecords []RAGSyncRecord
	SyncedIDs      []string
	IncomingRecords []RAGSyncRecord
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if limit > len(m.PendingRecords) {
		return m.PendingRecords, nil
	}
	return m.PendingRecords[:limit], nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	m.SyncedIDs = append(m.SyncedIDs, ids...)
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	m.IncomingRecords = append(m.IncomingRecords, records...)
	return nil
}

func TestRAGSyncServiceMock(t *testing.T) {
	mockService := &MockRAGSyncService{
		PendingRecords: []RAGSyncRecord{
			{ID: "mem1", Context: "context 1", SyncStatus: SyncStatusPending},
			{ID: "mem2", Context: "context 2", SyncStatus: SyncStatusPending},
		},
	}

	ctx := context.Background()

	// Test FetchPendingSyncs
	pending, err := mockService.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(pending) != 2 {
		t.Errorf("expected 2 pending records, got %d", len(pending))
	}

	// Test MarkSynced
	idsToSync := []string{"mem1", "mem2"}
	err = mockService.MarkSynced(ctx, idsToSync)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(mockService.SyncedIDs) != 2 {
		t.Errorf("expected 2 synced IDs, got %d", len(mockService.SyncedIDs))
	}

	// Test ProcessIncomingSync
	incoming := []RAGSyncRecord{
		{ID: "mem3", Context: "context 3", SyncStatus: SyncStatusSynced, LastSyncAt: time.Now()},
	}
	err = mockService.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(mockService.IncomingRecords) != 1 {
		t.Errorf("expected 1 incoming record, got %d", len(mockService.IncomingRecords))
	}
}
