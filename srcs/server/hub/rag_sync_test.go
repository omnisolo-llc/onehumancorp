package hub

import (
	"context"
	"errors"
	"testing"
	"time"
)

// MockRAGSyncService is a mock implementation of RAGSyncService for testing.
type MockRAGSyncService struct {
	Records           []RAGSyncRecord
	MarkSyncedCalled  bool
	ProcessSyncCalled bool
	ShouldError       bool
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if m.ShouldError {
		return nil, errors.New("mock fetch error")
	}
	var pending []RAGSyncRecord
	for _, r := range m.Records {
		if r.SyncStatus == SyncStatusPending {
			pending = append(pending, r)
		}
	}
	if len(pending) > limit {
		pending = pending[:limit]
	}
	return pending, nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	m.MarkSyncedCalled = true
	if m.ShouldError {
		return errors.New("mock mark synced error")
	}
	idMap := make(map[string]bool)
	for _, id := range ids {
		idMap[id] = true
	}
	for i, r := range m.Records {
		if idMap[r.ID] {
			m.Records[i].SyncStatus = SyncStatusSynced
			m.Records[i].LastSyncAt = time.Now()
		}
	}
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	m.ProcessSyncCalled = true
	if m.ShouldError {
		return errors.New("mock process incoming sync error")
	}
	// In a real implementation this would upsert into cloud DB
	m.Records = append(m.Records, records...)
	return nil
}

func TestMockRAGSyncService(t *testing.T) {
	mockService := &MockRAGSyncService{
		Records: []RAGSyncRecord{
			{ID: "1", Context: "Memory 1", Vector: []float32{0.1, 0.2}, SyncStatus: SyncStatusPending},
			{ID: "2", Context: "Memory 2", Vector: []float32{0.3, 0.4}, SyncStatus: SyncStatusPending},
			{ID: "3", Context: "Memory 3", Vector: []float32{0.5, 0.6}, SyncStatus: SyncStatusSynced},
		},
	}

	ctx := context.Background()

	// Test FetchPendingSyncs
	pending, err := mockService.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(pending) != 2 {
		t.Fatalf("expected 2 pending records, got %d", len(pending))
	}

	// Test MarkSynced
	err = mockService.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if !mockService.MarkSyncedCalled {
		t.Fatal("expected MarkSynced to be called")
	}

	// Verify it was marked synced
	pendingAgain, _ := mockService.FetchPendingSyncs(ctx, 10)
	if len(pendingAgain) != 1 {
		t.Fatalf("expected 1 pending record after sync, got %d", len(pendingAgain))
	}
	if pendingAgain[0].ID != "2" {
		t.Fatalf("expected record 2 to be pending, got %s", pendingAgain[0].ID)
	}

	// Test ProcessIncomingSync
	newRecords := []RAGSyncRecord{
		{ID: "4", Context: "Memory 4", Vector: []float32{0.7, 0.8}, SyncStatus: SyncStatusSynced},
	}
	err = mockService.ProcessIncomingSync(ctx, newRecords)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if !mockService.ProcessSyncCalled {
		t.Fatal("expected ProcessIncomingSync to be called")
	}
	if len(mockService.Records) != 4 {
		t.Fatalf("expected 4 total records after processing incoming sync, got %d", len(mockService.Records))
	}
}
