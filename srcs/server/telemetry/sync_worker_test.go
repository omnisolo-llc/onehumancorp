package telemetry

import (
	"context"
	"encoding/json"
	"io"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestSyncWorker(t *testing.T) {
	// Use in-memory SQLite for testing
	t.Setenv("DATABASE_URL", "sqlite://:memory:")
	t.Setenv("OHC_STANDALONE", "true")
	t.Setenv("OHC_TELEMETRY_ENABLED", "true")

	ctx := context.Background()
	database, err := db.New(ctx)
	if err != nil {
		t.Fatalf("failed to init db: %v", err)
	}
	defer database.Close()

	if err := database.RunMigrations(ctx); err != nil {
		t.Fatalf("failed to run migrations: %v", err)
	}

	// Create table manually just in case the migration is not embedded correctly in tests
	_, err = database.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS telemetry_buffer (
			id INTEGER PRIMARY KEY AUTOINCREMENT,
			metric_type TEXT NOT NULL,
			payload TEXT NOT NULL,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	InitBufferMetricFunc(database)
	if BufferMetricFunc == nil {
		t.Fatal("expected BufferMetricFunc to be set")
	}

	err = BufferMetricFunc(ctx, "test_metric", `{"key": "value"}`)
	if err != nil {
		t.Fatalf("BufferMetricFunc failed: %v", err)
	}

	var count int
	err = database.QueryRow(ctx, "SELECT COUNT(*) FROM telemetry_buffer").Scan(&count)
	if err != nil {
		t.Fatalf("failed to query count: %v", err)
	}
	if count != 1 {
		t.Fatalf("expected 1 record, got %d", count)
	}

	var receivedMetrics []BufferedMetric
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.Header.Get("X-OHC-Conflict-Resolution") != "force-local" {
			t.Errorf("missing header: %s", r.Header.Get("X-OHC-Conflict-Resolution"))
		}

		body, _ := io.ReadAll(r.Body)
		err := json.Unmarshal(body, &receivedMetrics)
		if err != nil {
			w.WriteHeader(http.StatusBadRequest)
			return
		}
		w.WriteHeader(http.StatusOK)
	}))
	defer server.Close()

	// Replace the default client to point to our test server?
	// Wait, we can just pass the server.URL as endpoint to syncBatch
	err = syncBatch(ctx, database, server.URL)
	if err != nil {
		t.Fatalf("syncBatch failed: %v", err)
	}

	if len(receivedMetrics) != 1 {
		t.Fatalf("expected 1 metric synced, got %d", len(receivedMetrics))
	}
	if receivedMetrics[0].MetricType != "test_metric" {
		t.Errorf("expected test_metric, got %s", receivedMetrics[0].MetricType)
	}

	// Verify deletion
	err = database.QueryRow(ctx, "SELECT COUNT(*) FROM telemetry_buffer").Scan(&count)
	if err != nil {
		t.Fatalf("failed to query count: %v", err)
	}
	if count != 0 {
		t.Fatalf("expected 0 records after sync, got %d", count)
	}
}

func TestSyncWorkerDisabled(t *testing.T) {
	t.Setenv("DATABASE_URL", "sqlite://:memory:")
	t.Setenv("OHC_STANDALONE", "true")
	t.Setenv("OHC_TELEMETRY_ENABLED", "false")

	ctx := context.Background()
	database, err := db.New(ctx)
	if err != nil {
		t.Fatalf("failed to init db: %v", err)
	}
	defer database.Close()

	InitBufferMetricFunc(database)
	if BufferMetricFunc != nil {
		t.Fatal("expected BufferMetricFunc to be nil when disabled")
	}
}
