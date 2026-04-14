package hub

import (
	"context"
	"testing"
	"time"
)

type mockAutoDreamSyncService struct {
	pending []AutoDreamSyncRecord
	synced  []string
	process []AutoDreamSyncRecord
}

func (m *mockAutoDreamSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]AutoDreamSyncRecord, error) {
	if limit > len(m.pending) {
		return m.pending, nil
	}
	return m.pending[:limit], nil
}

func (m *mockAutoDreamSyncService) MarkSynced(ctx context.Context, ids []string) error {
	m.synced = append(m.synced, ids...)
	return nil
}

func (m *mockAutoDreamSyncService) ProcessIncomingSync(ctx context.Context, records []AutoDreamSyncRecord) error {
	m.process = append(m.process, records...)
	return nil
}

func TestAutoDreamSyncService(t *testing.T) {
	mock := &mockAutoDreamSyncService{
		pending: []AutoDreamSyncRecord{
			{ID: "1", Context: "test", SyncStatus: SyncStatusPending},
		},
	}

	ctx := context.Background()

	records, err := mock.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}

	err = mock.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(mock.synced) != 1 || mock.synced[0] != "1" {
		t.Fatalf("expected 1 synced record with ID 1, got %v", mock.synced)
	}

	err = mock.ProcessIncomingSync(ctx, []AutoDreamSyncRecord{
		{ID: "2", Context: "test2", SyncStatus: SyncStatusSynced, LastSyncAt: time.Now()},
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(mock.process) != 1 || mock.process[0].ID != "2" {
		t.Fatalf("expected 1 processed record with ID 2, got %v", mock.process)
	}
}
