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
		}
	}
	if len(pending) > limit {
		return pending[:limit], nil
	}
	return pending, nil
}

func (m *mockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	for i := range m.records {
		for _, id := range ids {
			if m.records[i].ID == id {
				m.records[i].SyncStatus = SyncStatusSynced
				m.records[i].LastSyncAt = time.Now()
			}
		}
	}
	return nil
}

func (m *mockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	for _, r := range records {
		found := false
		for i := range m.records {
			if m.records[i].ID == r.ID {
				m.records[i] = r
				found = true
				break
			}
		}
		if !found {
			m.records = append(m.records, r)
		}
	}
	return nil
}

func TestRAGSyncService(t *testing.T) {
	mockService := &mockRAGSyncService{
		records: []RAGSyncRecord{
			{ID: "1", Context: "test context 1", SyncStatus: SyncStatusPending},
			{ID: "2", Context: "test context 2", SyncStatus: SyncStatusSynced},
			{ID: "3", Context: "test context 3", SyncStatus: SyncStatusPending},
		},
	}

	ctx := context.Background()

	// Test FetchPendingSyncs
	pending, err := mockService.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(pending) != 2 {
		t.Fatalf("expected 2 pending records, got %d", len(pending))
	}

	// Test MarkSynced
	err = mockService.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	pending, err = mockService.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(pending) != 1 {
		t.Fatalf("expected 1 pending record, got %d", len(pending))
	}

	// Test ProcessIncomingSync
	newRecord := RAGSyncRecord{ID: "4", Context: "test context 4", SyncStatus: SyncStatusPending}
	err = mockService.ProcessIncomingSync(ctx, []RAGSyncRecord{newRecord})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(mockService.records) != 4 {
		t.Fatalf("expected 4 records, got %d", len(mockService.records))
	}
}
