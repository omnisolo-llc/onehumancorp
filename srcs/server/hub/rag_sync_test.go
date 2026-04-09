package hub

import (
	"context"
	"testing"
	"time"
)

type mockRAGSyncService struct {
	pending   []RAGSyncRecord
	synced    []string
	processed []RAGSyncRecord
}

func (m *mockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	return m.pending, nil
}

func (m *mockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	m.synced = append(m.synced, ids...)
	return nil
}

func (m *mockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	m.processed = append(m.processed, records...)
	return nil
}

func TestRAGSyncService(t *testing.T) {
	svc := &mockRAGSyncService{
		pending: []RAGSyncRecord{
			{ID: "1", Context: "test", SyncStatus: SyncStatusPending},
		},
	}

	ctx := context.Background()

	pending, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil || len(pending) != 1 {
		t.Fatalf("expected 1 pending record, got %d", len(pending))
	}

	err = svc.MarkSynced(ctx, []string{"1"})
	if err != nil || len(svc.synced) != 1 {
		t.Fatalf("expected 1 synced record, got %d", len(svc.synced))
	}

	err = svc.ProcessIncomingSync(ctx, []RAGSyncRecord{{ID: "2", Context: "new", SyncStatus: SyncStatusSynced, LastSyncAt: time.Now()}})
	if err != nil || len(svc.processed) != 1 {
		t.Fatalf("expected 1 processed record, got %d", len(svc.processed))
	}
}
