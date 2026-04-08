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

func TestMockFetchPendingSyncs(t *testing.T) {
	mockService := &MockRAGSyncService{
		Records: []RAGSyncRecord{
			{ID: "1", SyncStatus: SyncStatusPending},
			{ID: "2", SyncStatus: SyncStatusSynced},
			{ID: "3", SyncStatus: SyncStatusPending},
		},
	}

	ctx := context.Background()
	pending, err := mockService.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(pending) != 2 {
		t.Fatalf("expected 2 pending records, got %d", len(pending))
	}
}

func TestMockMarkSynced(t *testing.T) {
	mockService := &MockRAGSyncService{
		Records: []RAGSyncRecord{
			{ID: "1", SyncStatus: SyncStatusPending},
			{ID: "2", SyncStatus: SyncStatusPending},
		},
	}

	ctx := context.Background()
	err := mockService.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if mockService.Records[0].SyncStatus != SyncStatusSynced {
		t.Fatalf("expected record 1 to be synced")
	}
	if mockService.Records[1].SyncStatus != SyncStatusPending {
		t.Fatalf("expected record 2 to still be pending")
	}
}

func TestMockProcessIncomingSync(t *testing.T) {
	mockService := &MockRAGSyncService{
		Records: []RAGSyncRecord{},
	}

	ctx := context.Background()
	incoming := []RAGSyncRecord{
		{ID: "1", Context: "test", SyncStatus: SyncStatusSynced},
	}
	err := mockService.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if len(mockService.Records) != 1 {
		t.Fatalf("expected 1 record after process incoming sync, got %d", len(mockService.Records))
	}
}
