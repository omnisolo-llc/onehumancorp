package orchestration

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"go.opentelemetry.io/otel/metric"
)

// Mock meter for testing telemetry updates.
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

func TestTokenBurnRateEngine(t *testing.T) {
	// Initialize telemetry with mock meter to avoid panics.
	_ = telemetry.InitWithMeter(mockMeter{})

	// Initialize the hub and trigger the token usage callback directly.
	hub := NewHub()

	// Mock token usage records via the exposed hub method
	hub.RecordTokenUsage("agent1", 100)
	hub.RecordTokenUsage("agent1", 50)

	hub.tokenUsageMu.Lock()
	if len(hub.tokenUsage["default"]) != 2 {
		t.Errorf("Expected 2 token usage samples, got %d", len(hub.tokenUsage["default"]))
	}
	hub.tokenUsageMu.Unlock()

	// Advance time for some samples to simulate older usage outside the 5-minute window
	hub.tokenUsageMu.Lock()
	hub.tokenUsage["default"][0].RecordedAt = time.Now().Add(-10 * time.Minute)
	hub.tokenUsageMu.Unlock()

	// Trigger calculation
	hub.calculateAndEmitTokenBurnRate(context.Background())

	// Verify older samples were pruned
	hub.tokenUsageMu.Lock()
	if len(hub.tokenUsage["default"]) != 1 {
		t.Errorf("Expected 1 valid token usage sample after pruning, got %d", len(hub.tokenUsage["default"]))
	}
	if hub.tokenUsage["default"][0].Count != 50 {
		t.Errorf("Expected remaining sample count to be 50, got %d", hub.tokenUsage["default"][0].Count)
	}
	hub.tokenUsageMu.Unlock()
}
