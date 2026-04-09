package hub

import (
	"context"
	"testing"
)

type MockRAGSyncService struct {
	Pending []RAGSyncRecord
	Synced  []string
	Pushed  []RAGSyncRecord
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if limit > len(m.Pending) {
		limit = len(m.Pending)
	}
	return m.Pending[:limit], nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	m.Synced = append(m.Synced, ids...)
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	m.Pushed = append(m.Pushed, records...)
	return nil
}

func TestRAGSyncService(t *testing.T) {
	mock := &MockRAGSyncService{
		Pending: []RAGSyncRecord{
			{ID: "1", Context: "test", Vector: []float32{1, 2, 3}, SyncStatus: SyncStatusPending},
		},
	}

	ctx := context.Background()
	records, err := mock.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}

	err = mock.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(mock.Synced) != 1 || mock.Synced[0] != "1" {
		t.Fatalf("expected id '1' to be marked synced")
	}

	err = mock.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(mock.Pushed) != 1 {
		t.Fatalf("expected 1 record pushed")
	}
}
