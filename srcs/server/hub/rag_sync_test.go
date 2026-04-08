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
			if len(pending) == limit {
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
			// Simulate updating counter metric
			RecordsSyncedCounter.Add(ctx, 1)
		}
	}
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		SyncErrorsCounter.Add(ctx, 1)
		return errors.New("no records provided")
	}
	m.records = append(m.records, records...)
	return nil
}

func TestRAGSyncFlow(t *testing.T) {
	ctx := context.Background()
	mockSvc := &MockRAGSyncService{
		records: []RAGSyncRecord{
			{ID: "1", Context: "Test 1", SyncStatus: SyncStatusPending},
			{ID: "2", Context: "Test 2", SyncStatus: SyncStatusPending},
			{ID: "3", Context: "Test 3", SyncStatus: SyncStatusSynced},
		},
	}

	// 1. Fetch pending
	pending, err := mockSvc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(pending) != 2 {
		t.Errorf("expected 2 pending records, got %d", len(pending))
	}

	// 2. Mark synced
	var idsToSync []string
	for _, p := range pending {
		idsToSync = append(idsToSync, p.ID)
	}
	err = mockSvc.MarkSynced(ctx, idsToSync)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	// 3. Verify they are marked synced
	pendingAgain, err := mockSvc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(pendingAgain) != 0 {
		t.Errorf("expected 0 pending records after MarkSynced, got %d", len(pendingAgain))
	}

	// 4. Test ProcessIncomingSync
	newRecords := []RAGSyncRecord{
		{ID: "4", Context: "Incoming 1", SyncStatus: SyncStatusPending},
	}
	err = mockSvc.ProcessIncomingSync(ctx, newRecords)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	finalPending, _ := mockSvc.FetchPendingSyncs(ctx, 10)
	if len(finalPending) != 1 || finalPending[0].ID != "4" {
		t.Errorf("expected 1 final pending record with ID 4, got %d", len(finalPending))
	}

	// 5. Test error case
	err = mockSvc.ProcessIncomingSync(ctx, []RAGSyncRecord{})
	if err == nil {
		t.Errorf("expected error for empty incoming sync")
	}
}
