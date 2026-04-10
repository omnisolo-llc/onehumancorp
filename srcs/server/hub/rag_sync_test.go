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
			Context:    "test context",
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

func TestRAGSyncServiceMock(t *testing.T) {
	svc := &mockRAGSyncService{}
	ctx := context.Background()
	records, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(records) != 1 {
		t.Errorf("expected 1 record, got %d", len(records))
	}
	if records[0].SyncStatus != SyncStatusPending {
		t.Errorf("expected status pending, got %s", records[0].SyncStatus)
	}
}
