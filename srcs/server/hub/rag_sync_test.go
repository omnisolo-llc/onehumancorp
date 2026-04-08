package hub_test

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/hub"
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

func TestFetchPendingSyncs(t *testing.T) {
	svc := &mockRAGSyncService{
		records: []hub.RAGSyncRecord{
			{ID: "1", SyncStatus: hub.SyncStatusPending},
			{ID: "2", SyncStatus: hub.SyncStatusSynced},
			{ID: "3", SyncStatus: hub.SyncStatusPending},
		},
	}

	res, err := svc.FetchPendingSyncs(context.Background(), 1)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(res) != 1 {
		t.Fatalf("expected 1 record, got %d", len(res))
	}
	if res[0].ID != "1" {
		t.Errorf("expected ID 1, got %s", res[0].ID)
	}
}

func TestMarkSynced(t *testing.T) {
	svc := &mockRAGSyncService{
		records: []hub.RAGSyncRecord{
			{ID: "1", SyncStatus: hub.SyncStatusPending},
		},
	}

	err := svc.MarkSynced(context.Background(), []string{"1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if svc.records[0].SyncStatus != hub.SyncStatusSynced {
		t.Errorf("expected status synced, got %s", svc.records[0].SyncStatus)
	}
}

func TestProcessIncomingSync(t *testing.T) {
	svc := &mockRAGSyncService{}
	err := svc.ProcessIncomingSync(context.Background(), []hub.RAGSyncRecord{
		{ID: "1", SyncStatus: hub.SyncStatusSynced},
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(svc.records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(svc.records))
	}
}
