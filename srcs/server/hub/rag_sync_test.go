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
	return m.records, nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	for i := range m.records {
		for _, id := range ids {
			if m.records[i].ID == id {
				m.records[i].SyncStatus = SyncStatusSynced
			}
		}
	}
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	m.records = append(m.records, records...)
	return nil
}

func TestRAGSyncFlow(t *testing.T) {
	service := &MockRAGSyncService{
		records: []RAGSyncRecord{
			{ID: "1", SyncStatus: SyncStatusPending},
			{ID: "2", SyncStatus: SyncStatusPending},
		},
	}

	ctx := context.Background()
	pending, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(pending) != 2 {
		t.Fatalf("expected 2 pending records, got %d", len(pending))
	}

	err = service.MarkSynced(ctx, []string{"1", "2"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if service.records[0].SyncStatus != SyncStatusSynced || service.records[1].SyncStatus != SyncStatusSynced {
		t.Fatalf("expected records to be synced")
	}

	err = service.ProcessIncomingSync(ctx, []RAGSyncRecord{
		{ID: "3", SyncStatus: SyncStatusSynced, LastSyncAt: time.Now()},
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(service.records) != 3 {
		t.Fatalf("expected 3 records total, got %d", len(service.records))
	}
}
