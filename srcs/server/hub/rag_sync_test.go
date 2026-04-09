package hub_test

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/hub"
)

type MockRAGSyncService struct {
	PendingRecords []hub.RAGSyncRecord
	SyncedIDs      []string
	IncomingRecords []hub.RAGSyncRecord
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]hub.RAGSyncRecord, error) {
	if limit > len(m.PendingRecords) {
		return m.PendingRecords, nil
	}
	return m.PendingRecords[:limit], nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	m.SyncedIDs = append(m.SyncedIDs, ids...)
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []hub.RAGSyncRecord) error {
	m.IncomingRecords = append(m.IncomingRecords, records...)
	return nil
}

func TestRAGSyncService_Mock(t *testing.T) {
	mockService := &MockRAGSyncService{
		PendingRecords: []hub.RAGSyncRecord{
			{
				ID:         "test-1",
				Context:    "test context 1",
				Vector:     []float32{0.1, 0.2},
				SyncStatus: hub.SyncStatusPending,
				LastSyncAt: time.Time{},
			},
		},
	}

	ctx := context.Background()

	// Test FetchPendingSyncs
	pending, err := mockService.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(pending) != 1 {
		t.Fatalf("expected 1 pending record, got %d", len(pending))
	}
	if pending[0].ID != "test-1" {
		t.Errorf("expected ID 'test-1', got '%s'", pending[0].ID)
	}

	// Test ProcessIncomingSync
	err = mockService.ProcessIncomingSync(ctx, pending)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(mockService.IncomingRecords) != 1 {
		t.Fatalf("expected 1 incoming record processed, got %d", len(mockService.IncomingRecords))
	}

	// Test MarkSynced
	err = mockService.MarkSynced(ctx, []string{pending[0].ID})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(mockService.SyncedIDs) != 1 || mockService.SyncedIDs[0] != "test-1" {
		t.Errorf("expected SyncedIDs to contain 'test-1', got %v", mockService.SyncedIDs)
	}
}

func TestMetricsInitialized(t *testing.T) {
	// Simple test to ensure the init() block runs and metrics are not nil.
	if hub.RecordsSyncedTotal == nil {
		t.Fatal("RecordsSyncedTotal metric was not initialized")
	}
	if hub.SyncErrorsTotal == nil {
		t.Fatal("SyncErrorsTotal metric was not initialized")
	}
}
