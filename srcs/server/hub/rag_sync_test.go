package hub

import (
	"context"
	"testing"
	"time"
)

type mockRAGSyncService struct {
	pendingRecords []RAGSyncRecord
	syncedIDs      []string
	incomingRecords []RAGSyncRecord
}

func (m *mockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if len(m.pendingRecords) > limit {
		return m.pendingRecords[:limit], nil
	}
	return m.pendingRecords, nil
}

func (m *mockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	m.syncedIDs = append(m.syncedIDs, ids...)
	return nil
}

func (m *mockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	m.incomingRecords = append(m.incomingRecords, records...)
	return nil
}

func TestFetchPendingSyncs(t *testing.T) {
	mockService := &mockRAGSyncService{
		pendingRecords: []RAGSyncRecord{
			{ID: "1", Context: "test1", SyncStatus: SyncStatusPending},
			{ID: "2", Context: "test2", SyncStatus: SyncStatusPending},
		},
	}

	records, err := mockService.FetchPendingSyncs(context.Background(), 1)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}
	if records[0].ID != "1" {
		t.Errorf("expected ID 1, got %s", records[0].ID)
	}
}

func TestMarkSynced(t *testing.T) {
	mockService := &mockRAGSyncService{}
	err := mockService.MarkSynced(context.Background(), []string{"1", "2"})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(mockService.syncedIDs) != 2 {
		t.Fatalf("expected 2 synced IDs, got %d", len(mockService.syncedIDs))
	}
}

func TestProcessIncomingSync(t *testing.T) {
	mockService := &mockRAGSyncService{}
	records := []RAGSyncRecord{
		{ID: "1", Context: "test1", SyncStatus: SyncStatusSynced, LastSyncAt: time.Now()},
	}
	err := mockService.ProcessIncomingSync(context.Background(), records)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(mockService.incomingRecords) != 1 {
		t.Fatalf("expected 1 incoming record, got %d", len(mockService.incomingRecords))
	}
}
