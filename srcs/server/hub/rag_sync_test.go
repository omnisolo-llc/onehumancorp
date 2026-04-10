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
	for _, inRec := range records {
		found := false
		for i, rec := range m.records {
			if rec.ID == inRec.ID {
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

func TestRAGSyncService(t *testing.T) {
	mock := &mockRAGSyncService{
		records: []RAGSyncRecord{
			{ID: "1", Context: "test 1", SyncStatus: SyncStatusPending},
			{ID: "2", Context: "test 2", SyncStatus: SyncStatusSynced},
			{ID: "3", Context: "test 3", SyncStatus: SyncStatusPending},
		},
	}

	ctx := context.Background()

	// Fetch Pending
	pending, err := mock.FetchPendingSyncs(ctx, 2)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(pending) != 2 {
		t.Fatalf("expected 2 pending records, got %d", len(pending))
	}

	// Mark Synced
	err = mock.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	// Fetch Pending again
	pending, err = mock.FetchPendingSyncs(ctx, 2)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(pending) != 1 || pending[0].ID != "3" {
		t.Fatalf("expected 1 pending record with ID 3, got %v", pending)
	}

	// Process Incoming Sync
	incoming := []RAGSyncRecord{
		{ID: "4", Context: "test 4", SyncStatus: SyncStatusSynced},
	}
	err = mock.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(mock.records) != 4 {
		t.Fatalf("expected 4 records, got %d", len(mock.records))
	}
}
