package hub

import (
	"context"
	"testing"
	"time"
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

func TestRAGSyncServiceInterface(t *testing.T) {
	var svc RAGSyncService = &mockRAGSyncService{}
	ctx := context.Background()

	records, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(records) != 1 || records[0].SyncStatus != SyncStatusPending {
		t.Errorf("expected 1 pending record, got %v", records)
	}

	err = svc.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Errorf("unexpected error: %v", err)
	}

	err = svc.ProcessIncomingSync(ctx, []RAGSyncRecord{{ID: "2", Context: "test", SyncStatus: SyncStatusSynced, LastSyncAt: time.Now()}})
	if err != nil {
		t.Errorf("unexpected error: %v", err)
	}
}
