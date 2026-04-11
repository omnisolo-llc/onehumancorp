package hub

import (
	"context"
	"testing"
	"time"
)

type MockRAGSyncService struct {
	Records []RAGSyncRecord
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	var pending []RAGSyncRecord
	for _, r := range m.Records {
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
	for i, r := range m.Records {
		if idMap[r.ID] {
			m.Records[i].SyncStatus = SyncStatusSynced
			m.Records[i].LastSyncAt = time.Now()
		}
	}
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	m.Records = append(m.Records, records...)
	return nil
}

func TestMockRAGSyncService(t *testing.T) {
	mockService := &MockRAGSyncService{
		Records: []RAGSyncRecord{
			{ID: "1", Context: "test1", Vector: []byte{0x01, 0x02}, SyncStatus: SyncStatusPending},
			{ID: "2", Context: "test2", Vector: []byte{0x03, 0x04}, SyncStatus: SyncStatusSynced},
			{ID: "3", Context: "test3", Vector: []byte{0x05, 0x06}, SyncStatus: SyncStatusPending},
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
	ids := []string{"1", "3"}
	err = mockService.MarkSynced(ctx, ids)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	pending, _ = mockService.FetchPendingSyncs(ctx, 10)
	if len(pending) != 0 {
		t.Fatalf("expected 0 pending records after MarkSynced, got %d", len(pending))
	}

	// Test ProcessIncomingSync
	newRecords := []RAGSyncRecord{
		{ID: "4", Context: "test4", Vector: []byte{0x07, 0x08}, SyncStatus: SyncStatusSynced},
	}
	err = mockService.ProcessIncomingSync(ctx, newRecords)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(mockService.Records) != 4 {
		t.Fatalf("expected 4 records total, got %d", len(mockService.Records))
	}
}
