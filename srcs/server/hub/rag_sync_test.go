package hub

import (
	"context"
	"testing"
	"time"

	"go.opentelemetry.io/otel/metric/noop"
)

// MockRAGSyncService is a simple mock for testing the basic logic flows.
type MockRAGSyncService struct {
	records []RAGSyncRecord
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	var pending []RAGSyncRecord
	for _, r := range m.records {
		if r.SyncStatus == SyncStatusPending {
			pending = append(pending, r)
		}
	}
	if len(pending) > limit {
		return pending[:limit], nil
	}
	return pending, nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	for _, id := range ids {
		for i, r := range m.records {
			if r.ID == id {
				m.records[i].SyncStatus = SyncStatusSynced
				now := time.Now(); m.records[i].LastSyncAt = &now
			}
		}
	}
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	m.records = append(m.records, records...)
	return nil
}

func TestRAGSyncService_Flow(t *testing.T) {
	// Initialize metrics with noop for test safety
	meter := noop.NewMeterProvider().Meter("test")
	InitRAGSyncMetrics(meter)

	ctx := context.Background()
	mockService := &MockRAGSyncService{
		records: []RAGSyncRecord{
			{ID: "r1", Context: "test 1", Vector: []float32{0.1, 0.2}, SyncStatus: SyncStatusPending},
			{ID: "r2", Context: "test 2", Vector: []float32{0.3, 0.4}, SyncStatus: SyncStatusSynced},
			{ID: "r3", Context: "test 3", Vector: []float32{0.5, 0.6}, SyncStatus: SyncStatusPending},
		},
	}

	// Test FetchPendingSyncs
	pending, err := mockService.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(pending) != 2 {
		t.Errorf("expected 2 pending records, got %d", len(pending))
	}

	// Test MarkSynced
	idsToSync := []string{"r1"}
	err = mockService.MarkSynced(ctx, idsToSync)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	// Verify MarkSynced worked correctly
	pendingAfterSync, _ := mockService.FetchPendingSyncs(ctx, 10)
	if len(pendingAfterSync) != 1 {
		t.Errorf("expected 1 pending record, got %d", len(pendingAfterSync))
	}

	// Test metrics wrapper
	RecordRAGRecordsSynced(ctx, 1)
	RecordRAGSyncError(ctx)

	// Test ProcessIncomingSync
	newRecs := []RAGSyncRecord{
		{ID: "r4", Context: "test 4", Vector: []float32{0.7, 0.8}, SyncStatus: SyncStatusSynced},
	}
	err = mockService.ProcessIncomingSync(ctx, newRecs)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(mockService.records) != 4 {
		t.Errorf("expected 4 total records, got %d", len(mockService.records))
	}
}
