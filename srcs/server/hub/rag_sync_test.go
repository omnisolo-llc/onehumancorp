package hub

import (
	"context"
	"testing"
	"time"
)

type MockRAGSyncService struct {
	PendingSyncs []RAGSyncRecord
	MarkedIDs    []string
	IncomingSync []RAGSyncRecord
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
	m.IncomingSync = append(m.IncomingSync, records...)
	return nil
}

func TestRAGSyncServiceMock(t *testing.T) {
	mockService := &MockRAGSyncService{
		PendingSyncs: []RAGSyncRecord{
			{
				ID:         "rec-1",
				Context:    "Sample context",
				Vector:     []float32{0.1, 0.2, 0.3},
				SyncStatus: SyncStatusPending,
				LastSyncAt: time.Now(),
			},
		},
	}

	ctx := context.Background()

	records, err := mockService.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error fetching pending syncs: %v", err)
	}

	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}

	err = mockService.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("unexpected error processing incoming syncs: %v", err)
	}

	if len(mockService.IncomingSync) != 1 {
		t.Fatalf("expected 1 incoming sync, got %d", len(mockService.IncomingSync))
	}

	err = mockService.MarkSynced(ctx, []string{records[0].ID})
	if err != nil {
		t.Fatalf("unexpected error marking synced: %v", err)
	}

	if len(mockService.MarkedIDs) != 1 || mockService.MarkedIDs[0] != records[0].ID {
		t.Fatalf("expected marked ID to be %s", records[0].ID)
	}
}
