package hub

import (
	"context"
	"errors"
	"testing"
	"time"
)

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
	for _, id := range ids {
		for i, r := range m.records {
			if r.ID == id {
				m.records[i].SyncStatus = SyncStatusSynced
				m.records[i].LastSyncAt = time.Now()
			}
		}
	}
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) > 0 && records[0].ID == "error-id" {
		return errors.New("simulated sync error")
	}
	for _, r := range records {
		r.SyncStatus = SyncStatusSynced
		m.records = append(m.records, r)
	}
	return nil
}

func TestRAGSyncService(t *testing.T) {
	mockService := &MockRAGSyncService{
		records: []RAGSyncRecord{
			{ID: "1", Context: "test 1", SyncStatus: SyncStatusPending},
			{ID: "2", Context: "test 2", SyncStatus: SyncStatusSynced},
			{ID: "3", Context: "test 3", SyncStatus: SyncStatusPending},
		},
	}

	ctx := context.Background()

	// Test FetchPendingSyncs
	pending, err := mockService.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(pending) != 2 {
		t.Errorf("expected 2 pending syncs, got %d", len(pending))
	}

	// Test ProcessIncomingSync success
	newRecords := []RAGSyncRecord{
		{ID: "4", Context: "test 4", SyncStatus: SyncStatusPending},
	}
	err = mockService.ProcessIncomingSync(ctx, newRecords)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	RecordRAGSyncSuccess(ctx, len(newRecords))

	// Test ProcessIncomingSync error
	errorRecords := []RAGSyncRecord{
		{ID: "error-id", Context: "test error", SyncStatus: SyncStatusPending},
	}
	err = mockService.ProcessIncomingSync(ctx, errorRecords)
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
	RecordRAGSyncError(ctx)

	// Test MarkSynced
	idsToMark := []string{"1"}
	err = mockService.MarkSynced(ctx, idsToMark)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	pendingAfterMark, _ := mockService.FetchPendingSyncs(ctx, 10)
	if len(pendingAfterMark) != 1 {
		t.Errorf("expected 1 pending sync, got %d", len(pendingAfterMark))
	}
}
