package hub

import (
	"context"
	"testing"
	"time"
)

type MockRAGSyncService struct {
	pendingRecords []RAGSyncRecord
	syncedIDs      []string
	incomingSyncs  []RAGSyncRecord
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if limit > len(m.pendingRecords) {
		return m.pendingRecords, nil
	}
	return m.pendingRecords[:limit], nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	m.syncedIDs = append(m.syncedIDs, ids...)
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	m.incomingSyncs = append(m.incomingSyncs, records...)
	return nil
}

func TestFetchPendingSyncs(t *testing.T) {
	mockService := &MockRAGSyncService{
		pendingRecords: []RAGSyncRecord{
			{ID: "1", Context: "test1", SyncStatus: SyncStatusPending},
			{ID: "2", Context: "test2", SyncStatus: SyncStatusPending},
		},
	}

	ctx := context.Background()
	records, err := mockService.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}

	if len(records) != 2 {
		t.Errorf("Expected 2 records, got %d", len(records))
	}
}

func TestMarkSynced(t *testing.T) {
	mockService := &MockRAGSyncService{}
	ctx := context.Background()

	idsToSync := []string{"1", "2"}
	err := mockService.MarkSynced(ctx, idsToSync)
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}

	if len(mockService.syncedIDs) != 2 {
		t.Errorf("Expected 2 synced IDs, got %d", len(mockService.syncedIDs))
	}
}

func TestProcessIncomingSync(t *testing.T) {
	mockService := &MockRAGSyncService{}
	ctx := context.Background()

	records := []RAGSyncRecord{
		{ID: "1", Context: "test1", SyncStatus: SyncStatusSynced, LastSyncAt: time.Now()},
	}

	err := mockService.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}

	if len(mockService.incomingSyncs) != 1 {
		t.Errorf("Expected 1 incoming sync, got %d", len(mockService.incomingSyncs))
	}
}
