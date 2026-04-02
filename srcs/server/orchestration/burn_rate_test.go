package orchestration

import (
	"context"
	"os"
	"path/filepath"
	"sync"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

func TestBurnRateForecaster(t *testing.T) {
	// Reset forecaster for test
	globalForecaster = &BurnRateForecaster{
		usageHistory: make(map[string][]usagePoint),
		window:       5 * time.Minute,
		interval:     1 * time.Minute,
	}

	// Fake telemetry call check
	var capturedRate float64
	var capturedTenant string
	var mu sync.Mutex

	// Create a wrapper context or override telemetry gauge?
	// Telemetry gauge is not easily mocked if we use global, but we can verify our own internal calculation function directly without calling telemetry directly, or we can mock RecordTokenBurnRate by overriding tokenBurnRateGauge in telemetry if it was exposed, which it isn't.
	// But `telemetry.RecordTokenBurnRate` doesn't crash if the gauge isn't initialized, so we can just verify the internal logic.

	RecordUsage("tenant-1", 100)
	RecordUsage("tenant-1", 200)

	globalForecaster.mu.Lock()
	if len(globalForecaster.usageHistory["tenant-1"]) != 2 {
		t.Fatalf("expected 2 usage points, got %d", len(globalForecaster.usageHistory["tenant-1"]))
	}
	globalForecaster.mu.Unlock()

	// Adjust timestamp of first to be outside the window to test cutoff
	globalForecaster.mu.Lock()
	globalForecaster.usageHistory["tenant-1"][0].timestamp = time.Now().Add(-10 * time.Minute)
	globalForecaster.mu.Unlock()

	// Should drop the first point on next RecordUsage
	RecordUsage("tenant-1", 50)
	globalForecaster.mu.Lock()
	if len(globalForecaster.usageHistory["tenant-1"]) != 2 { // the one outside window is filtered out
		t.Fatalf("expected 2 usage points after cleanup, got %d", len(globalForecaster.usageHistory["tenant-1"]))
	}
	globalForecaster.mu.Unlock()

	// Test compute logic by calling computeAndExportBurnRate manually
	// Total valid tokens = 200 + 50 = 250.
	// Minutes = 5
	// Rate = 50 per min.
	// We can't easily intercept `telemetry.RecordTokenBurnRate` without modifying telemetry pkg, but we can ensure it doesn't crash.
	globalForecaster.computeAndExportBurnRate(context.Background())

	StartForecaster(context.Background())
	time.Sleep(10 * time.Millisecond) // Let it run
	StopForecaster()
}

func TestSQLiteConcurrencyThrottlingCheck(t *testing.T) {
	// Ensure that the condition inside UpsertMission correctly identifies standalone sqlite.
	dbPath := filepath.Join(t.TempDir(), "throttle.db")
	db, err := NewSIPDB(dbPath)
	if err != nil {
		t.Fatalf("Failed to create db: %v", err)
	}
	defer db.Close()

	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()

	// Fire multiple UpsertMission calls. The throttle semaphore size is 2.
	// If it doesn't deadlock, and processes all, it's working.
	var wg sync.WaitGroup
	for i := 0; i < 5; i++ {
		wg.Add(1)
		go func(id int) {
			defer wg.Done()
			_ = db.UpsertMission(ctx, "mission-1", "PENDING", "{}", false)
		}(i)
	}
	wg.Wait()
}
