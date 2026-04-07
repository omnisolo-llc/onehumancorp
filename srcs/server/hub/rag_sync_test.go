package hub

import (
	"context"
	"testing"
	"time"

	"go.opentelemetry.io/otel/metric/noop"
)

type MockRAGSyncService struct {
	PendingSyncs []RAGSyncRecord
	MarkedSynced []string
	IncomingSync []RAGSyncRecord
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if limit > len(m.PendingSyncs) {
		limit = len(m.PendingSyncs)
	}
	return m.PendingSyncs[:limit], nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	m.MarkedSynced = append(m.MarkedSynced, ids...)
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	m.IncomingSync = append(m.IncomingSync, records...)
	return nil
}

func TestRAGSyncServiceInterface(t *testing.T) {
	mockService := &MockRAGSyncService{
		PendingSyncs: []RAGSyncRecord{
			{
				ID:         "1",
				Context:    "Test context",
				Vector:     []float32{0.1, 0.2, 0.3},
				SyncStatus: SyncStatusPending,
				LastSyncAt: time.Now(),
			},
		},
	}

	ctx := context.Background()

	// Test FetchPendingSyncs
	pending, err := mockService.FetchPendingSyncs(ctx, 1)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 1 {
		t.Fatalf("Expected 1 pending sync, got %d", len(pending))
	}
	if pending[0].ID != "1" {
		t.Errorf("Expected ID 1, got %s", pending[0].ID)
	}

	// Test MarkSynced
	err = mockService.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}
	if len(mockService.MarkedSynced) != 1 {
		t.Fatalf("Expected 1 marked synced, got %d", len(mockService.MarkedSynced))
	}

	// Test ProcessIncomingSync
	err = mockService.ProcessIncomingSync(ctx, []RAGSyncRecord{
		{
			ID:         "2",
			Context:    "Incoming context",
			Vector:     []float32{0.4, 0.5, 0.6},
			SyncStatus: SyncStatusSynced,
			LastSyncAt: time.Now(),
		},
	})
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}
	if len(mockService.IncomingSync) != 1 {
		t.Fatalf("Expected 1 incoming sync, got %d", len(mockService.IncomingSync))
	}
}

func TestInitRAGSyncMetrics(t *testing.T) {
	provider := noop.NewMeterProvider()
	meter := provider.Meter("test_meter")

	err := InitRAGSyncMetrics(meter)
	if err != nil {
		t.Fatalf("InitRAGSyncMetrics failed: %v", err)
	}

	if RAGRecordsSyncedTotal == nil {
		t.Error("RAGRecordsSyncedTotal is nil")
	}
	if RAGSyncErrorsTotal == nil {
		t.Error("RAGSyncErrorsTotal is nil")
	}

	// Ensure we can use the metrics without panicking
	ctx := context.Background()
	RAGRecordsSyncedTotal.Add(ctx, 1)
	RAGSyncErrorsTotal.Add(ctx, 1)
}
