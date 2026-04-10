package hub

import (
	"context"
	"testing"
	"time"
)

type mockRAGSyncService struct {
	pendingRecords []RAGSyncRecord
	syncedIDs      []string
	processed      []RAGSyncRecord
}

func (m *mockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if limit > len(m.pendingRecords) {
		return m.pendingRecords, nil
	}
	return m.pendingRecords[:limit], nil
}

func (m *mockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	m.syncedIDs = append(m.syncedIDs, ids...)
	return nil
}

func (m *mockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	m.processed = append(m.processed, records...)
	return nil
}

func TestRAGSyncService(t *testing.T) {
	ctx := context.Background()
	svc := &mockRAGSyncService{
		pendingRecords: []RAGSyncRecord{
			{ID: "1", Context: "test", SyncStatus: SyncStatusPending},
			{ID: "2", Context: "test2", SyncStatus: SyncStatusPending},
		},
	}

	records, err := svc.FetchPendingSyncs(ctx, 1)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(records) != 1 || records[0].ID != "1" {
		t.Fatalf("expected 1 record with ID 1, got %+v", records)
	}

	err = svc.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(svc.syncedIDs) != 1 || svc.syncedIDs[0] != "1" {
		t.Fatalf("expected 1 synced ID '1', got %v", svc.syncedIDs)
	}

	err = svc.ProcessIncomingSync(ctx, []RAGSyncRecord{{ID: "3", Context: "incoming", SyncStatus: SyncStatusSynced, LastSyncAt: time.Now()}})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(svc.processed) != 1 || svc.processed[0].ID != "3" {
		t.Fatalf("expected 1 processed record with ID 3, got %+v", svc.processed)
	}
}
