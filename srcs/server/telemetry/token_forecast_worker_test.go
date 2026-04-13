package telemetry

import (
	"context"
	"testing"
	"time"

	"go.opentelemetry.io/otel/metric/noop"
)

func TestTokenForecastWorker(t *testing.T) {
	// Initialize the global gauge with a noop meter to prevent nil panic
	meter := noop.NewMeterProvider().Meter("test")
	var err error
	tokenBurnRateGauge, err = meter.Float64Gauge("ohc_token_burn_rate_forecast")
	if err != nil {
		t.Fatalf("Failed to create gauge: %v", err)
	}

	worker := NewTokenForecastWorker(10*time.Millisecond, 1*time.Minute)

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	worker.Start(ctx)

	// Add some usage
	orgID := "tenant-1"
	worker.RecordUsage(orgID, 600)
	worker.RecordUsage(orgID, 600)

	// Wait a bit for the worker to process
	time.Sleep(50 * time.Millisecond)

	// Calculate expected rate: 1200 tokens over 1 minute window = 1200 tokens/min
	worker.mu.Lock()
	records, ok := worker.usageHistory[orgID]
	worker.mu.Unlock()

	if !ok {
		t.Errorf("Expected records for %s, got none", orgID)
	}
	if len(records) != 2 {
		t.Errorf("Expected 2 records, got %d", len(records))
	}

	worker.Stop()
}
