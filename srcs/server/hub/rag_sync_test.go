package hub

import (
	"context"
	"testing"
	"time"
)

type MockRAGSyncService struct {
	PendingRecords []RAGSyncRecord
	MarkedSynced   []string
	Processed      []RAGSyncRecord
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if len(m.PendingRecords) > limit {
		return m.PendingRecords[:limit], nil
	}
	return m.PendingRecords, nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	m.MarkedSynced = append(m.MarkedSynced, ids...)
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	m.Processed = append(m.Processed, records...)
	return nil
}

func TestRAGSyncService_Mock(t *testing.T) {
	mock := &MockRAGSyncService{
		PendingRecords: []RAGSyncRecord{
			{
				ID:         "1",
				Context:    "test context",
				SyncStatus: SyncStatusPending,
				LastSyncAt: time.Now(),
			},
		},
	}

	records, err := mock.FetchPendingSyncs(context.Background(), 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}
	if records[0].SyncStatus != SyncStatusPending {
		t.Errorf("expected pending status, got %v", records[0].SyncStatus)
	}

	err = mock.MarkSynced(context.Background(), []string{"1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(mock.MarkedSynced) != 1 || mock.MarkedSynced[0] != "1" {
		t.Errorf("expected marked synced ID '1', got %v", mock.MarkedSynced)
	}

	err = mock.ProcessIncomingSync(context.Background(), records)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(mock.Processed) != 1 || mock.Processed[0].ID != "1" {
		t.Errorf("expected processed record ID '1', got %v", mock.Processed)
	}
}
