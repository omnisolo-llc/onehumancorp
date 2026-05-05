package hub

import (
	"context"
	"testing"

)

// MockSyncService is a mock implementation of AutoDreamSyncService
type MockSyncService struct {
	PendingRecords []AutoDreamSyncRecord
	IncomingRecords []AutoDreamSyncRecord
	MarkedSynced []string
}

func (m *MockSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]AutoDreamSyncRecord, error) {
	if limit > len(m.PendingRecords) {
		limit = len(m.PendingRecords)
	}
	return m.PendingRecords[:limit], nil
}

func (m *MockSyncService) ProcessIncomingSyncs(ctx context.Context, records []AutoDreamSyncRecord) error {
	m.IncomingRecords = append(m.IncomingRecords, records...)
	return nil
}

func (m *MockSyncService) MarkRecordsSynced(ctx context.Context, recordIDs []string) error {
	m.MarkedSynced = append(m.MarkedSynced, recordIDs...)
	return nil
}

func TestAutoDreamSyncService(t *testing.T) {
	mockService := &MockSyncService{
		PendingRecords: []AutoDreamSyncRecord{
			{ID: "record1", Content: "content1"},
			{ID: "record2", Content: "content2"},
		},
	}

	ctx := context.Background()

	// Test FetchPendingSyncs
	records, err := mockService.FetchPendingSyncs(ctx, 1)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(records) != 1 {
		t.Errorf("expected 1 record, got %d", len(records))
	}
	if records[0].ID != "record1" {
		t.Errorf("expected record1, got %s", records[0].ID)
	}

	// Test ProcessIncomingSyncs
	err = mockService.ProcessIncomingSyncs(ctx, []AutoDreamSyncRecord{
		{ID: "record3", Content: "content3"},
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(mockService.IncomingRecords) != 1 {
		t.Errorf("expected 1 incoming record, got %d", len(mockService.IncomingRecords))
	}

	// Test MarkRecordsSynced
	err = mockService.MarkRecordsSynced(ctx, []string{"record1", "record2"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(mockService.MarkedSynced) != 2 {
		t.Errorf("expected 2 marked synced, got %d", len(mockService.MarkedSynced))
	}
}
