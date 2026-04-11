package hub_test

import (
	"context"
	"errors"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/hub"
)

// MockRAGSyncService is a mock implementation of hub.RAGSyncService
type MockRAGSyncService struct {
	Records map[string]hub.RAGSyncRecord
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]hub.RAGSyncRecord, error) {
	var pending []hub.RAGSyncRecord
	for _, rec := range m.Records {
		if rec.SyncStatus == hub.SyncStatusPending {
			pending = append(pending, rec)
			if len(pending) == limit {
				break
			}
		}
	}
	return pending, nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	for _, id := range ids {
		if rec, ok := m.Records[id]; ok {
			rec.SyncStatus = hub.SyncStatusSynced
			rec.LastSyncAt = time.Now()
			m.Records[id] = rec
		} else {
            return errors.New("record not found")
        }
	}
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []hub.RAGSyncRecord) error {
	for _, rec := range records {
		rec.SyncStatus = hub.SyncStatusSynced
        rec.LastSyncAt = time.Now()
		m.Records[rec.ID] = rec
	}
	return nil
}

func TestFetchPendingSyncs(t *testing.T) {
	mock := &MockRAGSyncService{
		Records: map[string]hub.RAGSyncRecord{
			"1": {ID: "1", Context: "test1", SyncStatus: hub.SyncStatusPending},
			"2": {ID: "2", Context: "test2", SyncStatus: hub.SyncStatusSynced},
		},
	}

	ctx := context.Background()
	pending, err := mock.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(pending) != 1 || pending[0].ID != "1" {
		t.Errorf("expected 1 pending record with ID 1, got %+v", pending)
	}
}

func TestMarkSynced(t *testing.T) {
	mock := &MockRAGSyncService{
		Records: map[string]hub.RAGSyncRecord{
			"1": {ID: "1", Context: "test1", SyncStatus: hub.SyncStatusPending},
		},
	}

	ctx := context.Background()
	err := mock.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	rec := mock.Records["1"]
	if rec.SyncStatus != hub.SyncStatusSynced {
		t.Errorf("expected status synced, got %s", rec.SyncStatus)
	}
}

func TestProcessIncomingSync(t *testing.T) {
	mock := &MockRAGSyncService{
		Records: make(map[string]hub.RAGSyncRecord),
	}

	ctx := context.Background()
	records := []hub.RAGSyncRecord{
		{ID: "1", Context: "incoming1"},
	}

	err := mock.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	rec, ok := mock.Records["1"]
	if !ok || rec.SyncStatus != hub.SyncStatusSynced {
		t.Errorf("expected record 1 to be saved and marked synced")
	}
}
