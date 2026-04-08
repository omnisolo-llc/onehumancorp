package hub

import (
	"context"
	"errors"
	"testing"
)

// MockRAGSyncService is a mock implementation of RAGSyncService for testing
type MockRAGSyncService struct {
	PendingRecords []RAGSyncRecord
	MarkedSynced   []string
	ProcessedSyncs []RAGSyncRecord
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
	m.MarkedSynced = append(m.MarkedSynced, ids...)
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if m.ProcessErr != nil {
		return m.ProcessErr
	}
	m.ProcessedSyncs = append(m.ProcessedSyncs, records...)
	return nil
}

func TestRAGSyncInterfaceAndMetrics(t *testing.T) {
	mockService := &MockRAGSyncService{
		PendingRecords: []RAGSyncRecord{
			{ID: "rec1", Context: "test 1", SyncStatus: SyncStatusPending},
			{ID: "rec2", Context: "test 2", SyncStatus: SyncStatusPending},
		},
	}

	ctx := context.Background()

	// Test FetchPendingSyncs
	records, err := mockService.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(records) != 2 {
		t.Fatalf("expected 2 records, got %d", len(records))
	}

	// Test MarkSynced and metric update
	err = mockService.MarkSynced(ctx, []string{"rec1", "rec2"})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(mockService.MarkedSynced) != 2 {
		t.Fatalf("expected 2 marked records, got %d", len(mockService.MarkedSynced))
	}

	// Trigger success metrics
	RecordSyncSuccess(ctx, 2)

	// Test error scenarios and metrics
	mockService.ProcessErr = errors.New("simulated processing error")
	err = mockService.ProcessIncomingSync(ctx, records)
	if err == nil {
		t.Fatalf("expected error, got nil")
	}

	// Trigger error metric
	RecordSyncError(ctx)
}
