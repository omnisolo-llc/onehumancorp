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
	if len(m.records) > limit {
		return m.records[:limit], nil
	}
	return m.records, nil
}

func (m *mockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	for _, id := range ids {
		for i, rec := range m.records {
			if rec.ID == id {
				m.records[i].SyncStatus = SyncStatusSynced
			}
		}
	}
	return nil
}

func (m *mockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	m.records = append(m.records, records...)
	return nil
}

func TestFetchPendingSyncs(t *testing.T) {
	mock := &mockRAGSyncService{
		records: []RAGSyncRecord{
			{ID: "1", SyncStatus: SyncStatusPending},
			{ID: "2", SyncStatus: SyncStatusPending},
		},
	}
	ctx := context.Background()
	res, err := mock.FetchPendingSyncs(ctx, 1)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(res) != 1 {
		t.Fatalf("expected 1 record, got %d", len(res))
	}
}

func TestMarkSynced(t *testing.T) {
	mock := &mockRAGSyncService{
		records: []RAGSyncRecord{
			{ID: "1", SyncStatus: SyncStatusPending},
		},
	}
	ctx := context.Background()
	err := mock.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if mock.records[0].SyncStatus != SyncStatusSynced {
		t.Fatalf("expected synced status, got %v", mock.records[0].SyncStatus)
	}
}

func TestProcessIncomingSync(t *testing.T) {
	mock := &mockRAGSyncService{}
	ctx := context.Background()
	now := time.Now()
	err := mock.ProcessIncomingSync(ctx, []RAGSyncRecord{
		{ID: "1", Context: "test", SyncStatus: SyncStatusSynced, LastSyncAt: now},
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(mock.records) != 1 {
		t.Fatalf("expected 1 record added")
	}
}
