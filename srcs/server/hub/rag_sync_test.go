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
		}
	}
	if limit > 0 && len(pending) > limit {
		pending = pending[:limit]
	}
	return pending, nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	for i, r := range m.Records {
		for _, id := range ids {
			if r.ID == id {
				m.Records[i].SyncStatus = SyncStatusSynced
				m.Records[i].LastSyncAt = time.Now()
			}
		}
	}
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	m.Records = append(m.Records, records...)
	return nil
}

func TestRAGSyncService(t *testing.T) {
	mockService := &MockRAGSyncService{
		Records: []RAGSyncRecord{
			{ID: "1", SyncStatus: SyncStatusPending},
			{ID: "2", SyncStatus: SyncStatusSynced},
		},
	}

	pending, err := mockService.FetchPendingSyncs(context.Background(), 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(pending) != 1 || pending[0].ID != "1" {
		t.Errorf("expected 1 pending record with ID '1', got %v", pending)
	}

	err = mockService.MarkSynced(context.Background(), []string{"1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	pending, _ = mockService.FetchPendingSyncs(context.Background(), 10)
	if len(pending) != 0 {
		t.Errorf("expected 0 pending records, got %v", len(pending))
	}
}
