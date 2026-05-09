package telemetry

import (
	"context"
	"testing"
)

func TestLocalSovereignty(t *testing.T) {
	// Initialize a global engine (mocking app startup)
	db := setupTestDB(t)
	defer db.Close()
	engine := NewTelemetrySyncEngine(db, "http://localhost:8080/metrics")
	InitGlobalSyncEngine(engine)

	t.Setenv("OHC_STANDALONE", "true")
	// Implicitly OHC_TELEMETRY_ENABLED is not set or false

	ctx := context.Background()

	attrs := map[string]interface{}{
		"service": "agent",
		"mode":    "standalone_no_telemetry",
	}

	// This should NO-OP because OHC_TELEMETRY_ENABLED is not true
	bufferMetricHelper(ctx, "test_sovereignty_metric", 1.0, attrs)

	var count int
	err := db.QueryRow("SELECT count(*) FROM local_telemetry_metrics WHERE metric_name = 'test_sovereignty_metric'").Scan(&count)
	if err != nil {
		t.Fatalf("Failed to query db: %v", err)
	}

	if count != 0 {
		t.Errorf("Expected 0 buffered metrics due to local sovereignty protection, but found %d", count)
	}

	// Now try explicitly disabling it
	t.Setenv("OHC_TELEMETRY_ENABLED", "false")
	bufferMetricHelper(ctx, "test_sovereignty_metric", 1.0, attrs)
	err = db.QueryRow("SELECT count(*) FROM local_telemetry_metrics WHERE metric_name = 'test_sovereignty_metric'").Scan(&count)
	if err != nil {
		t.Fatalf("Failed to query db: %v", err)
	}
	if count != 0 {
		t.Errorf("Expected 0 buffered metrics when explicitly disabled, but found %d", count)
	}

	// Now try enabling it to ensure it buffers
	t.Setenv("OHC_TELEMETRY_ENABLED", "true")
	bufferMetricHelper(ctx, "test_sovereignty_metric_enabled", 1.0, attrs)
	err = db.QueryRow("SELECT count(*) FROM local_telemetry_metrics WHERE metric_name = 'test_sovereignty_metric_enabled'").Scan(&count)
	if err != nil {
		t.Fatalf("Failed to query db: %v", err)
	}
	if count != 1 {
		t.Errorf("Expected 1 buffered metric when explicitly enabled, but found %d", count)
	}
}
