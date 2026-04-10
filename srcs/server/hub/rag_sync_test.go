package hub

import (
	"context"
	"testing"
	"time"
)

type MockRAGSyncService struct {
	PendingRecords []RAGSyncRecord
	MarkedIDs      []string
	ProcessedData  []RAGSyncRecord
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if limit < len(m.PendingRecords) {
		return m.PendingRecords[:limit], nil
	}
	return m.PendingRecords, nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	m.MarkedIDs = append(m.MarkedIDs, ids...)
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	m.ProcessedData = append(m.ProcessedData, records...)
	return nil
}

func TestRAGSyncServiceInterface(t *testing.T) {
	mockService := &MockRAGSyncService{
		PendingRecords: []RAGSyncRecord{
			{
				ID:         "mem1",
				Context:    "Test context",
				SyncStatus: SyncStatusPending,
			},
		},
	}

	ctx := context.Background()

	// Test FetchPendingSyncs
	records, err := mockService.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}

	// Test MarkSynced
	err = mockService.MarkSynced(ctx, []string{"mem1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(mockService.MarkedIDs) != 1 || mockService.MarkedIDs[0] != "mem1" {
		t.Fatalf("expected ID mem1 to be marked as synced")
	}

	// Test ProcessIncomingSync
	err = mockService.ProcessIncomingSync(ctx, []RAGSyncRecord{
		{
			ID:         "mem2",
			Context:    "Incoming context",
			SyncStatus: SyncStatusPending,
			LastSyncAt: time.Now(),
		},
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(mockService.ProcessedData) != 1 || mockService.ProcessedData[0].ID != "mem2" {
		t.Fatalf("expected record mem2 to be processed")
	}
}
