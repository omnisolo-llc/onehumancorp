package hub

import (
	"context"
	"testing"
)

type mockRAGSyncService struct {
	pending []RAGSyncRecord
	synced  []string
	cloudDB []RAGSyncRecord
}

func (m *mockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if limit > len(m.pending) {
		return m.pending, nil
	}
	return m.pending[:limit], nil
}

func (m *mockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	m.synced = append(m.synced, ids...)
	return nil
}

func (m *mockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	m.cloudDB = append(m.cloudDB, records...)
	return nil
}

func TestRAGSyncService_Flow(t *testing.T) {
	svc := &mockRAGSyncService{
		pending: []RAGSyncRecord{
			{ID: "1", Context: "test1", SyncStatus: SyncStatusPending},
			{ID: "2", Context: "test2", SyncStatus: SyncStatusPending},
		},
	}

	ctx := context.Background()
	pending, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(pending) != 2 {
		t.Fatalf("expected 2 pending records, got %d", len(pending))
	}

	err = svc.ProcessIncomingSync(ctx, pending)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(svc.cloudDB) != 2 {
		t.Fatalf("expected 2 records in cloudDB, got %d", len(svc.cloudDB))
	}

	ids := []string{"1", "2"}
	err = svc.MarkSynced(ctx, ids)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(svc.synced) != 2 {
		t.Fatalf("expected 2 synced ids, got %d", len(svc.synced))
	}
}
