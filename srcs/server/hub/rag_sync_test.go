package hub_test

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/hub"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"go.opentelemetry.io/otel/metric/noop"
)

// MockRAGSyncService is a mock implementation of hub.RAGSyncService for testing.
type MockRAGSyncService struct {
	PendingRecords []hub.RAGSyncRecord
	MarkedIDs      []string
	ProcessedData  []hub.RAGSyncRecord
	FetchErr       error
	MarkErr        error
	ProcessErr     error
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]hub.RAGSyncRecord, error) {
	if m.FetchErr != nil {
		if telemetry.RagSyncErrorsTotal != nil {
			telemetry.RagSyncErrorsTotal.Add(ctx, 1)
		}
		return nil, m.FetchErr
	}
	return m.PendingRecords, nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if m.MarkErr != nil {
		if telemetry.RagSyncErrorsTotal != nil {
			telemetry.RagSyncErrorsTotal.Add(ctx, 1)
		}
		return m.MarkErr
	}
	m.MarkedIDs = append(m.MarkedIDs, ids...)
	if telemetry.RagRecordsSyncedTotal != nil {
		telemetry.RagRecordsSyncedTotal.Add(ctx, int64(len(ids)))
	}
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []hub.RAGSyncRecord) error {
	if m.ProcessErr != nil {
		if telemetry.RagSyncErrorsTotal != nil {
			telemetry.RagSyncErrorsTotal.Add(ctx, 1)
		}
		return m.ProcessErr
	}
	m.ProcessedData = append(m.ProcessedData, records...)
	if telemetry.RagRecordsSyncedTotal != nil {
		telemetry.RagRecordsSyncedTotal.Add(ctx, int64(len(records)))
	}
	return nil
}

func TestRAGSyncServiceMock(t *testing.T) {
	// Initialize telemetry with noop meter so counters are not nil
	meter := noop.NewMeterProvider().Meter("test")
	telemetry.RagRecordsSyncedTotal, _ = meter.Int64Counter("rag_records_synced_total")
	telemetry.RagSyncErrorsTotal, _ = meter.Int64Counter("rag_sync_errors_total")

	ctx := context.Background()

	t.Run("FetchPendingSyncs", func(t *testing.T) {
		mockService := &MockRAGSyncService{
			PendingRecords: []hub.RAGSyncRecord{
				{
					ID:         "record-1",
					Context:    "test context",
					SyncStatus: hub.SyncStatusPending,
					LastSyncAt: time.Now(),
				},
			},
		}

		records, err := mockService.FetchPendingSyncs(ctx, 10)
		if err != nil {
			t.Fatalf("expected no error, got %v", err)
		}
		if len(records) != 1 {
			t.Fatalf("expected 1 record, got %d", len(records))
		}
		if records[0].ID != "record-1" {
			t.Fatalf("expected record-1, got %s", records[0].ID)
		}
	})

	t.Run("MarkSynced", func(t *testing.T) {
		mockService := &MockRAGSyncService{}
		ids := []string{"record-1", "record-2"}

		err := mockService.MarkSynced(ctx, ids)
		if err != nil {
			t.Fatalf("expected no error, got %v", err)
		}
		if len(mockService.MarkedIDs) != 2 {
			t.Fatalf("expected 2 marked IDs, got %d", len(mockService.MarkedIDs))
		}
	})

	t.Run("ProcessIncomingSync", func(t *testing.T) {
		mockService := &MockRAGSyncService{}
		records := []hub.RAGSyncRecord{
			{
				ID:         "record-3",
				Context:    "cloud context",
				SyncStatus: hub.SyncStatusPending,
				LastSyncAt: time.Now(),
			},
		}

		err := mockService.ProcessIncomingSync(ctx, records)
		if err != nil {
			t.Fatalf("expected no error, got %v", err)
		}
		if len(mockService.ProcessedData) != 1 {
			t.Fatalf("expected 1 processed record, got %d", len(mockService.ProcessedData))
		}
	})
}
