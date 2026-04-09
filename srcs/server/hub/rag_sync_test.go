package hub_test

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/hub"
)

type mockRAGSyncService struct {
	pending []hub.RAGSyncRecord
}

func (m *mockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]hub.RAGSyncRecord, error) {
	if limit > len(m.pending) {
		return m.pending, nil
	}
	return m.pending[:limit], nil
}

func (m *mockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	for i := range m.pending {
		for _, id := range ids {
			if m.pending[i].ID == id {
				m.pending[i].SyncStatus = hub.SyncStatusSynced
			}
		}
	}
	return nil
}

func (m *mockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []hub.RAGSyncRecord) error {
	return nil
}

func TestRAGSyncService_Mock(t *testing.T) {
	mockService := &mockRAGSyncService{
		pending: []hub.RAGSyncRecord{
			{ID: "1", Context: "test 1", SyncStatus: hub.SyncStatusPending, LastSyncAt: time.Now()},
			{ID: "2", Context: "test 2", SyncStatus: hub.SyncStatusPending, LastSyncAt: time.Now()},
		},
	}

	ctx := context.Background()

	// Test FetchPendingSyncs
	pending, err := mockService.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(pending) != 2 {
		t.Fatalf("expected 2 pending records, got %d", len(pending))
	}

	// Test MarkSynced
	err = mockService.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if mockService.pending[0].SyncStatus != hub.SyncStatusSynced {
		t.Fatalf("expected record 1 to be synced")
	}
	if mockService.pending[1].SyncStatus != hub.SyncStatusPending {
		t.Fatalf("expected record 2 to still be pending")
	}

    // Test ProcessIncomingSync
    err = mockService.ProcessIncomingSync(ctx, pending)
    if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
}

func TestSyncDaemon(t *testing.T) {
    mockService := &mockRAGSyncService{
		pending: []hub.RAGSyncRecord{
			{ID: "1", Context: "test 1", SyncStatus: hub.SyncStatusPending, LastSyncAt: time.Now()},
		},
	}
    daemon := hub.NewSyncDaemon(mockService)

    ctx, cancel := context.WithTimeout(context.Background(), 2 * time.Second)
    defer cancel()

    go daemon.Start(ctx)
    <-ctx.Done()
}
