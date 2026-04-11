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
	return m.records, nil
}

func (m *mockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	for i := range m.records {
		for _, id := range ids {
			if m.records[i].ID == id {
				m.records[i].SyncStatus = SyncStatusSynced
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
	service := &mockRAGSyncService{}

	records := []RAGSyncRecord{
		{ID: "1", Context: "test context", Vector: []byte{1, 2, 3}, SyncStatus: SyncStatusPending, LastSyncAt: time.Now()},
	}

	err := service.ProcessIncomingSync(context.Background(), records)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	fetched, err := service.FetchPendingSyncs(context.Background(), 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}

	if len(fetched) != 1 {
		t.Fatalf("expected 1 record, got %d", len(fetched))
	}

	err = service.MarkSynced(context.Background(), []string{"1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	if service.records[0].SyncStatus != SyncStatusSynced {
		t.Fatalf("expected sync status to be synced, got %s", service.records[0].SyncStatus)
	}
}
