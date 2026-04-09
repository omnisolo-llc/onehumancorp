package hub

import (
	"context"
	"testing"
)

type mockRAGSyncService struct{}

func (m *mockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	return []RAGSyncRecord{{ID: "1", SyncStatus: SyncStatusPending}}, nil
}

func (m *mockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	return nil
}

func (m *mockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	return nil
}

func TestRAGSyncService_Interface(t *testing.T) {
	var service RAGSyncService = &mockRAGSyncService{}
	records, err := service.FetchPendingSyncs(context.Background(), 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(records) != 1 || records[0].SyncStatus != SyncStatusPending {
		t.Errorf("unexpected records: %+v", records)
	}
}
