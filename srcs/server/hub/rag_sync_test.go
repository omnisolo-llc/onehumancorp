package hub

import (
	"context"
	"testing"
	"time"
)

type MockRAGSyncService struct {
	records []RAGSyncRecord
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	var result []RAGSyncRecord
	for _, r := range m.records {
		if r.SyncStatus == SyncStatusPending {
			result = append(result, r)
			if len(result) == limit {
				break
			}
		}
	}
	return result, nil
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

func TestMockRAGSyncService(t *testing.T) {
	service := &MockRAGSyncService{}

	records := []RAGSyncRecord{
		{ID: "1", Context: "test 1", SyncStatus: SyncStatusPending},
		{ID: "2", Context: "test 2", SyncStatus: SyncStatusPending},
	}
	_ = service.ProcessIncomingSync(context.Background(), records)

	pending, _ := service.FetchPendingSyncs(context.Background(), 10)
	if len(pending) != 2 {
		t.Errorf("Expected 2 pending syncs, got %d", len(pending))
	}

	_ = service.MarkSynced(context.Background(), []string{"1"})

	pendingAfter, _ := service.FetchPendingSyncs(context.Background(), 10)
	if len(pendingAfter) != 1 {
		t.Errorf("Expected 1 pending sync, got %d", len(pendingAfter))
	}
}
