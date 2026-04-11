package orchestration

import (
	"context"
	"testing"
)

type mockRAGSyncService struct {
	records []RAGSyncRecord
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
	for i, r := range m.records {
		for _, id := range ids {
			if r.ID == id {
				m.records[i].SyncStatus = SyncStatusSynced
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

func TestRAGSyncService(t *testing.T) {
	mock := &mockRAGSyncService{}
	ctx := context.Background()

	// Initial push to cloud
	incoming := []RAGSyncRecord{
		{ID: "m1", Context: "context 1", SyncStatus: SyncStatusPending},
		{ID: "m2", Context: "context 2", SyncStatus: SyncStatusPending},
	}

	err := mock.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}
	if len(mock.records) != 2 {
		t.Fatalf("Expected 2 records, got %d", len(mock.records))
	}

	// Add a pending sync to local mock state manually to test fetch
	mock.records = append(mock.records, RAGSyncRecord{ID: "m3", Context: "context 3", SyncStatus: SyncStatusPending})

	pending, err := mock.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 1 {
		t.Fatalf("Expected 1 pending record, got %d", len(pending))
	}
	if pending[0].ID != "m3" {
		t.Fatalf("Expected m3, got %s", pending[0].ID)
	}

	err = mock.MarkSynced(ctx, []string{"m3"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	pendingAfter, err := mock.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pendingAfter) != 0 {
		t.Fatalf("Expected 0 pending records after sync, got %d", len(pendingAfter))
	}
}
