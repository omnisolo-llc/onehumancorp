package hub

import (
	"context"
	"testing"
	"time"
)

type MockRAGSyncService struct {
	PendingRecords []RAGSyncRecord
	MarkedSynced   []string
	IncomingSyncs  []RAGSyncRecord
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
	m.IncomingSyncs = append(m.IncomingSyncs, records...)
	return nil
}

func TestRAGSyncService(t *testing.T) {
	mock := &MockRAGSyncService{
		PendingRecords: []RAGSyncRecord{
			{
				ID:         "record-1",
				Context:    "test context",
				Vector:     []float32{0.1, 0.2},
				SyncStatus: SyncStatusPending,
			},
		},
	}

	ctx := context.Background()

	// Test FetchPendingSyncs
	records, err := mock.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}
	if records[0].ID != "record-1" {
		t.Errorf("expected record ID 'record-1', got '%s'", records[0].ID)
	}

	// Test MarkSynced
	err = mock.MarkSynced(ctx, []string{"record-1"})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(mock.MarkedSynced) != 1 || mock.MarkedSynced[0] != "record-1" {
		t.Errorf("MarkedSynced was not updated correctly")
	}

	// Test ProcessIncomingSync
	incoming := []RAGSyncRecord{
		{
			ID:         "record-2",
			Context:    "incoming context",
			Vector:     []float32{0.3, 0.4},
			SyncStatus: SyncStatusSynced,
			LastSyncAt: time.Now(),
		},
	}
	err = mock.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(mock.IncomingSyncs) != 1 || mock.IncomingSyncs[0].ID != "record-2" {
		t.Errorf("IncomingSyncs was not updated correctly")
	}
}
