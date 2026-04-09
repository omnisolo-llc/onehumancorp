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
	var result []RAGSyncRecord
	for _, r := range m.records {
		if r.SyncStatus == SyncStatusPending {
			result = append(result, r)
			if len(result) >= limit {
				break
			}
		}
	}
	return result, nil
}

func (m *mockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	for i, r := range m.records {
		for _, id := range ids {
			if r.ID == id {
				m.records[i].SyncStatus = SyncStatusSynced
				m.records[i].LastSyncAt = time.Now()
			}
		}
	}
	return nil
}

func (m *mockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	for _, r := range records {
		r.SyncStatus = SyncStatusSynced
		m.records = append(m.records, r)
	}
	return nil
}

func TestMockRAGSyncService(t *testing.T) {
	mock := &mockRAGSyncService{
		records: []RAGSyncRecord{
			{ID: "1", Context: "test 1", SyncStatus: SyncStatusPending},
			{ID: "2", Context: "test 2", SyncStatus: SyncStatusPending},
			{ID: "3", Context: "test 3", SyncStatus: SyncStatusSynced},
		},
	}

	ctx := context.Background()

	pending, err := mock.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(pending) != 2 {
		t.Errorf("expected 2 pending records, got %d", len(pending))
	}

	err = mock.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	pending, err = mock.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(pending) != 1 {
		t.Errorf("expected 1 pending record, got %d", len(pending))
	}

	err = mock.ProcessIncomingSync(ctx, []RAGSyncRecord{
		{ID: "4", Context: "test 4", SyncStatus: SyncStatusPending},
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
}
