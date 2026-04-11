package hub

import (
	"context"
	"testing"
	"time"
)

type mockRAGSyncService struct{}

func (m *mockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	return []RAGSyncRecord{}, nil
}

func (m *mockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	return nil
}

func (m *mockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	return nil
}

func TestMockRAGSyncService(t *testing.T) {
	var service RAGSyncService = &mockRAGSyncService{}

	records, err := service.FetchPendingSyncs(context.Background(), 10)
	if err != nil {
		t.Errorf("FetchPendingSyncs returned an error: %v", err)
	}
	if len(records) != 0 {
		t.Errorf("Expected 0 records, got %d", len(records))
	}

	err = service.MarkSynced(context.Background(), []string{"1", "2"})
	if err != nil {
		t.Errorf("MarkSynced returned an error: %v", err)
	}

	err = service.ProcessIncomingSync(context.Background(), []RAGSyncRecord{
		{ID: "1", Context: "test", SyncStatus: SyncStatusPending, LastSyncAt: time.Now()},
	})
	if err != nil {
		t.Errorf("ProcessIncomingSync returned an error: %v", err)
	}
}
