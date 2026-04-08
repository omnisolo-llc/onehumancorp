package hub

import (
	"context"
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
	for i, r := range m.records {
		if idMap[r.ID] {
			m.records[i].SyncStatus = SyncStatusSynced
			m.records[i].LastSyncAt = time.Now()
		}
	}
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	// For mock purposes, just append
	m.records = append(m.records, records...)
	return nil
}

func TestFetchPendingSyncs(t *testing.T) {
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
		t.Errorf("expected 2 pending records, got %d", len(pending))
	}
}

func TestMarkSynced(t *testing.T) {
	mock := &MockRAGSyncService{
		records: []RAGSyncRecord{
			{ID: "1", SyncStatus: SyncStatusPending},
			{ID: "2", SyncStatus: SyncStatusPending},
		},
	}

	err := mock.MarkSynced(context.Background(), []string{"1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if mock.records[0].SyncStatus != SyncStatusSynced {
		t.Errorf("expected record 1 to be synced")
	}

	if mock.records[1].SyncStatus != SyncStatusPending {
		t.Errorf("expected record 2 to be pending")
	}
}

func TestProcessIncomingSync(t *testing.T) {
	mock := &MockRAGSyncService{}

	err := mock.ProcessIncomingSync(context.Background(), []RAGSyncRecord{
		{ID: "1", SyncStatus: SyncStatusPending},
	})

	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(mock.records) != 1 {
		t.Errorf("expected 1 record after process incoming sync, got %d", len(mock.records))
	}
}
