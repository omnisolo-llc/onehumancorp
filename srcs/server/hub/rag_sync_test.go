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
	if limit > len(m.PendingRecords) {
		limit = len(m.PendingRecords)
	}
	return m.PendingRecords[:limit], nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	m.MarkedSynced = append(m.MarkedSynced, ids...)
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	m.ProcessedSyncs = append(m.ProcessedSyncs, records...)
	return nil
}

func TestFetchPendingSyncs(t *testing.T) {
	mock := &MockRAGSyncService{
		PendingRecords: []RAGSyncRecord{
			{ID: "1", Context: "test 1", SyncStatus: SyncStatusPending},
			{ID: "2", Context: "test 2", SyncStatus: SyncStatusPending},
		},
	}

	ctx := context.Background()
	records, err := mock.FetchPendingSyncs(ctx, 1)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}

	if len(records) != 1 {
		t.Fatalf("Expected 1 record, got %d", len(records))
	}

	if records[0].ID != "1" {
		t.Fatalf("Expected record ID 1, got %s", records[0].ID)
	}
}

func TestMarkSynced(t *testing.T) {
	mock := &MockRAGSyncService{}
	ctx := context.Background()

	err := mock.MarkSynced(ctx, []string{"1", "2"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	if len(mock.MarkedSynced) != 2 {
		t.Fatalf("Expected 2 marked synced, got %d", len(mock.MarkedSynced))
	}
}

func TestProcessIncomingSync(t *testing.T) {
	mock := &MockRAGSyncService{}
	ctx := context.Background()

	now := time.Now()
	records := []RAGSyncRecord{
		{ID: "1", Context: "test 1", Vector: []float32{0.1, 0.2}, SyncStatus: SyncStatusPending, LastSyncAt: now},
	}

	err := mock.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	if len(mock.ProcessedSyncs) != 1 {
		t.Fatalf("Expected 1 processed sync, got %d", len(mock.ProcessedSyncs))
	}
}
