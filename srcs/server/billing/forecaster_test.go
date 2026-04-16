package billing

import (
	"context"
	"testing"
	"time"
)

func TestForecaster_BurnRates(t *testing.T) {
	forecaster := NewForecaster(10*time.Millisecond, 1*time.Hour, 1000.0)
	orgID := "test-org"

	// 1. Initial rates should be 0
	tRate, uRate := forecaster.GetBurnRates(orgID)
	if tRate != 0 || uRate != 0 {
		t.Errorf("Initial rates should be 0, got tokens=%v, usd=%v", tRate, uRate)
	}

	// 2. Record first usage
	now := time.Now()
	forecaster.mu.Lock()
	forecaster.usageHistory[orgID] = append(forecaster.usageHistory[orgID], usageRecord{
		timestamp: now.Add(-2 * time.Minute),
		tokens:    1000,
		costUSD:   1.0,
	})
	forecaster.usageHistory[orgID] = append(forecaster.usageHistory[orgID], usageRecord{
		timestamp: now,
		tokens:    2000,
		costUSD:   2.0,
	})
	forecaster.mu.Unlock()

	// 4. Verify rates (1000 tokens / 2 min = 500 tokens/min, 1.0 USD / 2 min = 0.5 USD/min)
	tRate, uRate = forecaster.GetBurnRates(orgID)
	if tRate < 499 || tRate > 501 {
		t.Errorf("Expected token rate ~500, got %v", tRate)
	}
	if uRate < 0.49 || uRate > 0.51 {
		t.Errorf("Expected USD rate ~0.5, got %v", uRate)
	}
}

func TestForecaster_CollectAndCalculate(t *testing.T) {
	forecaster := NewForecaster(1*time.Second, 1*time.Hour, 0.1)
	orgID := "test-org"

	tokens := int64(0)
	cost := 0.0

	forecaster.SetDataProviders(
		func(ctx context.Context) []string { return []string{orgID} },
		func(ctx context.Context, id string) (int64, float64) {
			return tokens, cost
		},
	)

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	// 1. First collection
	tokens = 1000
	cost = 1.0
	forecaster.collectAndCalculate(ctx)

	// Rates should still be 0 because only one data point
	tr, ur := forecaster.GetBurnRates(orgID)
	if tr != 0 || ur != 0 {
		t.Errorf("Expected 0 rates with one data point, got %v, %v", tr, ur)
	}

	// Manually backdate the first record to simulate time passing
	forecaster.mu.Lock()
	forecaster.usageHistory[orgID][0].timestamp = time.Now().Add(-1 * time.Minute)
	forecaster.mu.Unlock()

	// 2. Second collection
	tokens = 2000
	cost = 2.0
	forecaster.collectAndCalculate(ctx)

	tr, ur = forecaster.GetBurnRates(orgID)
	if tr < 999 || tr > 1001 {
		t.Errorf("Expected token rate ~1000, got %v", tr)
	}
	if ur < 0.99 || ur > 1.01 {
		t.Errorf("Expected USD rate ~1.0, got %v", ur)
	}
}

func TestForecaster_StopIdempotency(t *testing.T) {
	f := NewForecaster(time.Second, time.Minute, 100)
	f.Stop()
	f.Stop() // Should not panic
}
