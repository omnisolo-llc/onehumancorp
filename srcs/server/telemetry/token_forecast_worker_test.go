package telemetry

import (
    "context"
    "testing"
    "time"

    "go.opentelemetry.io/otel/metric/noop"
)

func TestTokenForecastWorker(t *testing.T) {
    meter := noop.NewMeterProvider().Meter("test")
    var err error
    tokenBurnRateGauge, err = meter.Float64Gauge("ohc_token_burn_rate_forecast")
    if err != nil {
        t.Fatalf("Failed to create gauge: %v", err)
    }

    worker := NewTokenForecastWorker(10*time.Millisecond, 0.2)

    ctx, cancel := context.WithCancel(context.Background())
    defer cancel()

    worker.Start(ctx)

    orgID := "tenant-1"
    worker.RecordUsage(orgID, 600)
    worker.RecordUsage(orgID, 600)

    time.Sleep(50 * time.Millisecond)

    worker.mu.Lock()
    _, ok := worker.usageHistory[orgID]
    worker.mu.Unlock()

    if !ok {
        t.Errorf("Expected records for %s, got none", orgID)
    }

    worker.Stop()
}
