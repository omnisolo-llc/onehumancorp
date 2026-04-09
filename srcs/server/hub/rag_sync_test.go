package hub_test

import (
	"context"
	"errors"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/hub"
)

type MockRAGSyncService struct {
	records []hub.RAGSyncRecord
	synced  []string
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]hub.RAGSyncRecord, error) {
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

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	idMap := make(map[string]bool)
	for _, id := range ids {
		idMap[id] = true
	}
	for i, r := range m.records {
		if idMap[r.ID] {
			m.records[i].SyncStatus = hub.SyncStatusSynced
			m.records[i].LastSyncAt = time.Now()
			m.synced = append(m.synced, r.ID)
		}
	}
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []hub.RAGSyncRecord) error {
	if len(records) == 0 {
		return errors.New("empty records")
	}
	m.records = append(m.records, records...)
	return nil
}

func TestRAGSyncServiceInterface(t *testing.T) {
	service := &MockRAGSyncService{
		records: []hub.RAGSyncRecord{
			{ID: "1", Context: "test 1", SyncStatus: hub.SyncStatusPending},
			{ID: "2", Context: "test 2", SyncStatus: hub.SyncStatusSynced},
			{ID: "3", Context: "test 3", SyncStatus: hub.SyncStatusPending},
		},
	}

	// Test FetchPendingSyncs
	pending, err := service.FetchPendingSyncs(context.Background(), 2)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(pending) != 2 {
		t.Fatalf("expected 2 pending records, got %d", len(pending))
	}

	// Test MarkSynced
	err = service.MarkSynced(context.Background(), []string{"1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if service.records[0].SyncStatus != hub.SyncStatusSynced {
		t.Errorf("expected record 1 to be synced")
	}

	// Test ProcessIncomingSync
	err = service.ProcessIncomingSync(context.Background(), []hub.RAGSyncRecord{
		{ID: "4", Context: "test 4", SyncStatus: hub.SyncStatusSynced},
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(service.records) != 4 {
		t.Errorf("expected 4 total records, got %d", len(service.records))
	}
}
