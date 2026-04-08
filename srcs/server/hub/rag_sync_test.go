package hub

import (
	"context"
	"testing"
	"time"
)

type mockRAGSyncService struct {
	records []RAGSyncRecord
}

func (m *mockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	var pending []RAGSyncRecord
	for _, r := range m.records {
		if r.SyncStatus == SyncStatusPending {
			pending = append(pending, r)
		}
	}
	if len(pending) > limit {
		return pending[:limit], nil
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
			m.records[i].SyncStatus = SyncStatusSynced
			m.records[i].LastSyncAt = time.Now()
		}
	}
	return nil
}

func (m *mockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	m.records = append(m.records, records...)
	return nil
}

func TestRAGSyncFlow(t *testing.T) {
	svc := &mockRAGSyncService{
		records: []RAGSyncRecord{
			{ID: "1", Context: "test1", Vector: "[0.1]", SyncStatus: SyncStatusPending},
			{ID: "2", Context: "test2", Vector: "[0.2]", SyncStatus: SyncStatusSynced},
		},
	}

	ctx := context.Background()

	pending, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(pending) != 1 {
		t.Fatalf("expected 1 pending record, got %d", len(pending))
	}

	err = svc.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	pendingAfter, _ := svc.FetchPendingSyncs(ctx, 10)
	if len(pendingAfter) != 0 {
		t.Fatalf("expected 0 pending records, got %d", len(pendingAfter))
	}

	err = svc.ProcessIncomingSync(ctx, []RAGSyncRecord{
		{ID: "3", Context: "test3", Vector: "[0.3]", SyncStatus: SyncStatusSynced},
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(svc.records) != 3 {
		t.Fatalf("expected 3 records, got %d", len(svc.records))
	}
}
