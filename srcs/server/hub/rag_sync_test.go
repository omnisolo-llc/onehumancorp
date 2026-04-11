package hub

import (
	"context"
	"errors"
	"testing"
	"time"
)

type mockRAGSyncService struct{}

func (m *mockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	return []RAGSyncRecord{
		{ID: "1", Context: "test", Vector: "vec", SyncStatus: SyncStatusPending, LastSyncAt: time.Time{}},
	}, nil
}

func (m *mockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return errors.New("no ids provided")
	}
	return nil
}

func (m *mockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	return nil
}

func TestRAGSyncService(t *testing.T) {
	var svc RAGSyncService = &mockRAGSyncService{}
	ctx := context.Background()

	records, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(records) != 1 {
		t.Errorf("expected 1 record, got %d", len(records))
	}

	err = svc.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Errorf("unexpected error: %v", err)
	}

	// Trigger metrics to verify syntax/compilation
	TrackRagSyncSuccess(ctx, 1)
	TrackRagSyncError(ctx)
}
