package billing

import (
	"context"
	"sync"
	"time"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

type Event struct {
	Timestamp time.Time
	Cost      float64
}

type Forecaster struct {
	mu           sync.Mutex
	events       map[string][]Event
	window       time.Duration
	monthlyHours float64

	// OpenTelemetry gauges
	meter metric.Meter
	gauge metric.Float64Gauge
}

func NewForecaster(window time.Duration) *Forecaster {
	meter := otel.Meter("github.com/onehumancorp/mono/ohc/billing")
	gauge, _ := meter.Float64Gauge("ohc_usd_burn_rate_forecast", metric.WithDescription("Predicted moving average of USD burn rate per month per tenant"))

	return &Forecaster{
		events:       make(map[string][]Event),
		window:       window,
		monthlyHours: 24 * 30,
		meter:        meter,
		gauge:        gauge,
	}
}

func (f *Forecaster) TrackEvent(tenantID string, cost float64) {
	f.mu.Lock()
	defer f.mu.Unlock()

	now := time.Now()

	f.events[tenantID] = append(f.events[tenantID], Event{
		Timestamp: now,
		Cost:      cost,
	})

	// Prune old events
	cutoff := now.Add(-f.window)
	var filtered []Event
	for _, e := range f.events[tenantID] {
		if e.Timestamp.After(cutoff) {
			filtered = append(filtered, e)
		}
	}
	f.events[tenantID] = filtered
}

// ProjectMonthlyCost extrapolates the cost for the next 30 days based on the current window.
func (f *Forecaster) ProjectMonthlyCost(ctx context.Context, tenantID string) float64 {
	f.mu.Lock()
	defer f.mu.Unlock()

	events, ok := f.events[tenantID]
	if !ok || len(events) == 0 {
		return 0.0
	}

	now := time.Now()
	cutoff := now.Add(-f.window)

	var totalCost float64
	var filtered []Event
	for _, e := range events {
		if e.Timestamp.After(cutoff) {
			totalCost += e.Cost
			filtered = append(filtered, e)
		}
	}

	f.events[tenantID] = filtered

	windowHours := f.window.Hours()
	if windowHours <= 0 {
		return 0.0
	}

	hourlyRate := totalCost / windowHours
	monthlyProjection := hourlyRate * f.monthlyHours

	// If we wanted to record it we would do it here (using Record() from float64gauge)
	// but currently the telemetry is globally recorded, so we just return the value for now.

	return monthlyProjection
}
