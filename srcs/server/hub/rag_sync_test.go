package hub_test

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/hub"
)

type MockRAGSyncService struct {
	PendingRecords []hub.RAGSyncRecord
	MarkedIDs      []string
	ProcessedData  []hub.RAGSyncRecord
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]hub.RAGSyncRecord, error) {
	if len(m.PendingRecords) > limit {
		return m.PendingRecords[:limit], nil
	}
	return m.PendingRecords, nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	m.MarkedIDs = append(m.MarkedIDs, ids...)
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []hub.RAGSyncRecord) error {
	m.ProcessedData = append(m.ProcessedData, records...)
	return nil
}

func TestMockRAGSyncService(t *testing.T) {
	mockService := &MockRAGSyncService{
		PendingRecords: []hub.RAGSyncRecord{
			{
				ID:         "record-1",
				Context:    "test context",
				Vector:     []float32{1.0, 2.0},
				SyncStatus: hub.SyncStatusPending,
				LastSyncAt: time.Now(),
			},
		},
	}

	ctx := context.Background()
	records, err := mockService.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}

	if len(records) != 1 {
		t.Fatalf("Expected 1 record, got %d", len(records))
	}
	if records[0].SyncStatus != hub.SyncStatusPending {
		t.Fatalf("Expected SyncStatusPending, got %s", records[0].SyncStatus)
	}

	err = mockService.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}
	if len(mockService.ProcessedData) != 1 {
		t.Fatalf("Expected 1 processed record, got %d", len(mockService.ProcessedData))
	}

	err = mockService.MarkSynced(ctx, []string{records[0].ID})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}
	if len(mockService.MarkedIDs) != 1 {
		t.Fatalf("Expected 1 marked ID, got %d", len(mockService.MarkedIDs))
	}
}

func TestMetricsInitialized(t *testing.T) {
	if hub.RagRecordsSyncedTotal == nil {
		t.Fatal("Expected RagRecordsSyncedTotal metric to be initialized")
	}
	if hub.RagSyncErrorsTotal == nil {
		t.Fatal("Expected RagSyncErrorsTotal metric to be initialized")
	}
}
