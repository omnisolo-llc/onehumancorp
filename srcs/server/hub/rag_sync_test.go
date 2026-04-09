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
	var pending []RAGSyncRecord
	for _, r := range m.records {
		if r.SyncStatus == SyncStatusPending {
			pending = append(pending, r)
		}
		if len(pending) == limit {
			break
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
			m.records[i].SyncStatus = SyncStatusSynced
			m.records[i].LastSyncAt = time.Now()
		}
	}
	return nil
}

func (m *mockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return errors.New("no records to sync")
	}
	m.records = append(m.records, records...)
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

	pending, err := mock.FetchPendingSyncs(context.Background(), 10)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(pending) != 2 {
		t.Fatalf("expected 2 pending records, got %d", len(pending))
	}
}

func TestMarkSynced(t *testing.T) {
	mock := &mockRAGSyncService{
		records: []RAGSyncRecord{
			{ID: "1", SyncStatus: SyncStatusPending},
			{ID: "2", SyncStatus: SyncStatusPending},
		},
	}

	err := mock.MarkSynced(context.Background(), []string{"1"})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if mock.records[0].SyncStatus != SyncStatusSynced {
		t.Errorf("expected record 1 to be synced")
	}
	if mock.records[1].SyncStatus != SyncStatusPending {
		t.Errorf("expected record 2 to be pending")
	}
}

func TestProcessIncomingSync(t *testing.T) {
	mock := &mockRAGSyncService{}
	records := []RAGSyncRecord{
		{ID: "1", Context: "test"},
	}

	err := mock.ProcessIncomingSync(context.Background(), records)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(mock.records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(mock.records))
	}
}
