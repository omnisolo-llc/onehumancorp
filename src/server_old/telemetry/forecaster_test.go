package telemetry

import (
	"context"
	"errors"
	"testing"
	"time"

	"go.opentelemetry.io/otel/metric"
	"go.opentelemetry.io/otel/metric/noop"
)

func TestForecaster(t *testing.T) {
	// Initialize metrics with a noop meter to prevent nil panic
	meter := noop.NewMeterProvider().Meter("test")
	var err error
	tokenBurnRateGauge, err = meter.Float64Gauge("ohc_token_burn_rate_forecast")
	if err != nil {
		t.Fatalf("Failed to create gauge: %v", err)
	}
	TokenBurnRatePredicted24h, err = meter.Float64Gauge("ohc_token_burn_rate_predicted_24h")
	if err != nil {
		t.Fatalf("Failed to create gauge: %v", err)
	}
	TokenBudgetAlertTotal, err = meter.Int64Counter("ohc_token_budget_alert_total")
	if err != nil {
		t.Fatalf("Failed to create counter: %v", err)
	}

	// Use a very short interval for testing
	f := NewForecaster(10*time.Millisecond, 1*time.Minute)

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	f.Start(ctx)

	// Set a budget that will be exceeded
	orgID := "tenant-1"
	f.SetBudget(orgID, 1000)

	// Add usage: 100 tokens.
	// In a 1 minute window, this is 100 tokens/min.
	// 24h prediction = 100 * 60 * 24 = 144,000 tokens.
	// This should exceed the budget of 1000.
	f.RecordUsage(orgID, 100)

	// Wait for the forecaster to process
	time.Sleep(50 * time.Millisecond)

	f.mu.Lock()
	records, ok := f.usageHistory[orgID]
	f.mu.Unlock()

	if !ok {
		t.Errorf("Expected records for %s, got none", orgID)
	}
	if len(records) != 1 {
		t.Errorf("Expected 1 record, got %d", len(records))
	}

	f.Stop()
}

func TestForecaster_NoBudget(t *testing.T) {
	meter := noop.NewMeterProvider().Meter("test")
	TokenBurnRatePredicted24h, _ = meter.Float64Gauge("ohc_token_burn_rate_predicted_24h")

	f := NewForecaster(10*time.Millisecond, 1*time.Minute)
	f.RecordUsage("no-budget-org", 100)
	f.calculateAndRecordRates(context.Background())
	// Should not panic or fail
}

func TestForecaster_ContextCancel(t *testing.T) {
	f := NewForecaster(10*time.Millisecond, 1*time.Minute)
	ctx, cancel := context.WithCancel(context.Background())
	f.Start(ctx)
	cancel()
	time.Sleep(20 * time.Millisecond)
	// Should terminate goroutine
}

func TestForecaster_DoubleStop(t *testing.T) {
	f := NewForecaster(10*time.Millisecond, 1*time.Minute)
	f.Stop()
	f.Stop() // Should not panic
}

type errorMeter struct {
	mockableMeter
	failGauge   bool
	failCounter bool
}

func (e *errorMeter) Float64Gauge(name string, options ...metric.Float64GaugeOption) (metric.Float64Gauge, error) {
	if e.failGauge {
		return nil, errors.New("gauge error")
	}
	return noop.NewMeterProvider().Meter("test").Float64Gauge(name)
}

func (e *errorMeter) Int64Counter(name string, options ...metric.Int64CounterOption) (metric.Int64Counter, error) {
	if e.failCounter {
		return nil, errors.New("counter error")
	}
	return noop.NewMeterProvider().Meter("test").Int64Counter(name)
}

func TestInitForecastingMetrics_Error(t *testing.T) {
	err := initForecastingMetrics(&errorMeter{failGauge: true})
	if err == nil {
		t.Error("expected error from Float64Gauge")
	}

	err = initForecastingMetrics(&errorMeter{failCounter: true})
	if err == nil {
		t.Error("expected error from Int64Counter")
	}
}
