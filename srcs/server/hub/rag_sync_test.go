package hub

import (
	"context"
	"testing"
	"time"
)

type MockRAGSyncService struct {
	PendingRecords []RAGSyncRecord
	MarkedSynced   []string
	ProcessedSyncs []RAGSyncRecord
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if limit > len(m.PendingRecords) {
		return m.PendingRecords, nil
	}
	return m.PendingRecords[:limit], nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	m.MarkedSynced = append(m.MarkedSynced, ids...)
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	m.ProcessedSyncs = append(m.ProcessedSyncs, records...)
	return nil
}

func TestRAGSyncService_FetchPendingSyncs(t *testing.T) {
	service := &MockRAGSyncService{
		PendingRecords: []RAGSyncRecord{
			{ID: "1", Context: "test 1", SyncStatus: SyncStatusPending, LastSyncAt: time.Now()},
			{ID: "2", Context: "test 2", SyncStatus: SyncStatusPending, LastSyncAt: time.Now()},
		},
	}

	records, err := service.FetchPendingSyncs(context.Background(), 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(records) != 2 {
		t.Errorf("expected 2 records, got %d", len(records))
	}
}

func TestRAGSyncService_MarkSynced(t *testing.T) {
	service := &MockRAGSyncService{}

	err := service.MarkSynced(context.Background(), []string{"1", "2"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(service.MarkedSynced) != 2 {
		t.Errorf("expected 2 marked records, got %d", len(service.MarkedSynced))
	}
}

func TestRAGSyncService_ProcessIncomingSync(t *testing.T) {
	service := &MockRAGSyncService{}

	records := []RAGSyncRecord{
		{ID: "1", Context: "test 1", SyncStatus: SyncStatusPending, LastSyncAt: time.Now()},
	}

	err := service.ProcessIncomingSync(context.Background(), records)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(service.ProcessedSyncs) != 1 {
		t.Errorf("expected 1 processed record, got %d", len(service.ProcessedSyncs))
	}
}
