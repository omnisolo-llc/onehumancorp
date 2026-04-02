package orchestration

import (
	"context"
	"testing"
)

func TestBurnRateEngine(t *testing.T) {
	InitBurnRateEngine()
	defer GlobalBurnRateEngine.Stop()

	// Simulate some usages
	GlobalBurnRateEngine.TrackUsage(context.Background(), "org1", 100)
	GlobalBurnRateEngine.TrackUsage(context.Background(), "org1", 200)

	// Since calculateForecast runs on ticker, let's manually trigger it to test logic
	GlobalBurnRateEngine.calculateForecast()

	// The average should be computed properly
	// e.usageHistory should now have an extra bucket
	GlobalBurnRateEngine.mu.Lock()
	if len(GlobalBurnRateEngine.usageHistory) != 2 {
		t.Errorf("expected usageHistory length 2, got %d", len(GlobalBurnRateEngine.usageHistory))
	}
	// The first bucket should contain 300
	if GlobalBurnRateEngine.usageHistory[0] != 300 {
		t.Errorf("expected bucket 0 to have 300, got %d", GlobalBurnRateEngine.usageHistory[0])
	}
	GlobalBurnRateEngine.mu.Unlock()
}
