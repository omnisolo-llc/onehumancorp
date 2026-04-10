package hybrid_sync

import (
    "context"
    "testing"
    "time"

    "go.opentelemetry.io/otel"
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

func TestRAGSyncService_Mock(t *testing.T) {
    mockService := &MockRAGSyncService{
        pendingRecords: []RAGSyncRecord{
            {ID: "1", Context: "test 1", SyncStatus: SyncStatusPending},
            {ID: "2", Context: "test 2", SyncStatus: SyncStatusPending},
        },
    }

    ctx := context.Background()

    // Test FetchPendingSyncs
    records, err := mockService.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("FetchPendingSyncs failed: %v", err)
    }
    if len(records) != 2 {
        t.Errorf("Expected 2 records, got %d", len(records))
    }

    // Test MarkSynced
    err = mockService.MarkSynced(ctx, []string{"1", "2"})
    if err != nil {
        t.Fatalf("MarkSynced failed: %v", err)
    }
    if len(mockService.syncedIDs) != 2 {
        t.Errorf("Expected 2 synced IDs, got %d", len(mockService.syncedIDs))
    }

    // Test ProcessIncomingSync
    now := time.Now()
    incoming := []RAGSyncRecord{
        {ID: "3", Context: "test 3", SyncStatus: SyncStatusSynced, LastSyncAt: &now},
    }
    err = mockService.ProcessIncomingSync(ctx, incoming)
    if err != nil {
        t.Fatalf("ProcessIncomingSync failed: %v", err)
    }
    if len(mockService.incomingSyncs) != 1 {
        t.Errorf("Expected 1 incoming sync record, got %d", len(mockService.incomingSyncs))
    }
}

func TestInitRAGSyncMetrics(t *testing.T) {
    meter := otel.GetMeterProvider().Meter("test-meter")
    err := InitRAGSyncMetrics(meter)
    if err != nil {
        t.Fatalf("InitRAGSyncMetrics failed: %v", err)
    }
    if RAGRecordsSyncedTotal == nil {
        t.Error("RAGRecordsSyncedTotal was not initialized")
    }
    if RAGSyncErrorsTotal == nil {
        t.Error("RAGSyncErrorsTotal was not initialized")
    }
}
