package orchestration

import (
	"context"
	"testing"
	"time"

	"go.opentelemetry.io/otel/metric"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

func TestTokenForecaster(t *testing.T) {
	hub := NewHub()

	// Create forecaster but manually override its parameters for fast testing
	forecaster := NewTokenForecaster(hub)
	forecaster.window = 5 * time.Millisecond // very short window
	forecaster.interval = 1 * time.Millisecond // fast ticks

	// Track calls to RecordTokenBurnRate
	var lastRate float64
	var burnRateCalled bool

	// Mock telemetry.RecordTokenBurnRate
	telemetry.InitWithMeter(&mockMeter{}) // Setup dummy telemetry if needed
	telemetry.BufferMetricFunc = func(ctx context.Context, metricType string, payload string) error { return nil }

	forecaster.Start()

	// Send some token usage
	telemetry.RecordTokenUsage(context.Background(), "agent-1", "SOFTWARE_ENGINEER", "gpt-4o", "prompt", 100)
	telemetry.RecordTokenUsage(context.Background(), "agent-1", "SOFTWARE_ENGINEER", "gpt-4o", "completion", 50)

	// Wait a bit to let the moving average calculate
	time.Sleep(10 * time.Millisecond)

	forecaster.Stop()

	// Check internal state
	forecaster.mu.Lock()
	defer forecaster.mu.Unlock()

	// We expect the history for the 'default' organization to have been cleared out
	// because our window is 5ms and we waited 10ms
	if len(forecaster.history["default"]) > 0 {
		t.Errorf("Expected history to be cleared out, but got %d events", len(forecaster.history["default"]))
	}

	_ = lastRate
	_ = burnRateCalled
}

// Minimal mock to satisfy InitWithMeter for tests
type mockMeter struct{}
func (m *mockMeter) Int64Counter(name string, options ...metric.Int64CounterOption) (metric.Int64Counter, error) { return &mockCounter{}, nil }
func (m *mockMeter) Float64Histogram(name string, options ...metric.Float64HistogramOption) (metric.Float64Histogram, error) { return nil, nil }
func (m *mockMeter) Float64Gauge(name string, options ...metric.Float64GaugeOption) (metric.Float64Gauge, error) { return &mockGauge{}, nil }

type mockGauge struct{ metric.Float64Gauge }
func (m *mockGauge) Record(ctx context.Context, value float64, options ...metric.RecordOption) {}
func (m *mockGauge) Enabled(ctx context.Context) bool { return true }

type mockCounter struct{ metric.Int64Counter }
func (m *mockCounter) Add(ctx context.Context, incr int64, options ...metric.AddOption) {}
func (m *mockCounter) Enabled(ctx context.Context) bool { return true }
