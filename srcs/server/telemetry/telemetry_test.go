package telemetry

import (
	"context"
	"math"
	"testing"
	"time"

	_ "github.com/mattn/go-sqlite3"
)

func TestTokenForecaster_EWMA(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()

	syncEngine := NewTelemetrySyncEngine(db, "http://example.com/sync")
	forecaster := NewTokenForecaster(syncEngine)

	tenantID := "test_tenant"

	// Initial usage
	forecaster.RecordUsage(tenantID, 100.0)

	ewma := forecaster.CalculateEWMA(tenantID)
	if ewma != 0.0 {
		t.Errorf("Expected EWMA 0.0, got %v", ewma) // Needs two points to calculate rate
	}

	// Wait 1 second and add more usage to calculate tokens/second
	// Let's modify the timestamp directly for test
	forecaster.mu.Lock()
	forecaster.lastTimestamp[tenantID] = forecaster.lastTimestamp[tenantID].Add(-1 * time.Second)
	forecaster.mu.Unlock()

	// 200 tokens over 1 second = 200 tokens/sec
	forecaster.RecordUsage(tenantID, 200.0)

	// EWMA should be initialized to 200.0 directly on first rate calculation
	ewma = forecaster.CalculateEWMA(tenantID)
	if math.Abs(ewma - 200.0) > 1.0 {
		t.Errorf("Expected EWMA ~200.0, got %v", ewma)
	}

	// Let's add a third point, wait 1 sec
	forecaster.mu.Lock()
	forecaster.lastTimestamp[tenantID] = forecaster.lastTimestamp[tenantID].Add(-1 * time.Second)
	forecaster.mu.Unlock()

	// 100 tokens over 1 second = 100 tokens/sec
	// EWMA = (100.0 * 0.2) + (200.0 * 0.8) = 20.0 + 160.0 = 180.0
	forecaster.RecordUsage(tenantID, 100.0)

	ewma = forecaster.CalculateEWMA(tenantID)
	if math.Abs(ewma - 180.0) > 1.0 {
		t.Errorf("Expected EWMA ~180.0, got %v", ewma)
	}
}

func TestTokenForecaster_Daemon(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()

	syncEngine := NewTelemetrySyncEngine(db, "http://example.com/sync")
	forecaster := NewTokenForecaster(syncEngine)

	tenantID := "test_tenant_daemon"
	forecaster.RecordUsage(tenantID, 150.0)
	forecaster.mu.Lock()
	forecaster.lastTimestamp[tenantID] = forecaster.lastTimestamp[tenantID].Add(-1 * time.Second)
	forecaster.mu.Unlock()
	forecaster.RecordUsage(tenantID, 100.0) // generates a burn rate

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	// Run daemon with very short interval
	go forecaster.StartForecastingDaemon(ctx, 10*time.Millisecond)

	time.Sleep(50 * time.Millisecond)

	var count int
	err := db.QueryRow("SELECT COUNT(*) FROM local_telemetry_metrics WHERE metric_name = 'ohc_token_burn_rate_forecast'").Scan(&count)
	if err != nil {
		t.Fatalf("Failed to query DB: %v", err)
	}

	if count == 0 {
		t.Errorf("Expected token burn rate forecasts to be buffered, but found 0")
	}
}
