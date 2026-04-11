package hub

import (
	"context"
	"errors"
	"testing"
	"time"
)

type MockRAGSyncService struct {
	PendingRecords []RAGSyncRecord
	MarkedSynced   []string
	ProcessedSyncs []RAGSyncRecord
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if limit < 0 {
		return nil, errors.New("invalid limit")
	}
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
			{ID: "1", SyncStatus: SyncStatusPending},
			{ID: "2", SyncStatus: SyncStatusPending},
		},
	}

	records, err := mock.FetchPendingSyncs(context.Background(), 1)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}
	if records[0].ID != "1" {
		t.Fatalf("expected record ID 1, got %s", records[0].ID)
	}
}

func TestMarkSynced(t *testing.T) {
	mock := &MockRAGSyncService{}
	ids := []string{"1", "2"}
	err := mock.MarkSynced(context.Background(), ids)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(mock.MarkedSynced) != 2 {
		t.Fatalf("expected 2 marked synced, got %d", len(mock.MarkedSynced))
	}
}

func TestProcessIncomingSync(t *testing.T) {
	mock := &MockRAGSyncService{}
	records := []RAGSyncRecord{
		{ID: "1", Context: "test context", LastSyncAt: time.Now()},
	}
	err := mock.ProcessIncomingSync(context.Background(), records)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(mock.ProcessedSyncs) != 1 {
		t.Fatalf("expected 1 processed sync, got %d", len(mock.ProcessedSyncs))
	}
}
