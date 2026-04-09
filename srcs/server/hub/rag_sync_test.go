package hub_test

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/hub"
)

type mockRAGSyncService struct {
	pending []hub.RAGSyncRecord
	synced  []string
	pushed  []hub.RAGSyncRecord
}

func (m *mockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]hub.RAGSyncRecord, error) {
	if limit > len(m.pending) {
		limit = len(m.pending)
	}
	return m.pending[:limit], nil
}

func (m *mockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	m.synced = append(m.synced, ids...)
	return nil
}

func (m *mockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []hub.RAGSyncRecord) error {
	m.pushed = append(m.pushed, records...)
	return nil
}

func TestRAGSyncService(t *testing.T) {
	mock := &mockRAGSyncService{
		pending: []hub.RAGSyncRecord{
			{ID: "1", Context: "test1", SyncStatus: hub.SyncStatusPending, LastSyncAt: time.Now()},
			{ID: "2", Context: "test2", SyncStatus: hub.SyncStatusPending, LastSyncAt: time.Now()},
		},
	}

	ctx := context.Background()

	pending, err := mock.FetchPendingSyncs(ctx, 1)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(pending) != 1 {
		t.Fatalf("expected 1 pending record, got %d", len(pending))
	}

	err = mock.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(mock.synced) != 1 || mock.synced[0] != "1" {
		t.Fatalf("expected ID '1' to be marked synced, got %v", mock.synced)
	}

	err = mock.ProcessIncomingSync(ctx, []hub.RAGSyncRecord{
		{ID: "3", Context: "test3", SyncStatus: hub.SyncStatusSynced, LastSyncAt: time.Now()},
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(mock.pushed) != 1 || mock.pushed[0].ID != "3" {
		t.Fatalf("expected 1 pushed record with ID '3', got %v", mock.pushed)
	}
}
