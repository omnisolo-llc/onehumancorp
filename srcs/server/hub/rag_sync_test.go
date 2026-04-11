package hub

import (
	"context"
	"testing"
	"time"
)

type MockRAGSyncService struct {
	PendingSyncs []RAGSyncRecord
	MarkedIDs    []string
	Processed    []RAGSyncRecord
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if limit > len(m.PendingSyncs) {
		return m.PendingSyncs, nil
	}
	return m.PendingSyncs[:limit], nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	m.MarkedIDs = append(m.MarkedIDs, ids...)
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	m.Processed = append(m.Processed, records...)
	return nil
}

func TestRAGSyncServiceFlow(t *testing.T) {
	mockService := &MockRAGSyncService{
		PendingSyncs: []RAGSyncRecord{
			{
				ID:         "mem_1",
				Context:    "Test context",
				Vector:     []float32{0.1, 0.2, 0.3},
				SyncStatus: SyncStatusPending,
				LastSyncAt: time.Now(),
			},
		},
	}

	ctx := context.Background()

	// 1. Fetch pending syncs
	records, err := mockService.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error fetching pending syncs: %v", err)
	}

	if len(records) != 1 {
		t.Fatalf("expected 1 pending record, got %d", len(records))
	}

	if records[0].ID != "mem_1" {
		t.Errorf("expected ID mem_1, got %s", records[0].ID)
	}

	// 2. Process incoming sync
	err = mockService.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("unexpected error processing incoming sync: %v", err)
	}

	if len(mockService.Processed) != 1 {
		t.Fatalf("expected 1 processed record, got %d", len(mockService.Processed))
	}

	// 3. Mark synced
	ids := []string{records[0].ID}
	err = mockService.MarkSynced(ctx, ids)
	if err != nil {
		t.Fatalf("unexpected error marking synced: %v", err)
	}

	if len(mockService.MarkedIDs) != 1 || mockService.MarkedIDs[0] != "mem_1" {
		t.Errorf("expected MarkedIDs to contain mem_1, got %v", mockService.MarkedIDs)
	}
}
