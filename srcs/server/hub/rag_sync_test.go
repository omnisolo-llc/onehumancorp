package hub_test

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/hub"
)

type MockRAGSyncService struct {
	PendingSyncs []hub.RAGSyncRecord
	SyncedIDs    []string
	Incoming     []hub.RAGSyncRecord
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]hub.RAGSyncRecord, error) {
	if limit < len(m.PendingSyncs) {
		return m.PendingSyncs[:limit], nil
	}
	return m.PendingSyncs, nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	m.SyncedIDs = append(m.SyncedIDs, ids...)
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []hub.RAGSyncRecord) error {
	m.Incoming = append(m.Incoming, records...)
	return nil
}

func TestMockRAGSyncService(t *testing.T) {
	mock := &MockRAGSyncService{
		PendingSyncs: []hub.RAGSyncRecord{
			{
				ID:         "1",
				Context:    "test context",
				Vector:     []float32{0.1, 0.2},
				SyncStatus: hub.SyncStatusPending,
				LastSyncAt: time.Now(),
			},
		},
	}

	ctx := context.Background()

	// Test FetchPendingSyncs
	pending, err := mock.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(pending) != 1 {
		t.Fatalf("expected 1 pending sync, got %d", len(pending))
	}

	// Test MarkSynced
	err = mock.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(mock.SyncedIDs) != 1 || mock.SyncedIDs[0] != "1" {
		t.Fatalf("expected SyncedIDs to contain '1', got %v", mock.SyncedIDs)
	}

	// Test ProcessIncomingSync
	incoming := []hub.RAGSyncRecord{
		{
			ID:         "2",
			Context:    "incoming context",
			Vector:     []float32{0.3, 0.4},
			SyncStatus: hub.SyncStatusSynced,
			LastSyncAt: time.Now(),
		},
	}
	err = mock.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(mock.Incoming) != 1 || mock.Incoming[0].ID != "2" {
		t.Fatalf("expected Incoming to contain record '2', got %v", mock.Incoming)
	}
}
