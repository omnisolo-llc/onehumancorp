package hub

import (
	"context"
	"testing"
)

type MockRAGSyncService struct {
	PendingSyncs []RAGSyncRecord
	SyncedIDs    []string
	IncomingSync []RAGSyncRecord
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if limit < len(m.PendingSyncs) {
		return m.PendingSyncs[:limit], nil
	}
	return m.PendingSyncs, nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	m.SyncedIDs = append(m.SyncedIDs, ids...)
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	m.IncomingSync = append(m.IncomingSync, records...)
	return nil
}

func TestFetchPendingSyncs(t *testing.T) {
	service := &MockRAGSyncService{
		PendingSyncs: []RAGSyncRecord{
			{ID: "1", Context: "test 1", SyncStatus: SyncStatusPending},
			{ID: "2", Context: "test 2", SyncStatus: SyncStatusPending},
		},
	}

	records, err := service.FetchPendingSyncs(context.Background(), 10)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if len(records) != 2 {
		t.Fatalf("expected 2 records, got %d", len(records))
	}
}

func TestMarkSynced(t *testing.T) {
	service := &MockRAGSyncService{}

	err := service.MarkSynced(context.Background(), []string{"1", "2"})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if len(service.SyncedIDs) != 2 {
		t.Fatalf("expected 2 synced ids, got %d", len(service.SyncedIDs))
	}
}

func TestProcessIncomingSync(t *testing.T) {
	service := &MockRAGSyncService{}

	records := []RAGSyncRecord{
		{ID: "1", Context: "test 1", SyncStatus: SyncStatusPending},
	}

	err := service.ProcessIncomingSync(context.Background(), records)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if len(service.IncomingSync) != 1 {
		t.Fatalf("expected 1 incoming sync, got %d", len(service.IncomingSync))
	}
}
