package orchestration

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"go.opentelemetry.io/otel/metric"
)

type MockMeter struct{}
func (m *MockMeter) Int64Counter(name string, options ...metric.Int64CounterOption) (metric.Int64Counter, error) { return &mockCounter{}, nil }
func (m *MockMeter) Float64Histogram(name string, options ...metric.Float64HistogramOption) (metric.Float64Histogram, error) { return nil, nil }
func (m *MockMeter) Float64Gauge(name string, options ...metric.Float64GaugeOption) (metric.Float64Gauge, error) { return &mockGauge{}, nil }

type mockCounter struct{ metric.Int64Counter }
func (m *mockCounter) Add(ctx context.Context, incr int64, options ...metric.AddOption) {}

type mockGauge struct{ metric.Float64Gauge }
func (m *mockGauge) Record(ctx context.Context, value float64, options ...metric.RecordOption) {}

func TestTokenBurnRateForecasting(t *testing.T) {
	// Setup telemetry mock
	_ = telemetry.InitWithMeter(&MockMeter{})

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	getActiveOrgs := func(ctx context.Context) []string {
		return []string{"org1"}
	}

	getSummary := func(orgID string) int64 {
		return 100
	}

	// Start fast ticker
	go StartTokenBurnRateForecasting(ctx, getActiveOrgs, getSummary, 10*time.Millisecond)

	time.Sleep(35 * time.Millisecond) // Give it enough time to tick a few times

	// If it doesn't crash, the moving average logic runs without panic
	cancel()
}
