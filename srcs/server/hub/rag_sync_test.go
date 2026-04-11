package hub

import (
	"context"
	"testing"
	"time"
)

type MockRAGSyncService struct {
	pendingSyncs  []RAGSyncRecord
	syncedIDs     []string
	incomingSyncs []RAGSyncRecord
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if limit > len(m.pendingSyncs) {
		limit = len(m.pendingSyncs)
	}
	return m.pendingSyncs[:limit], nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	m.syncedIDs = append(m.syncedIDs, ids...)
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	m.incomingSyncs = append(m.incomingSyncs, records...)
	return nil
}

func TestRAGSyncService_FetchPendingSyncs(t *testing.T) {
	mockSvc := &MockRAGSyncService{
		pendingSyncs: []RAGSyncRecord{
			{ID: "1", SyncStatus: SyncStatusPending},
			{ID: "2", SyncStatus: SyncStatusPending},
		},
	}

	records, err := mockSvc.FetchPendingSyncs(context.Background(), 1)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(records) != 1 {
		t.Errorf("expected 1 record, got %d", len(records))
	}
}

func TestRAGSyncService_MarkSynced(t *testing.T) {
	mockSvc := &MockRAGSyncService{}
	err := mockSvc.MarkSynced(context.Background(), []string{"1", "2"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(mockSvc.syncedIDs) != 2 {
		t.Errorf("expected 2 synced IDs, got %d", len(mockSvc.syncedIDs))
	}
}

func TestRAGSyncService_ProcessIncomingSync(t *testing.T) {
	mockSvc := &MockRAGSyncService{}
	records := []RAGSyncRecord{
		{ID: "1", SyncStatus: SyncStatusSynced, LastSyncAt: time.Now()},
	}
	err := mockSvc.ProcessIncomingSync(context.Background(), records)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(mockSvc.incomingSyncs) != 1 {
		t.Errorf("expected 1 incoming sync record, got %d", len(mockSvc.incomingSyncs))
	}
}
