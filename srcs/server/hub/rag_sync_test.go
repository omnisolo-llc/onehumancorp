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
			if len(pending) == limit {
				break
			}
		}
	}
	return pending, nil
}

func (m *mockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	idMap := make(map[string]bool)
	for _, id := range ids {
		idMap[id] = true
	}
	for i := range m.records {
		if idMap[m.records[i].ID] {
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

func TestRAGSyncService(t *testing.T) {
	mockService := &mockRAGSyncService{
		records: []RAGSyncRecord{
			{ID: "r1", Context: "ctx1", SyncStatus: SyncStatusPending},
			{ID: "r2", Context: "ctx2", SyncStatus: SyncStatusSynced},
			{ID: "r3", Context: "ctx3", SyncStatus: SyncStatusPending},
		},
	}

	ctx := context.Background()

	pending, err := mockService.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(pending) != 2 {
		t.Fatalf("expected 2 pending records, got %d", len(pending))
	}

	err = mockService.MarkSynced(ctx, []string{"r1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	pendingAfterMark, err := mockService.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(pendingAfterMark) != 1 {
		t.Fatalf("expected 1 pending record, got %d", len(pendingAfterMark))
	}

	err = mockService.ProcessIncomingSync(ctx, []RAGSyncRecord{
		{ID: "r4", Context: "ctx4", SyncStatus: SyncStatusSynced, LastSyncAt: time.Now()},
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(mockService.records) != 4 {
		t.Fatalf("expected 4 total records, got %d", len(mockService.records))
	}
}
