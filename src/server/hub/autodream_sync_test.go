package hub_test

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/src/server/hub"
)

type mockAutoDreamSyncService struct {
	records []*hub.AutoDreamSyncRecord
}

func (m *mockAutoDreamSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]*hub.AutoDreamSyncRecord, error) {
	var pending []*hub.AutoDreamSyncRecord
	for _, r := range m.records {
		if r.SyncStatus == "pending" {
			pending = append(pending, r)
		}
	}
	if len(pending) > limit {
		return pending[:limit], nil
	}
	return pending, nil
}

func (m *mockAutoDreamSyncService) ProcessIncomingSync(ctx context.Context, payload *hub.AutoDreamSyncRecord) error {
	m.records = append(m.records, payload)
	return nil
}

func (m *mockAutoDreamSyncService) MarkRecordSynced(ctx context.Context, recordID string) error {
	for _, r := range m.records {
		if r.ID == recordID {
			r.SyncStatus = "synced"
			now := time.Now()
			r.LastSyncAt = &now
		}
	}
	return nil
}

func TestAutoDreamSyncService_FetchPendingSyncs(t *testing.T) {
	svc := &mockAutoDreamSyncService{
		records: []*hub.AutoDreamSyncRecord{
			{ID: "1", SyncStatus: "pending"},
			{ID: "2", SyncStatus: "synced"},
			{ID: "3", SyncStatus: "pending"},
		},
	}

	pending, err := svc.FetchPendingSyncs(context.Background(), 10)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(pending) != 2 {
		t.Fatalf("expected 2 pending records, got %d", len(pending))
	}
}

func TestAutoDreamSyncService_ProcessIncomingSync(t *testing.T) {
	svc := &mockAutoDreamSyncService{}
	record := &hub.AutoDreamSyncRecord{ID: "1", SyncStatus: "pending"}

	err := svc.ProcessIncomingSync(context.Background(), record)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if len(svc.records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(svc.records))
	}
}

func TestAutoDreamSyncService_MarkRecordSynced(t *testing.T) {
	svc := &mockAutoDreamSyncService{
		records: []*hub.AutoDreamSyncRecord{
			{ID: "1", SyncStatus: "pending"},
		},
	}

	err := svc.MarkRecordSynced(context.Background(), "1")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if svc.records[0].SyncStatus != "synced" {
		t.Fatalf("expected sync_status to be 'synced', got %s", svc.records[0].SyncStatus)
	}
	if svc.records[0].LastSyncAt == nil {
		t.Fatalf("expected last_sync_at to be set")
	}
}
