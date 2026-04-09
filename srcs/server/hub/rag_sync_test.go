package hub

import (
	"context"
	"testing"
	"time"
)

type mockRAGSyncService struct {
	pendingRecords   []RAGSyncRecord
	syncedIDs        []string
	processedRecords []RAGSyncRecord
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
	m.processedRecords = append(m.processedRecords, records...)
	return nil
}

func TestMockRAGSyncService(t *testing.T) {
	svc := &mockRAGSyncService{
		pendingRecords: []RAGSyncRecord{
			{ID: "1", Context: "test 1", SyncStatus: SyncStatusPending},
			{ID: "2", Context: "test 2", SyncStatus: SyncStatusPending},
		},
	}

	ctx := context.Background()

	records, err := svc.FetchPendingSyncs(ctx, 1)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(records) != 1 {
		t.Errorf("expected 1 record, got %d", len(records))
	}

	err = svc.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(svc.syncedIDs) != 1 || svc.syncedIDs[0] != "1" {
		t.Errorf("expected synced ID '1', got %v", svc.syncedIDs)
	}

	err = svc.ProcessIncomingSync(ctx, []RAGSyncRecord{{ID: "3", Context: "test 3", SyncStatus: SyncStatusSynced, LastSyncAt: time.Now()}})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(svc.processedRecords) != 1 {
		t.Errorf("expected 1 processed record, got %d", len(svc.processedRecords))
	}
}
