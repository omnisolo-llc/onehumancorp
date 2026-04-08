package hub

import (
	"context"
	"testing"
	"time"
)

// MockRAGSyncService is a simple mock implementation of the RAGSyncService interface.
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

	for i, r := range m.records {
		if idMap[r.ID] {
			m.records[i].SyncStatus = SyncStatusSynced
			m.records[i].LastSyncAt = time.Now()
		}
	}
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	for _, incoming := range records {
		found := false
		for i, existing := range m.records {
			if existing.ID == incoming.ID {
				m.records[i] = incoming
				found = true
				break
			}
		}
		if !found {
			m.records = append(m.records, incoming)
		}
	}
	return nil
}

func TestMockRAGSyncService_FetchPendingSyncs(t *testing.T) {
	mock := &MockRAGSyncService{
		records: []RAGSyncRecord{
			{ID: "1", SyncStatus: SyncStatusPending},
			{ID: "2", SyncStatus: SyncStatusSynced},
			{ID: "3", SyncStatus: SyncStatusPending},
		},
	}

	pending, err := mock.FetchPendingSyncs(context.Background(), 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(pending) != 2 {
		t.Fatalf("expected 2 pending records, got %d", len(pending))
	}
}

func TestMockRAGSyncService_MarkSynced(t *testing.T) {
	mock := &MockRAGSyncService{
		records: []RAGSyncRecord{
			{ID: "1", SyncStatus: SyncStatusPending},
		},
	}

	err := mock.MarkSynced(context.Background(), []string{"1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if mock.records[0].SyncStatus != SyncStatusSynced {
		t.Fatalf("expected sync status to be synced, got %v", mock.records[0].SyncStatus)
	}
	if mock.records[0].LastSyncAt.IsZero() {
		t.Fatalf("expected LastSyncAt to be set, was zero")
	}
}

func TestMockRAGSyncService_ProcessIncomingSync(t *testing.T) {
	mock := &MockRAGSyncService{
		records: []RAGSyncRecord{},
	}

	incoming := []RAGSyncRecord{
		{ID: "1", Context: "test context", SyncStatus: SyncStatusSynced, LastSyncAt: time.Now()},
	}

	err := mock.ProcessIncomingSync(context.Background(), incoming)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(mock.records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(mock.records))
	}
	if mock.records[0].Context != "test context" {
		t.Fatalf("expected context 'test context', got '%s'", mock.records[0].Context)
	}
}
