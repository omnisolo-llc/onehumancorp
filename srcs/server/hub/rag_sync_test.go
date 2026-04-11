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
	idMap := make(map[string]bool)
	for _, id := range ids {
		idMap[id] = true
	}

	for i := range m.records {
		if idMap[m.records[i].ID] {
			m.records[i].SyncStatus = SyncStatusSynced
			m.records[i].LastSyncAt = time.Now()
		}
	}
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return errors.New("no records provided")
	}
	m.records = append(m.records, records...)
	return nil
}

func TestMockRAGSyncService(t *testing.T) {
	mock := &MockRAGSyncService{
		records: []RAGSyncRecord{
			{ID: "1", Context: "test 1", SyncStatus: SyncStatusPending},
			{ID: "2", Context: "test 2", SyncStatus: SyncStatusSynced},
			{ID: "3", Context: "test 3", SyncStatus: SyncStatusPending},
		},
	}

	ctx := context.Background()

	pending, err := mock.FetchPendingSyncs(ctx, 1)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(pending) != 1 {
		t.Fatalf("expected 1 pending record, got %d", len(pending))
	}
	if pending[0].ID != "1" {
		t.Fatalf("expected record ID 1, got %s", pending[0].ID)
	}

	err = mock.MarkSynced(ctx, []string{"1", "3"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	pending, err = mock.FetchPendingSyncs(ctx, 5)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(pending) != 0 {
		t.Fatalf("expected 0 pending records after MarkSynced, got %d", len(pending))
	}

	err = mock.ProcessIncomingSync(ctx, []RAGSyncRecord{
		{ID: "4", Context: "test 4", SyncStatus: SyncStatusSynced},
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(mock.records) != 4 {
		t.Fatalf("expected 4 total records after ProcessIncomingSync, got %d", len(mock.records))
	}
}
