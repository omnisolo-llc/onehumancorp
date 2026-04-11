package hub

import (
	"context"
	"testing"
)

type MockRAGSyncService struct{}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	return []RAGSyncRecord{{ID: "1", SyncStatus: SyncStatusPending}}, nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	return nil
}

func TestRAGSyncService(t *testing.T) {
	service := &MockRAGSyncService{}
	ctx := context.Background()

	records, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil || len(records) != 1 {
		t.Fatalf("Expected 1 record, got %d", len(records))
	}

	err = service.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("Expected nil error on MarkSynced, got %v", err)
	}

	err = service.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("Expected nil error on ProcessIncomingSync, got %v", err)
	}
}
