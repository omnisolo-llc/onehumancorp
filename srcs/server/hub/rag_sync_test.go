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
	if limit > len(m.records) {
		limit = len(m.records)
	}
	return m.records[:limit], nil
}

func (m *mockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	for _, id := range ids {
		for i, record := range m.records {
			if record.ID == id {
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

func TestRAGSyncService(t *testing.T) {
	service := &mockRAGSyncService{
		records: []RAGSyncRecord{
			{ID: "1", Context: "Test 1", Vector: []float32{0.1, 0.2}, SyncStatus: SyncStatusPending},
			{ID: "2", Context: "Test 2", Vector: []float32{0.3, 0.4}, SyncStatus: SyncStatusPending},
		},
	}

	ctx := context.Background()
	pending, err := service.FetchPendingSyncs(ctx, 2)
	if err != nil {
		t.Fatalf("Failed to fetch pending syncs: %v", err)
	}

	if len(pending) != 2 {
		t.Fatalf("Expected 2 pending syncs, got %d", len(pending))
	}

	err = service.MarkSynced(ctx, []string{"1", "2"})
	if err != nil {
		t.Fatalf("Failed to mark synced: %v", err)
	}

	if service.records[0].SyncStatus != SyncStatusSynced || service.records[1].SyncStatus != SyncStatusSynced {
		t.Fatalf("Expected records to be marked synced")
	}

	newRecords := []RAGSyncRecord{
		{ID: "3", Context: "Test 3", Vector: []float32{0.5, 0.6}, SyncStatus: SyncStatusPending},
	}
	err = service.ProcessIncomingSync(ctx, newRecords)
	if err != nil {
		t.Fatalf("Failed to process incoming sync: %v", err)
	}

	if len(service.records) != 3 {
		t.Fatalf("Expected 3 records, got %d", len(service.records))
	}
}
