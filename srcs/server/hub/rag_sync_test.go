package hub_test

import (
	"context"
	"testing"
	"time"

	"ohc/srcs/server/hub"
)

type mockRAGSyncService struct {
	records []hub.RAGSyncRecord
}

func (m *mockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]hub.RAGSyncRecord, error) {
	var pending []hub.RAGSyncRecord
	for _, r := range m.records {
		if r.SyncStatus == hub.SyncStatusPending {
			pending = append(pending, r)
			if len(pending) == limit {
				break
			}
		}
	}
	return pending, nil
}

func (m *mockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	idMap := make(map[string]bool)
	for _, id := range ids {
		idMap[id] = true
	}

	for i, r := range m.records {
		if idMap[r.ID] {
			m.records[i].SyncStatus = hub.SyncStatusSynced
			m.records[i].LastSyncAt = time.Now()
		}
	}
	return nil
}

func (m *mockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []hub.RAGSyncRecord) error {
	m.records = append(m.records, records...)
	return nil
}

func TestRAGSyncService(t *testing.T) {
	mock := &mockRAGSyncService{
		records: []hub.RAGSyncRecord{
			{ID: "1", Context: "test 1", SyncStatus: hub.SyncStatusPending},
			{ID: "2", Context: "test 2", SyncStatus: hub.SyncStatusSynced},
			{ID: "3", Context: "test 3", SyncStatus: hub.SyncStatusPending},
		},
	}

	ctx := context.Background()

	// Test FetchPendingSyncs
	pending, err := mock.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(pending) != 2 {
		t.Errorf("expected 2 pending records, got %d", len(pending))
	}

	// Test MarkSynced
	err = mock.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	pending, _ = mock.FetchPendingSyncs(ctx, 10)
	if len(pending) != 1 {
		t.Errorf("expected 1 pending record after marking 1 synced, got %d", len(pending))
	}

	// Test ProcessIncomingSync
	newRecords := []hub.RAGSyncRecord{
		{ID: "4", Context: "test 4", SyncStatus: hub.SyncStatusPending},
	}
	err = mock.ProcessIncomingSync(ctx, newRecords)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	pending, _ = mock.FetchPendingSyncs(ctx, 10)
	if len(pending) != 2 {
		t.Errorf("expected 2 pending records after processing 1 incoming, got %d", len(pending))
	}
}
