package hub

import (
    "context"
    "testing"
    "time"

    "go.opentelemetry.io/otel/metric/noop"
)

type mockRAGSyncService struct{}

func (m *mockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    return []RAGSyncRecord{
        {
            ID:         "test-1",
            Context:    "test context",
            Vector:     []byte{1, 2, 3},
            SyncStatus: SyncStatusPending,
            LastSyncAt: time.Now(),
        },
    }, nil
}

func (m *mockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
    return nil
}

func (m *mockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    return nil
}

func TestRAGSyncService_Interface(t *testing.T) {
    var service RAGSyncService = &mockRAGSyncService{}
    records, err := service.FetchPendingSyncs(context.Background(), 10)
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if len(records) != 1 {
        t.Fatalf("expected 1 record, got %d", len(records))
    }
    if records[0].SyncStatus != SyncStatusPending {
        t.Fatalf("expected pending status, got %v", records[0].SyncStatus)
    }
}

func TestNewRAGSyncMetrics(t *testing.T) {
    meter := noop.NewMeterProvider().Meter("test")
    metrics, err := NewRAGSyncMetrics(meter)
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if metrics == nil {
        t.Fatal("expected metrics to be initialized")
    }
}
