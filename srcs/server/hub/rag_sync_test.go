package hub

import (
	"context"
	"errors"
	"testing"
	"time"
)

// MockRAGSyncService is a mock implementation of RAGSyncService for testing
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
	if len(pending) > limit && limit > 0 {
		return pending[:limit], nil
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
			recordsSyncedCounter.Add(ctx, 1)
		}
	}
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    if len(records) == 0 {
        return errors.New("no records to sync")
    }
	for _, r := range records {
		r.SyncStatus = SyncStatusSynced
		m.records = append(m.records, r)
	}
	return nil
}

func TestRAGSyncService(t *testing.T) {
	ctx := context.Background()
	mockService := &MockRAGSyncService{
		records: []RAGSyncRecord{
			{ID: "1", Context: "context 1", SyncStatus: SyncStatusPending},
			{ID: "2", Context: "context 2", SyncStatus: SyncStatusSynced},
			{ID: "3", Context: "context 3", SyncStatus: SyncStatusPending},
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
	err = mockService.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	pending, _ = mockService.FetchPendingSyncs(ctx, 10)
	if len(pending) != 1 {
		t.Errorf("expected 1 pending record after marking 1 as synced, got %d", len(pending))
	}

	// Test ProcessIncomingSync
	newRecords := []RAGSyncRecord{
		{ID: "4", Context: "context 4", SyncStatus: SyncStatusPending},
	}
	err = mockService.ProcessIncomingSync(ctx, newRecords)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(mockService.records) != 4 {
		t.Errorf("expected 4 total records after processing incoming sync, got %d", len(mockService.records))
	}
}
