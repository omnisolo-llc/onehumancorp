package hub

import (
	"context"
	"testing"
	"time"
)

type MockRAGSyncService struct {
	PendingRecords []RAGSyncRecord
	MarkedSynced   []string
	ProcessedSyncs []RAGSyncRecord
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if limit < len(m.PendingRecords) {
		return m.PendingRecords[:limit], nil
	}
	return m.PendingRecords, nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	m.MarkedSynced = append(m.MarkedSynced, ids...)
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	m.ProcessedSyncs = append(m.ProcessedSyncs, records...)
	return nil
}

func TestRAGSyncServiceInterface(t *testing.T) {
	mock := &MockRAGSyncService{
		PendingRecords: []RAGSyncRecord{
			{ID: "1", Context: "test1", SyncStatus: SyncStatusPending},
			{ID: "2", Context: "test2", SyncStatus: SyncStatusPending},
		},
	}

	ctx := context.Background()
	records, err := mock.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(records) != 2 {
		t.Errorf("expected 2 records, got %d", len(records))
	}

	ids := []string{"1", "2"}
	err = mock.MarkSynced(ctx, ids)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(mock.MarkedSynced) != 2 {
		t.Errorf("expected 2 marked synced, got %d", len(mock.MarkedSynced))
	}

	syncRecords := []RAGSyncRecord{
		{ID: "3", Context: "test3", SyncStatus: SyncStatusSynced, LastSyncAt: time.Now()},
	}

	err = mock.ProcessIncomingSync(ctx, syncRecords)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(mock.ProcessedSyncs) != 1 {
		t.Errorf("expected 1 processed sync, got %d", len(mock.ProcessedSyncs))
	}
}
