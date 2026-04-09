package hub

import (
	"context"
	"testing"
	"time"
)

type MockRAGSyncService struct {
	PendingRecords []RAGSyncRecord
	MarkedSynced   []string
	ProcessedIn    []RAGSyncRecord
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if limit > len(m.PendingRecords) {
		return m.PendingRecords, nil
	}
	return m.PendingRecords[:limit], nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	m.MarkedSynced = append(m.MarkedSynced, ids...)
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	m.ProcessedIn = append(m.ProcessedIn, records...)
	return nil
}

func TestRAGSyncService(t *testing.T) {
	mockService := &MockRAGSyncService{
		PendingRecords: []RAGSyncRecord{
			{ID: "test-1", Context: "Test Context 1", SyncStatus: SyncStatusPending, LastSyncAt: time.Now()},
			{ID: "test-2", Context: "Test Context 2", SyncStatus: SyncStatusPending, LastSyncAt: time.Now()},
		},
	}

	var service RAGSyncService = mockService
	ctx := context.Background()

	// Test FetchPendingSyncs
	pending, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 2 {
		t.Errorf("expected 2 pending records, got %d", len(pending))
	}

	// Test MarkSynced
	err = service.MarkSynced(ctx, []string{"test-1", "test-2"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}
	if len(mockService.MarkedSynced) != 2 {
		t.Errorf("expected 2 marked records, got %d", len(mockService.MarkedSynced))
	}

	// Test ProcessIncomingSync
	incoming := []RAGSyncRecord{
		{ID: "test-3", Context: "Test Context 3", SyncStatus: SyncStatusSynced, LastSyncAt: time.Now()},
	}
	err = service.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}
	if len(mockService.ProcessedIn) != 1 {
		t.Errorf("expected 1 processed record, got %d", len(mockService.ProcessedIn))
	}
}
