package hub

import (
	"context"
	"testing"
	"time"
)

type mockRAGSyncService struct{}

func (m *mockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	return []RAGSyncRecord{
		{
			ID:         "test-id",
			Context:    "test-context",
			Vector:     []float32{0.1, 0.2},
			SyncStatus: SyncStatusPending,
			LastSyncAt: time.Now(),
		},
	}, nil
}

func (m *mockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	return nil
}

func (m *mockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	return nil
}

func TestMockRAGSyncService(t *testing.T) {
	service := &mockRAGSyncService{}

	records, err := service.FetchPendingSyncs(context.Background(), 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}

	if records[0].SyncStatus != SyncStatusPending {
		t.Errorf("expected pending status, got %v", records[0].SyncStatus)
	}
}
