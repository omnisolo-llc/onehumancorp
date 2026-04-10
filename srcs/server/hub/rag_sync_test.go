package hub

import (
	"context"
	"testing"
)

type MockRAGSyncService struct {
	PendingRecords []RAGSyncRecord
	MarkedSynced   []string
	Processed      []RAGSyncRecord
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if limit > len(m.PendingRecords) {
		return m.PendingRecords, nil
	}
	return m.PendingRecords[:limit], nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	m.MarkedSynced = append(m.MarkedSynced, ids...)
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	m.Processed = append(m.Processed, records...)
	return nil
}

func TestRAGSyncService(t *testing.T) {
	mock := &MockRAGSyncService{
		PendingRecords: []RAGSyncRecord{
			{ID: "1", Context: "Test Context", SyncStatus: SyncStatusPending},
		},
	}

	records, err := mock.FetchPendingSyncs(context.Background(), 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs error: %v", err)
	}

	if len(records) != 1 || records[0].ID != "1" {
		t.Errorf("Unexpected pending records: %v", records)
	}

	err = mock.MarkSynced(context.Background(), []string{"1"})
	if err != nil {
		t.Fatalf("MarkSynced error: %v", err)
	}

	if len(mock.MarkedSynced) != 1 || mock.MarkedSynced[0] != "1" {
		t.Errorf("Unexpected marked synced: %v", mock.MarkedSynced)
	}

	err = mock.ProcessIncomingSync(context.Background(), records)
	if err != nil {
		t.Fatalf("ProcessIncomingSync error: %v", err)
	}

	if len(mock.Processed) != 1 || mock.Processed[0].ID != "1" {
		t.Errorf("Unexpected processed records: %v", mock.Processed)
	}
}
