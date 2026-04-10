package hub

import (
	"context"
	"testing"
	"time"
)

type MockRAGSyncService struct {
	pending []RAGSyncRecord
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if limit > len(m.pending) {
		return m.pending, nil
	}
	return m.pending[:limit], nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	for _, id := range ids {
		for i, record := range m.pending {
			if record.ID == id {
				m.pending[i].SyncStatus = SyncStatusSynced
				m.pending[i].LastSyncAt = time.Now()
			}
		}
	}
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	return nil
}

func TestMockRAGSyncService(t *testing.T) {
	mock := &MockRAGSyncService{
		pending: []RAGSyncRecord{
			{ID: "1", SyncStatus: SyncStatusPending},
		},
	}
	records, err := mock.FetchPendingSyncs(context.Background(), 1)
	if err != nil {
		t.Fatalf("FetchPendingSyncs error: %v", err)
	}
	if len(records) != 1 {
		t.Fatalf("Expected 1 record, got %d", len(records))
	}
	err = mock.MarkSynced(context.Background(), []string{"1"})
	if err != nil {
		t.Fatalf("MarkSynced error: %v", err)
	}
	if mock.pending[0].SyncStatus != SyncStatusSynced {
		t.Fatalf("Expected synced status, got %s", mock.pending[0].SyncStatus)
	}
}
