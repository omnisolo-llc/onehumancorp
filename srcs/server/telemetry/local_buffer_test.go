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

func TestRedactInterfacePIIMaliciousPayloads(t *testing.T) {
	payloadRaw := `{
		"payload": {
			"credit_card": "4111-1111-1111-1111",
			"cvv": "123",
			"dob": "1990-01-01",
			"passport_number": "A1234567",
			"bank_account": "123456789",
			"stripe_token": "tok_123456789",
			"billing_address": "123 Main St, Anytown USA",
			"ssn": "123-45-6789",
			"phone_number": "555-123-4567",
			"email_address": "malicious@example.com",
			"tenant_id": "tenant-123",
			"organization_id": "org-456",
			"session_id": "session-789",
			"ip_address": "192.168.1.1",
			"mac_address": "00:1B:44:11:3A:B7",
			"geolocation": "37.7749,-122.4194"
		},
		"nested": {
			"deep": {
				"secret_key": "sk-1234567890",
				"api_key": "ak-0987654321",
				"auth_token": "Bearer token",
				"password_hash": "hash",
				"cookie_session": "cookie",
				"credential_id": "cred-1"
			}
		},
		"array_of_evil": [
			{ "full_name": "John Doe", "email_address": "john@doe.com" },
			{ "address": "456 Elm St", "phone_number": "555-987-6543" }
		],
		"safe_field": "This should not be redacted",
		"another_safe": 123
	}`

	var payload map[string]interface{}
	if err := json.Unmarshal([]byte(payloadRaw), &payload); err != nil {
		t.Fatalf("failed to unmarshal payload: %v", err)
	}

	redactedRaw := RedactInterfacePII(payload)
	redacted, ok := redactedRaw.(map[string]interface{})
	if !ok {
		t.Fatalf("expected map[string]interface{}, got %T", redactedRaw)
	}

	// Verify root level safe fields
	if redacted["safe_field"] != "This should not be redacted" {
		t.Errorf("expected safe_field to not be redacted, got %v", redacted["safe_field"])
	}
	if redacted["another_safe"].(float64) != 123 {
		t.Errorf("expected another_safe to be 123, got %v", redacted["another_safe"])
	}

	// Because the key is "payload", the entire object gets redacted to "[REDACTED]"
	if redacted["payload"] != "[REDACTED]" {
		t.Errorf("expected payload to be redacted, got %v", redacted["payload"])
	}

	// Verify deeply nested secret redactions
	nested := redacted["nested"].(map[string]interface{})
	deep := nested["deep"].(map[string]interface{})
	if deep["secret_key"] != "[REDACTED]" {
		t.Errorf("expected secret_key to be redacted")
	}
	if deep["api_key"] != "[REDACTED]" {
		t.Errorf("expected api_key to be redacted")
	}
	if deep["auth_token"] != "[REDACTED]" {
		t.Errorf("expected auth_token to be redacted")
	}
	if deep["password_hash"] != "[REDACTED]" {
		t.Errorf("expected password_hash to be redacted")
	}
	if deep["cookie_session"] != "[REDACTED]" {
		t.Errorf("expected cookie_session to be redacted")
	}
	if deep["credential_id"] != "[REDACTED]" {
		t.Errorf("expected credential_id to be redacted")
	}

	// Verify array redactions
	arrayOfEvil := redacted["array_of_evil"].([]interface{})
	item0 := arrayOfEvil[0].(map[string]interface{})
	if item0["full_name"] != "[REDACTED]" {
		t.Errorf("expected item0 full_name to be redacted")
	}
	if item0["email_address"] != "[REDACTED]" {
		t.Errorf("expected item0 email_address to be redacted")
	}

	item1 := arrayOfEvil[1].(map[string]interface{})
	if item1["address"] != "[REDACTED]" {
		t.Errorf("expected item1 address to be redacted")
	}
	if item1["phone_number"] != "[REDACTED]" {
		t.Errorf("expected item1 phone_number to be redacted")
	}
}

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

	// Wait for daemon to run
	time.Sleep(200 * time.Millisecond)

	var countSynced int
	err = db.QueryRow("SELECT count(*) FROM local_telemetry_metrics WHERE synced_to_cloud = TRUE").Scan(&countSynced)
	if err != nil || countSynced != 1 {
		t.Fatalf("Expected 1 synced row after daemon run")
	}
}
