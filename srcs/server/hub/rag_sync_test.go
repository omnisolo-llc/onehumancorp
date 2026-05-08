package hub

import (
	"context"
	"testing"
	"time"
)

type mockRAGSyncService struct {
	records []RAGSyncRecord
}

func (m *mockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
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

func (m *mockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	for i, r := range m.records {
		for _, id := range ids {
			if r.ID == id {
				m.records[i].SyncStatus = SyncStatusSynced
				m.records[i].LastSyncAt = time.Now()
			}
		}
	}
	return nil
}

func (m *mockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	m.records = append(m.records, records...)
	return nil
}

func TestFetchPendingSyncs(t *testing.T) {
	mockService := &mockRAGSyncService{
		records: []RAGSyncRecord{
			{ID: "1", SyncStatus: SyncStatusPending},
			{ID: "2", SyncStatus: SyncStatusSynced},
			{ID: "3", SyncStatus: SyncStatusPending},
		},
	}

	pending, err := mockService.FetchPendingSyncs(context.Background(), 10)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if len(pending) != 2 {
		t.Fatalf("expected 2 pending records, got %d", len(pending))
	}
}

func TestMarkSynced(t *testing.T) {
	mockService := &mockRAGSyncService{
		records: []RAGSyncRecord{
			{ID: "1", SyncStatus: SyncStatusPending},
			{ID: "2", SyncStatus: SyncStatusPending},
		},
	}

	err := mockService.MarkSynced(context.Background(), []string{"1"})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if mockService.records[0].SyncStatus != SyncStatusSynced {
		t.Fatalf("expected record 1 to be synced")
	}

	if mockService.records[1].SyncStatus != SyncStatusPending {
		t.Fatalf("expected record 2 to be pending")
	}
}

func TestProcessIncomingSync(t *testing.T) {
	mockService := &mockRAGSyncService{
		records: []RAGSyncRecord{},
	}

	recordsToSync := []RAGSyncRecord{
		{ID: "1", SyncStatus: SyncStatusSynced},
		{ID: "2", SyncStatus: SyncStatusSynced},
	}

	err := mockService.ProcessIncomingSync(context.Background(), recordsToSync)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if len(mockService.records) != 2 {
		t.Fatalf("expected 2 records to be processed, got %d", len(mockService.records))
	}
}
