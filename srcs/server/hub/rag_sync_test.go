package hub

import (
	"context"
	"testing"
	"time"
)

type MockRAGSyncService struct {
	PendingRecords []RAGSyncRecord
	MarkedIDs      []string
	Incoming       []RAGSyncRecord
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
	m.Incoming = append(m.Incoming, records...)
	return nil
}

func TestRAGSyncService_InterfaceMock(t *testing.T) {
	mockService := &MockRAGSyncService{
		PendingRecords: []RAGSyncRecord{
			{ID: "1", Context: "test 1", SyncStatus: SyncStatusPending, LastSyncAt: time.Now()},
		},
	}

	var service RAGSyncService = mockService

	ctx := context.Background()

	records, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(records) != 1 {
		t.Fatalf("Expected 1 record, got %d", len(records))
	}

	err = service.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	if len(mockService.MarkedIDs) != 1 || mockService.MarkedIDs[0] != "1" {
		t.Fatalf("MarkSynced did not update mock state correctly")
	}

	err = service.ProcessIncomingSync(ctx, []RAGSyncRecord{
		{ID: "2", Context: "test 2", SyncStatus: SyncStatusPending},
	})
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}
	if len(mockService.Incoming) != 1 || mockService.Incoming[0].ID != "2" {
		t.Fatalf("ProcessIncomingSync did not update mock state correctly")
	}
}
