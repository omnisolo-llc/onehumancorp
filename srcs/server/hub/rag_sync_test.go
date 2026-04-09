package hub_test

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/hub"
)

type mockSyncService struct {
	records []hub.RAGSyncRecord
}

func (m *mockSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]hub.RAGSyncRecord, error) {
	var pending []hub.RAGSyncRecord
	for _, r := range m.records {
		if r.SyncStatus == hub.SyncStatusPending {
			pending = append(pending, r)
		}
	}
	if len(pending) > limit {
		pending = pending[:limit]
	}
	return pending, nil
}

func (m *mockSyncService) MarkSynced(ctx context.Context, ids []string) error {
	idMap := make(map[string]bool)
	for _, id := range ids {
		idMap[id] = true
	}
	for i := range m.records {
		if idMap[m.records[i].ID] {
			m.records[i].SyncStatus = hub.SyncStatusSynced
			m.records[i].LastSyncAt = time.Now()
		}
	}
	return nil
}

func (m *mockSyncService) ProcessIncomingSync(ctx context.Context, records []hub.RAGSyncRecord) error {
	for _, r := range records {
		found := false
		for i := range m.records {
			if m.records[i].ID == r.ID {
				m.records[i] = r
				found = true
				break
			}
		}
		if !found {
			m.records = append(m.records, r)
		}
	}
	return nil
}

func TestRAGSyncServiceFlow(t *testing.T) {
	mock := &mockSyncService{
		records: []hub.RAGSyncRecord{
			{ID: "1", Context: "test1", SyncStatus: hub.SyncStatusPending},
			{ID: "2", Context: "test2", SyncStatus: hub.SyncStatusPending},
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
	ids := []string{"1"}
	err = mock.MarkSynced(ctx, ids)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	pending, err = mock.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(pending) != 1 {
		t.Errorf("expected 1 pending record after marking one synced, got %d", len(pending))
	}

	// Test ProcessIncomingSync
	newRecords := []hub.RAGSyncRecord{
		{ID: "3", Context: "test3", SyncStatus: hub.SyncStatusSynced, LastSyncAt: time.Now()},
	}
	err = mock.ProcessIncomingSync(ctx, newRecords)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(mock.records) != 3 {
		t.Errorf("expected 3 records after process, got %d", len(mock.records))
	}
}
