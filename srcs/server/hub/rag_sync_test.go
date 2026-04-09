package hub

import (
	"context"
	"testing"
	"time"
)

type MockRAGSyncService struct {
	PendingRecords []RAGSyncRecord
	MarkedSynced   []string
	Processed      []RAGSyncRecord
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if limit < len(m.PendingRecords) {
		return m.PendingRecords[:limit], nil
	}
	return m.PendingRecords, nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	m.MarkedSynced = append(m.MarkedSynced, ids...)
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	m.Processed = append(m.Processed, records...)
	return nil
}

func TestRAGSyncService(t *testing.T) {
	svc := &MockRAGSyncService{
		PendingRecords: []RAGSyncRecord{
			{
				ID:         "test-id-1",
				Context:    "Test context",
				SyncStatus: SyncStatusPending,
				LastSyncAt: time.Now(),
			},
		},
	}

	ctx := context.Background()

	// Test FetchPendingSyncs
	records, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(records) != 1 {
		t.Errorf("Expected 1 record, got %d", len(records))
	}

	// Test MarkSynced
	err = svc.MarkSynced(ctx, []string{"test-id-1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}
	if len(svc.MarkedSynced) != 1 || svc.MarkedSynced[0] != "test-id-1" {
		t.Errorf("Expected test-id-1 to be marked synced")
	}

	// Test ProcessIncomingSync
	incoming := []RAGSyncRecord{
		{
			ID:         "test-id-2",
			Context:    "Incoming context",
			SyncStatus: SyncStatusSynced,
			LastSyncAt: time.Now(),
		},
	}
	err = svc.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}
	if len(svc.Processed) != 1 || svc.Processed[0].ID != "test-id-2" {
		t.Errorf("Expected test-id-2 to be processed")
	}
}
