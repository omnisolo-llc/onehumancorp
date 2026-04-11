package hub

import (
	"context"
	"testing"
	"time"
)

type MockRAGSyncService struct {
	Records []RAGSyncRecord
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	var pending []RAGSyncRecord
	for _, r := range m.Records {
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

	for i, r := range m.Records {
		if idMap[r.ID] {
			m.Records[i].SyncStatus = SyncStatusSynced
			m.Records[i].LastSyncAt = time.Now()
		}
	}
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	m.Records = append(m.Records, records...)
	return nil
}

func TestFetchPendingSyncs(t *testing.T) {
	mockService := &MockRAGSyncService{
		Records: []RAGSyncRecord{
			{ID: "1", SyncStatus: SyncStatusPending},
			{ID: "2", SyncStatus: SyncStatusSynced},
			{ID: "3", SyncStatus: SyncStatusPending},
		},
	}

	pending, err := mockService.FetchPendingSyncs(context.Background(), 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(pending) != 2 {
		t.Errorf("expected 2 pending records, got %d", len(pending))
	}

	if pending[0].ID != "1" || pending[1].ID != "3" {
		t.Errorf("unexpected records returned")
	}
}

func TestMarkSynced(t *testing.T) {
	mockService := &MockRAGSyncService{
		Records: []RAGSyncRecord{
			{ID: "1", SyncStatus: SyncStatusPending},
			{ID: "2", SyncStatus: SyncStatusPending},
		},
	}

	err := mockService.MarkSynced(context.Background(), []string{"1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if mockService.Records[0].SyncStatus != SyncStatusSynced {
		t.Errorf("expected record 1 to be synced")
	}

	if mockService.Records[1].SyncStatus != SyncStatusPending {
		t.Errorf("expected record 2 to still be pending")
	}
}

func TestProcessIncomingSync(t *testing.T) {
	mockService := &MockRAGSyncService{}

	records := []RAGSyncRecord{
		{ID: "1", SyncStatus: SyncStatusSynced},
	}

	err := mockService.ProcessIncomingSync(context.Background(), records)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(mockService.Records) != 1 || mockService.Records[0].ID != "1" {
		t.Errorf("expected 1 record to be added")
	}
}
