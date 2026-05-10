package telemetry

import (
	"context"
	"database/sql"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	_ "github.com/mattn/go-sqlite3"
)

func setupTestDB(t *testing.T) *sql.DB {
	db, err := sql.Open("sqlite3", ":memory:")
	if err != nil {
		t.Fatalf("Failed to open test db: %v", err)
	}

	_, err = db.Exec(`
		CREATE TABLE IF NOT EXISTS local_telemetry_metrics (
			id TEXT PRIMARY KEY,
			metric_name TEXT NOT NULL,
			value REAL NOT NULL,
			attributes TEXT,
			timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
			synced_to_cloud BOOLEAN NOT NULL DEFAULT FALSE
		);
	`)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}
	return db
}

func TestTelemetrySyncEngine_BufferMetric(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()

	engine := NewTelemetrySyncEngine(db, "http://localhost:8080/metrics")
	ctx := context.Background()

	attrs := map[string]interface{}{"service": "agent", "mode": "standalone"}
	err := engine.BufferMetric(ctx, "agent_execution_time", 1.5, attrs)
	if err != nil {
		t.Fatalf("Failed to buffer metric: %v", err)
	}

	var count int
	err = db.QueryRow("SELECT count(*) FROM local_telemetry_metrics").Scan(&count)
	if err != nil {
		t.Fatalf("Failed to query db: %v", err)
	}
	if count != 1 {
		t.Fatalf("Expected 1 row, got %d", count)
	}
}

func TestTelemetrySyncEngine_SyncPendingMetrics(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()

	// Mock the cloud endpoint
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		var pt MetricPoint
		if err := json.NewDecoder(r.Body).Decode(&pt); err != nil {
			t.Errorf("Failed to decode request body: %v", err)
			w.WriteHeader(http.StatusBadRequest)
			return
		}
		if pt.MetricName != "test_metric" {
			t.Errorf("Expected metric name 'test_metric', got '%s'", pt.MetricName)
		}
		w.WriteHeader(http.StatusOK)
	}))
	defer server.Close()

	engine := NewTelemetrySyncEngine(db, server.URL)
	ctx := context.Background()

	attrs := map[string]interface{}{"service": "agent"}
	err := engine.BufferMetric(ctx, "test_metric", 42.0, attrs)
	if err != nil {
		t.Fatalf("Failed to buffer metric: %v", err)
	}

	// First verify it's unsynced
	var countUnsynced int
	err = db.QueryRow("SELECT count(*) FROM local_telemetry_metrics WHERE synced_to_cloud = FALSE").Scan(&countUnsynced)
	if err != nil || countUnsynced != 1 {
		t.Fatalf("Expected 1 unsynced row")
	}

	err = engine.SyncPendingMetrics(ctx)
	if err != nil {
		t.Fatalf("Failed to sync metrics: %v", err)
	}

	// Now verify it's synced
	var countSynced int
	err = db.QueryRow("SELECT count(*) FROM local_telemetry_metrics WHERE synced_to_cloud = TRUE").Scan(&countSynced)
	if err != nil || countSynced != 1 {
		t.Fatalf("Expected 1 synced row")
	}

	var countUnsyncedAfter int
	err = db.QueryRow("SELECT count(*) FROM local_telemetry_metrics WHERE synced_to_cloud = FALSE").Scan(&countUnsyncedAfter)
	if err != nil || countUnsyncedAfter != 0 {
		t.Fatalf("Expected 0 unsynced rows")
	}
}

func TestTelemetrySyncEngine_SyncPendingMetrics_Failure(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()

	// Mock the cloud endpoint returning an error
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusInternalServerError)
	}))
	defer server.Close()

	engine := NewTelemetrySyncEngine(db, server.URL)
	ctx := context.Background()

	attrs := map[string]interface{}{"service": "agent"}
	err := engine.BufferMetric(ctx, "fail_metric", 42.0, attrs)
	if err != nil {
		t.Fatalf("Failed to buffer metric: %v", err)
	}

	err = engine.SyncPendingMetrics(ctx)
	if err != nil {
		t.Fatalf("Failed to execute sync method: %v", err)
	}

	// Verify it remains unsynced
	var countUnsynced int
	err = db.QueryRow("SELECT count(*) FROM local_telemetry_metrics WHERE synced_to_cloud = FALSE").Scan(&countUnsynced)
	if err != nil || countUnsynced != 1 {
		t.Fatalf("Expected 1 unsynced row")
	}
}

func TestTelemetrySyncEngine_StartSyncDaemon(t *testing.T) {
	t.Parallel()
	db := setupTestDB(t)
	defer db.Close()

	// Mock the cloud endpoint
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
	}))
	defer server.Close()

	engine := NewTelemetrySyncEngine(db, server.URL)
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	attrs := map[string]interface{}{"service": "agent"}
	err := engine.BufferMetric(ctx, "test_metric_daemon", 12.0, attrs)
	if err != nil {
		t.Fatalf("Failed to buffer metric: %v", err)
	}

	go engine.StartSyncDaemon(ctx, 50*time.Millisecond)

	// Wait for daemon to run and wait up to a few cycles for database
	for i := 0; i < 10; i++ {
		time.Sleep(50 * time.Millisecond)
	}

	var countSynced int
	err = db.QueryRow("SELECT count(*) FROM local_telemetry_metrics WHERE synced_to_cloud = TRUE").Scan(&countSynced)
	if err != nil || countSynced != 1 {
		t.Fatalf("Expected 1 synced row after daemon run")
	}
}

func TestBufferMetricHelper_RedactsPII(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()

	engine := NewTelemetrySyncEngine(db, "http://localhost:8080/metrics")
	InitGlobalSyncEngine(engine)

	t.Setenv("OHC_STANDALONE", "true")
	t.Setenv("OHC_TELEMETRY_ENABLED", "true")

	ctx := context.Background()

	attrs := map[string]interface{}{
		"service":   "agent",
		"tenant_id": "secret-tenant-123",
		"email":     "user@example.com",
		"ip_address":  "192.168.1.1",
		"mac_address": "00:1B:44:11:3A:B7",
		"geolocation": "37.7749,-122.4194",
		"nested": map[string]interface{}{
			"password": "my-secret-password",
			"safe":     "value",
		},
	}

	bufferMetricHelper(ctx, "test_redaction_metric", 1.0, attrs)

	var attrStr string
	err := db.QueryRow("SELECT attributes FROM local_telemetry_metrics WHERE metric_name = 'test_redaction_metric'").Scan(&attrStr)
	if err != nil {
		t.Fatalf("Failed to query db: %v", err)
	}

	var storedAttrs map[string]interface{}
	if err := json.Unmarshal([]byte(attrStr), &storedAttrs); err != nil {
		t.Fatalf("Failed to unmarshal attributes: %v", err)
	}

	if storedAttrs["service"] != "agent" {
		t.Errorf("Expected 'service' to be 'agent', got %v", storedAttrs["service"])
	}
	if storedAttrs["tenant_id"] != "[REDACTED]" {
		t.Errorf("Expected 'tenant_id' to be redacted, got %v", storedAttrs["tenant_id"])
	}
	if storedAttrs["email"] != "[REDACTED]" {
		t.Errorf("Expected 'email' to be redacted, got %v", storedAttrs["email"])
	}
	if storedAttrs["ip_address"] != "[REDACTED]" {
		t.Errorf("Expected 'ip_address' to be redacted, got %v", storedAttrs["ip_address"])
	}
	if storedAttrs["mac_address"] != "[REDACTED]" {
		t.Errorf("Expected 'mac_address' to be redacted, got %v", storedAttrs["mac_address"])
	}
	if storedAttrs["geolocation"] != "[REDACTED]" {
		t.Errorf("Expected 'geolocation' to be redacted, got %v", storedAttrs["geolocation"])
	}

	nested, ok := storedAttrs["nested"].(map[string]interface{})
	if !ok {
		t.Fatalf("Expected nested to be a map")
	}
	if nested["password"] != "[REDACTED]" {
		t.Errorf("Expected nested 'password' to be redacted, got %v", nested["password"])
	}
	if nested["safe"] != "value" {
		t.Errorf("Expected nested 'safe' to be 'value', got %v", nested["safe"])
	}
}
