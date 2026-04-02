package orchestration

import (
	"context"
	"testing"
)

func TestTokenBurnRateCalculation(t *testing.T) {
	ResetForecastingEngineForTest()

	// Use an empty telemetry initialization or we just verify that it doesn't crash
	// and records the values properly internally.

	// Create context
	ctx := context.Background()

	// Simulate some usage
	RecordUsageForForecasting("tenant-1", 100)
	RecordUsageForForecasting("tenant-2", 500)
	RecordUsageForForecasting("tenant-1", 150)

	// Manually trigger the calculation that usually runs in a ticker
	calculateBurnRate(ctx)

	// tenant-1 should have 250, tenant-2 should have 500 diff
	burnRateMu.Lock()
	if tenantPreviousUsage["tenant-1"] != 250 {
		t.Errorf("Expected 250 for tenant-1, got %d", tenantPreviousUsage["tenant-1"])
	}
	if tenantPreviousUsage["tenant-2"] != 500 {
		t.Errorf("Expected 500 for tenant-2, got %d", tenantPreviousUsage["tenant-2"])
	}
	burnRateMu.Unlock()

	// Add more usage
	RecordUsageForForecasting("tenant-1", 50)
	RecordUsageForForecasting("tenant-2", 0)

	calculateBurnRate(ctx)

	// tenant-1 diff is 50, tenant-2 diff is 0
	burnRateMu.Lock()
	if tenantPreviousUsage["tenant-1"] != 300 {
		t.Errorf("Expected 300 for tenant-1, got %d", tenantPreviousUsage["tenant-1"])
	}
	if tenantPreviousUsage["tenant-2"] != 500 {
		t.Errorf("Expected 500 for tenant-2, got %d", tenantPreviousUsage["tenant-2"])
	}
	burnRateMu.Unlock()
}
