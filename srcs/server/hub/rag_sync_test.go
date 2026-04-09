package hub

import (
	"context"
	"errors"
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
			if len(result) == limit {
				break
			}
		}
	}
	return result, nil
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
			RecordSyncSuccess(ctx, 1)
		}
	}
	return nil
}

func (m *mockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	for _, r := range records {
		if r.Context == "error" {
			RecordSyncError(ctx)
			return errors.New("simulated sync error")
		}
		// Upsert logic mock
		m.records = append(m.records, r)
		RecordSyncSuccess(ctx, 1)
	}
	return nil
}

func TestFetchPendingSyncs(t *testing.T) {
	mock := &mockRAGSyncService{
		records: []RAGSyncRecord{
			{ID: "1", SyncStatus: SyncStatusPending},
			{ID: "2", SyncStatus: SyncStatusSynced},
			{ID: "3", SyncStatus: SyncStatusPending},
		},
	}

	records, err := mock.FetchPendingSyncs(context.Background(), 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(records) != 2 {
		t.Errorf("expected 2 pending records, got %d", len(records))
	}
}

func TestMarkSynced(t *testing.T) {
	mock := &mockRAGSyncService{
		records: []RAGSyncRecord{
			{ID: "1", SyncStatus: SyncStatusPending},
		},
	}

	err := mock.MarkSynced(context.Background(), []string{"1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if mock.records[0].SyncStatus != SyncStatusSynced {
		t.Errorf("expected status synced, got %v", mock.records[0].SyncStatus)
	}
}

func TestProcessIncomingSync(t *testing.T) {
	mock := &mockRAGSyncService{}

	err := mock.ProcessIncomingSync(context.Background(), []RAGSyncRecord{
		{ID: "1", SyncStatus: SyncStatusSynced},
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(mock.records) != 1 {
		t.Errorf("expected 1 record to be processed, got %d", len(mock.records))
	}

	err = mock.ProcessIncomingSync(context.Background(), []RAGSyncRecord{
		{ID: "2", Context: "error"},
	})
	if err == nil {
		t.Errorf("expected error, got nil")
	}
}
