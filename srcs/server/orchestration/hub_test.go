package orchestration

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"go.opentelemetry.io/otel/metric"
)

type mockMeter struct{}

func (m mockMeter) Int64Counter(name string, options ...metric.Int64CounterOption) (metric.Int64Counter, error) {
	return nil, nil
}
func (m mockMeter) Float64Histogram(name string, options ...metric.Float64HistogramOption) (metric.Float64Histogram, error) {
	return nil, nil
}
func (m mockMeter) Float64Gauge(name string, options ...metric.Float64GaugeOption) (metric.Float64Gauge, error) {
	return mockGauge{}, nil
}

type mockGauge struct{}

func (g mockGauge) Record(ctx context.Context, value float64, options ...metric.RecordOption) {}

func TestHub_TokenBurnRate(t *testing.T) {
	telemetry.InitWithMeter(mockMeter{})

	hub := NewHub()

	hub.RecordTokenUsageToHub("org-1", 1000)
	hub.RecordTokenUsageToHub("org-1", 2000)
	hub.RecordTokenUsageToHub("org-2", 500)

	hub.calculateTokenBurnRates(context.Background())

	hub.tokenRateMu.Lock()
	defer hub.tokenRateMu.Unlock()

	if len(hub.tokenHistory["org-1"]) != 2 {
		t.Fatalf("Expected 2 records for org-1, got %d", len(hub.tokenHistory["org-1"]))
	}
	if len(hub.tokenHistory["org-2"]) != 1 {
		t.Fatalf("Expected 1 record for org-2, got %d", len(hub.tokenHistory["org-2"]))
	}
}
