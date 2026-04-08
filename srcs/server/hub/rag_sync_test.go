package hub

import (
	"context"
	"testing"
	"time"
)

// MockRAGSyncService is a mock implementation of RAGSyncService for testing
type MockRAGSyncService struct {
	records []RAGSyncRecord
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	var pending []RAGSyncRecord
	count := 0
	for _, r := range m.records {
		if r.SyncStatus == SyncStatusPending {
			pending = append(pending, r)
			count++
			if count >= limit {
				break
			}
		}
	}
	return pending, nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	idMap := make(map[string]bool)
	for _, id := range ids {
		idMap[id] = true
	}

	for i, r := range m.records {
		if idMap[r.ID] {
			m.records[i].SyncStatus = SyncStatusSynced
			m.records[i].LastSyncAt = time.Now()
		}
	}
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	m.records = append(m.records, records...)
	return nil
}

func TestRAGSyncService(t *testing.T) {
	ctx := context.Background()
	mockService := &MockRAGSyncService{
		records: []RAGSyncRecord{
			{ID: "1", Context: "Test Context 1", Vector: []float32{0.1, 0.2}, SyncStatus: SyncStatusPending},
			{ID: "2", Context: "Test Context 2", Vector: []float32{0.3, 0.4}, SyncStatus: SyncStatusSynced},
			{ID: "3", Context: "Test Context 3", Vector: []float32{0.5, 0.6}, SyncStatus: SyncStatusPending},
		},
	}

	// Test FetchPendingSyncs
	pending, err := mockService.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 2 {
		t.Fatalf("Expected 2 pending records, got %d", len(pending))
	}
	if pending[0].ID != "1" || pending[1].ID != "3" {
		t.Fatalf("Expected records 1 and 3, got %s and %s", pending[0].ID, pending[1].ID)
	}

	// Test MarkSynced
	err = mockService.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}
	pendingAfterSync, err := mockService.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pendingAfterSync) != 1 {
		t.Fatalf("Expected 1 pending record after sync, got %d", len(pendingAfterSync))
	}
	if pendingAfterSync[0].ID != "3" {
		t.Fatalf("Expected record 3, got %s", pendingAfterSync[0].ID)
	}

	// Test ProcessIncomingSync
	incoming := []RAGSyncRecord{
		{ID: "4", Context: "Test Context 4", Vector: []float32{0.7, 0.8}, SyncStatus: SyncStatusSynced},
	}
	err = mockService.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}
	if len(mockService.records) != 4 {
		t.Fatalf("Expected 4 records after incoming sync, got %d", len(mockService.records))
	}
}
