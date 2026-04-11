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
		pending = pending[:limit]
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

func TestMockRAGSyncService(t *testing.T) {
	service := &mockRAGSyncService{
		records: []RAGSyncRecord{
			{ID: "1", Context: "test 1", SyncStatus: SyncStatusPending},
			{ID: "2", Context: "test 2", SyncStatus: SyncStatusSynced},
			{ID: "3", Context: "test 3", SyncStatus: SyncStatusPending},
		},
	}

	ctx := context.Background()

	pending, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(pending) != 2 {
		t.Fatalf("expected 2 pending records, got %d", len(pending))
	}

	if pending[0].ID != "1" || pending[1].ID != "3" {
		t.Fatalf("unexpected pending records")
	}

	err = service.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	pending, err = service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(pending) != 1 {
		t.Fatalf("expected 1 pending record, got %d", len(pending))
	}

	if pending[0].ID != "3" {
		t.Fatalf("unexpected pending record")
	}

	err = service.ProcessIncomingSync(ctx, []RAGSyncRecord{
		{ID: "4", Context: "test 4", SyncStatus: SyncStatusPending},
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	pending, err = service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(pending) != 2 {
		t.Fatalf("expected 2 pending records, got %d", len(pending))
	}
}
