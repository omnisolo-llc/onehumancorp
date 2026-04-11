package hub

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"go.opentelemetry.io/otel/metric"
)

// MockRAGSyncService is a mock implementation of RAGSyncService for testing
type MockRAGSyncService struct {
	records []RAGSyncRecord
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if limit > len(m.records) {
		limit = len(m.records)
	}
	return m.records[:limit], nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	for _, id := range ids {
		for i, rec := range m.records {
			if rec.ID == id {
				m.records[i].SyncStatus = SyncStatusSynced
				m.records[i].LastSyncAt = time.Now()
				telemetry.RagRecordsSyncedTotal.Add(ctx, 1)
			}
		}
	}
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	m.records = append(m.records, records...)
	return nil
}

func TestRAGSyncService(t *testing.T) {
	// Initialize telemetry with a mock meter to avoid panics
	mockMeter := &mockMeter{}
	telemetry.InitWithMeter(mockMeter)

	ctx := context.Background()
	service := &MockRAGSyncService{
		records: []RAGSyncRecord{
			{ID: "1", Context: "test 1", SyncStatus: SyncStatusPending},
			{ID: "2", Context: "test 2", SyncStatus: SyncStatusPending},
		},
	}

	// Test FetchPendingSyncs
	pending, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 2 {
		t.Errorf("expected 2 pending records, got %d", len(pending))
	}

	// Test MarkSynced
	err = service.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	if service.records[0].SyncStatus != SyncStatusSynced {
		t.Errorf("expected record 1 to be synced")
	}

	// Test ProcessIncomingSync
	newRecs := []RAGSyncRecord{
		{ID: "3", Context: "test 3", SyncStatus: SyncStatusSynced},
	}
	err = service.ProcessIncomingSync(ctx, newRecs)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	if len(service.records) != 3 {
		t.Errorf("expected 3 records after process incoming, got %d", len(service.records))
	}
}

// Minimal mock meter for testing
type mockMeter struct{}

func (m *mockMeter) Int64Counter(name string, options ...metric.Int64CounterOption) (metric.Int64Counter, error) {
	return &mockCounter{}, nil
}
func (m *mockMeter) Int64UpDownCounter(name string, options ...metric.Int64UpDownCounterOption) (metric.Int64UpDownCounter, error) {
	return &mockUpDownCounter{}, nil
}
func (m *mockMeter) Float64Histogram(name string, options ...metric.Float64HistogramOption) (metric.Float64Histogram, error) {
	return &mockHistogram{}, nil
}
func (m *mockMeter) Float64Gauge(name string, options ...metric.Float64GaugeOption) (metric.Float64Gauge, error) {
	return &mockGauge{}, nil
}
func (m *mockMeter) Int64Histogram(name string, options ...metric.Int64HistogramOption) (metric.Int64Histogram, error) {
	return &mockInt64Histogram{}, nil
}

type mockCounter struct{ metric.Int64Counter }
func (c *mockCounter) Add(ctx context.Context, incr int64, options ...metric.AddOption) {}

type mockUpDownCounter struct{ metric.Int64UpDownCounter }
func (c *mockUpDownCounter) Add(ctx context.Context, incr int64, options ...metric.AddOption) {}

type mockHistogram struct{ metric.Float64Histogram }
func (h *mockHistogram) Record(ctx context.Context, val float64, options ...metric.RecordOption) {}

type mockGauge struct{ metric.Float64Gauge }
func (g *mockGauge) Record(ctx context.Context, val float64, options ...metric.RecordOption) {}

type mockInt64Histogram struct{ metric.Int64Histogram }
func (h *mockInt64Histogram) Record(ctx context.Context, val int64, options ...metric.RecordOption) {}
