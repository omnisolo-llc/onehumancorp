package hub

import (
	"context"
	"testing"
	"time"
)

// MockRAGSyncService is a mock implementation of RAGSyncService for testing
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
	for _, id := range ids {
		for i, r := range m.Records {
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

func TestRAGSyncServiceMock(t *testing.T) {
	mock := &MockRAGSyncService{
		Records: []RAGSyncRecord{
			{ID: "1", Context: "test 1", SyncStatus: SyncStatusPending},
			{ID: "2", Context: "test 2", SyncStatus: SyncStatusSynced},
			{ID: "3", Context: "test 3", SyncStatus: SyncStatusPending},
		},
	}

	ctx := context.Background()

	pending, err := mock.FetchPendingSyncs(ctx, 2)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(pending) != 2 {
		t.Errorf("expected 2 pending records, got %d", len(pending))
	}
	if pending[0].ID != "1" || pending[1].ID != "3" {
		t.Errorf("unexpected pending records")
	}

	err = mock.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	pendingAfter, err := mock.FetchPendingSyncs(ctx, 2)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(pendingAfter) != 1 {
		t.Errorf("expected 1 pending record, got %d", len(pendingAfter))
	}
	if pendingAfter[0].ID != "3" {
		t.Errorf("unexpected pending record")
	}

	err = mock.ProcessIncomingSync(ctx, []RAGSyncRecord{
		{ID: "4", Context: "test 4", SyncStatus: SyncStatusPending},
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(mock.Records) != 4 {
		t.Errorf("expected 4 records, got %d", len(mock.Records))
	}
}
