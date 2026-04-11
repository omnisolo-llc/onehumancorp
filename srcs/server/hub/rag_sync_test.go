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
	for _, id := range ids {
		for i, r := range m.records {
			if r.ID == id {
				m.records[i].SyncStatus = SyncStatusSynced
				m.records[i].LastSyncAt = time.Now()
			}
		}
	}
	return nil
}

func (m *mockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	for _, inRec := range records {
		found := false
		for i, r := range m.records {
			if r.ID == inRec.ID {
				m.records[i] = inRec
				found = true
				break
			}
		}
		if !found {
			m.records = append(m.records, inRec)
		}
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
		t.Errorf("expected 2 records, got %d", len(records))
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
		t.Errorf("expected status to be synced, got %s", mock.records[0].SyncStatus)
	}
	if mock.records[0].LastSyncAt.IsZero() {
		t.Errorf("expected last sync time to be set")
	}
}

func TestProcessIncomingSync(t *testing.T) {
	mock := &mockRAGSyncService{
		records: []RAGSyncRecord{},
	}

	err := mock.ProcessIncomingSync(context.Background(), []RAGSyncRecord{
		{ID: "1", SyncStatus: SyncStatusPending},
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(mock.records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(mock.records))
	}
	if mock.records[0].ID != "1" {
		t.Errorf("expected ID 1, got %s", mock.records[0].ID)
	}
}
