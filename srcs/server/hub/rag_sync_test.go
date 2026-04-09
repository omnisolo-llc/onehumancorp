package hub

import (
	"context"
	"testing"
	"time"

	"go.opentelemetry.io/otel/metric/noop"
)

type MockRAGSyncService struct {
	records []RAGSyncRecord
	synced  []string
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	var pending []RAGSyncRecord
	for _, r := range m.records {
		if r.SyncStatus == SyncStatusPending {
			pending = append(pending, r)
		}
	}
	if len(pending) > limit {
		return pending[:limit], nil
	}
	return pending, nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	m.synced = append(m.synced, ids...)
	for i, r := range m.records {
		for _, id := range ids {
			if r.ID == id {
				m.records[i].SyncStatus = SyncStatusSynced
			}
		}
	}
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	m.records = append(m.records, records...)
	return nil
}

func TestRAGSyncServiceMock(t *testing.T) {
	mockService := &MockRAGSyncService{
		records: []RAGSyncRecord{
			{ID: "1", Context: "test context 1", SyncStatus: SyncStatusPending, LastSyncAt: time.Now()},
			{ID: "2", Context: "test context 2", SyncStatus: SyncStatusPending, LastSyncAt: time.Now()},
		},
	}

	ctx := context.Background()

	// Test FetchPendingSyncs
	pending, err := mockService.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(pending) != 2 {
		t.Errorf("expected 2 pending records, got %d", len(pending))
	}

	// Test MarkSynced
	idsToSync := []string{"1", "2"}
	err = mockService.MarkSynced(ctx, idsToSync)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(mockService.synced) != 2 {
		t.Errorf("expected 2 synced records, got %d", len(mockService.synced))
	}

	// Test FetchPendingSyncs after sync
	pendingAfter, err := mockService.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(pendingAfter) != 0 {
		t.Errorf("expected 0 pending records, got %d", len(pendingAfter))
	}

	// Test ProcessIncomingSync
	newRecords := []RAGSyncRecord{
		{ID: "3", Context: "test context 3", SyncStatus: SyncStatusPending, LastSyncAt: time.Now()},
	}
	err = mockService.ProcessIncomingSync(ctx, newRecords)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(mockService.records) != 3 {
		t.Errorf("expected 3 total records, got %d", len(mockService.records))
	}
}

func TestInitRAGSyncMetrics(t *testing.T) {
	meter := noop.NewMeterProvider().Meter("test")
	err := InitRAGSyncMetrics(meter)
	if err != nil {
		t.Fatalf("unexpected error initializing metrics: %v", err)
	}
	if RAGRecordsSyncedTotal == nil {
		t.Error("expected RAGRecordsSyncedTotal to be initialized")
	}
	if RAGSyncErrorsTotal == nil {
		t.Error("expected RAGSyncErrorsTotal to be initialized")
	}
}
