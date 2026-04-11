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
	// For testing, just append the incoming records
	m.records = append(m.records, records...)
	return nil
}

func TestRAGSyncService(t *testing.T) {
	ctx := context.Background()
	svc := &mockRAGSyncService{
		records: []RAGSyncRecord{
			{ID: "1", Context: "test context 1", SyncStatus: SyncStatusPending},
			{ID: "2", Context: "test context 2", SyncStatus: SyncStatusSynced},
			{ID: "3", Context: "test context 3", SyncStatus: SyncStatusPending},
		},
	}

	pending, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(pending) != 2 {
		t.Fatalf("expected 2 pending records, got %d", len(pending))
	}

	ids := []string{"1"}
	err = svc.MarkSynced(ctx, ids)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	pendingAfter, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(pendingAfter) != 1 {
		t.Fatalf("expected 1 pending record, got %d", len(pendingAfter))
	}

	err = svc.ProcessIncomingSync(ctx, []RAGSyncRecord{
		{ID: "4", Context: "incoming test context", SyncStatus: SyncStatusSynced},
	})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if len(svc.records) != 4 {
		t.Fatalf("expected 4 records after process incoming, got %d", len(svc.records))
	}
}
