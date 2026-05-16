package hub

import (
	"context"
	"testing"
	"time"
)

type mockRAGSyncService struct {
	records []RAGSyncRecord
	synced  map[string]bool
}

func (m *mockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	var pending []RAGSyncRecord
	for _, r := range m.records {
		if r.SyncStatus == SyncStatusPending {
			pending = append(pending, r)
			if len(pending) == limit {
				break
			}
		}
	}
	return pending, nil
}

func (m *mockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	for _, id := range ids {
		m.synced[id] = true
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
	// Simple mock: just add or update records
	for _, nr := range records {
		found := false
		for i, er := range m.records {
			if er.ID == nr.ID {
				m.records[i] = nr
				found = true
				break
			}
		}
		if !found {
			m.records = append(m.records, nr)
		}
	}
	return nil
}

func TestRAGSyncFlow(t *testing.T) {
	mock := &mockRAGSyncService{
		records: []RAGSyncRecord{
			{ID: "1", Context: "test 1", SyncStatus: SyncStatusPending},
			{ID: "2", Context: "test 2", SyncStatus: SyncStatusPending},
		},
		synced: make(map[string]bool),
	}

	ctx := context.Background()

	pending, err := mock.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}

	if len(pending) != 2 {
		t.Errorf("Expected 2 pending records, got %d", len(pending))
	}

	ids := []string{"1", "2"}
	err = mock.MarkSynced(ctx, ids)
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	if !mock.synced["1"] || !mock.synced["2"] {
		t.Errorf("Records were not marked as synced")
	}

	pendingAfter, _ := mock.FetchPendingSyncs(ctx, 10)
	if len(pendingAfter) != 0 {
		t.Errorf("Expected 0 pending records after MarkSynced, got %d", len(pendingAfter))
	}

	newRecords := []RAGSyncRecord{
		{ID: "3", Context: "test 3", SyncStatus: SyncStatusSynced},
	}
	err = mock.ProcessIncomingSync(ctx, newRecords)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	if len(mock.records) != 3 {
		t.Errorf("Expected 3 total records, got %d", len(mock.records))
	}
}
