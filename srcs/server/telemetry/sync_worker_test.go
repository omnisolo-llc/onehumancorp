package telemetry

import (
	"context"
	"database/sql"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"os"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func newTestProvider(t *testing.T) db.Provider {
	t.Helper()
	dbInstance, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open test sqlite db: %v", err)
	}

	if err := dbInstance.PingContext(context.Background()); err != nil {
		t.Fatalf("failed to ping test sqlite db: %v", err)
	}

	t.Cleanup(func() {
		dbInstance.Close()
	})

	return db.NewSqliteProvider(dbInstance)
}

func TestBufferMetricToSQLite(t *testing.T) {
	// Enable telemetry so it actually writes to the mock DB.
	os.Setenv("OHC_STANDALONE", "true")
	os.Setenv("OHC_TELEMETRY_ENABLED", "true")
	defer os.Unsetenv("OHC_STANDALONE")
	defer os.Unsetenv("OHC_TELEMETRY_ENABLED")

	pool := newTestProvider(t)
	ctx := context.Background()

	// Ensure the table exists.
	_, err := pool.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS telemetry_buffer (
			id INTEGER PRIMARY KEY AUTOINCREMENT,
			metric_type VARCHAR NOT NULL,
			payload TEXT NOT NULL,
			created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
		)
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	bufferFunc := BufferMetricToSQLite(pool)
	err = bufferFunc(ctx, "test_metric", `{"key":"value"}`)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	var count int
	err = pool.QueryRow(ctx, "SELECT COUNT(*) FROM telemetry_buffer").Scan(&count)
	if err != nil {
		t.Fatalf("failed to query count: %v", err)
	}
	if count != 1 {
		t.Fatalf("expected 1 record, got %d", count)
	}

	// Test opt-in enforcement (disabled telemetry)
	os.Setenv("OHC_TELEMETRY_ENABLED", "false")
	err = bufferFunc(ctx, "test_metric_disabled", `{"key":"value"}`)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	var countAfter int
	err = pool.QueryRow(ctx, "SELECT COUNT(*) FROM telemetry_buffer").Scan(&countAfter)
	if err != nil {
		t.Fatalf("failed to query count: %v", err)
	}
	if countAfter != 1 {
		t.Fatalf("expected 1 record still, got %d", countAfter) // Should not have inserted
	}
}

func TestSyncMetrics(t *testing.T) {
	pool := newTestProvider(t)
	ctx := context.Background()

	// Ensure the table exists.
	_, err := pool.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS telemetry_buffer (
			id INTEGER PRIMARY KEY AUTOINCREMENT,
			metric_type VARCHAR NOT NULL,
			payload TEXT NOT NULL,
			created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
		)
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	// Insert test data
	_, err = pool.Exec(ctx, "INSERT INTO telemetry_buffer (metric_type, payload) VALUES ($1, $2), ($3, $4)",
		"metric_1", `{"a":1}`, "metric_2", `{"b":2}`)
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}

	receivedHeader := ""
	var receivedRecords []MetricRecord

	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		receivedHeader = r.Header.Get("X-OHC-Conflict-Resolution")
		if err := json.NewDecoder(r.Body).Decode(&receivedRecords); err != nil {
			t.Errorf("failed to decode request body: %v", err)
		}
		w.WriteHeader(http.StatusOK)
	}))
	defer ts.Close()

	count, err := syncMetrics(ctx, pool, ts.URL)
	if err != nil {
		t.Fatalf("syncMetrics failed: %v", err)
	}
	if count != 2 {
		t.Fatalf("expected to sync 2 records, got %d", count)
	}

	if receivedHeader != "force-local" {
		t.Errorf("expected force-local header, got %s", receivedHeader)
	}

	if len(receivedRecords) != 2 {
		t.Errorf("expected 2 records synced, got %d", len(receivedRecords))
	}

	var countAfter int
	err = pool.QueryRow(ctx, "SELECT COUNT(*) FROM telemetry_buffer").Scan(&countAfter)
	if err != nil {
		t.Fatalf("failed to query count: %v", err)
	}
	if countAfter != 0 {
		t.Errorf("expected 0 records after successful sync, got %d", countAfter)
	}
}

func TestSyncWorkerIntegration(t *testing.T) {
	pool := newTestProvider(t)
	ctx, cancel := context.WithCancel(context.Background())

	_, err := pool.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS telemetry_buffer (
			id INTEGER PRIMARY KEY AUTOINCREMENT,
			metric_type VARCHAR NOT NULL,
			payload TEXT NOT NULL,
			created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
		)
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
	}))
	defer ts.Close()

	// Insert test data
	_, err = pool.Exec(ctx, "INSERT INTO telemetry_buffer (metric_type, payload) VALUES ($1, $2)", "metric_test", `{"a":1}`)
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}

	// We override the time inside StartSyncWorker for tests? It's hardcoded to 1 minute, but syncMetrics works.
	// Since StartSyncWorker uses a long ticker, let's just make sure the go routine spins up without crashing.
	StartSyncWorker(ctx, pool, ts.URL)
	cancel()
	time.Sleep(10 * time.Millisecond) // Allow go routine to exit
}
