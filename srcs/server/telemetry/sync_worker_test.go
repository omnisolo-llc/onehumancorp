package telemetry

import (
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"os"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/dbtest"
)

func TestSyncWorker(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	os.Setenv("OHC_TELEMETRY_ENABLED", "true")
	defer os.Unsetenv("OHC_STANDALONE")
	defer os.Unsetenv("OHC_TELEMETRY_ENABLED")

	prov := dbtest.NewTestProvider(t)
	defer prov.Close()

	ctx := context.Background()

	_, err := prov.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS telemetry_buffer (
			id TEXT PRIMARY KEY DEFAULT (hex(randomblob(16))),
			metric_type TEXT NOT NULL,
			payload TEXT NOT NULL,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.Header.Get("X-OHC-Conflict-Resolution") != "force-local" {
			t.Errorf("missing header")
		}
		var data []map[string]interface{}
		json.NewDecoder(r.Body).Decode(&data)
		if len(data) != 1 {
			t.Errorf("expected 1 record, got %d", len(data))
		}
		w.WriteHeader(http.StatusOK)
	}))
	defer server.Close()

	// Insert test data
	_, err = prov.Exec(ctx, "INSERT INTO telemetry_buffer (metric_type, payload) VALUES ($1, $2)", "test_metric", `{"foo":"bar"}`)
	if err != nil {
		t.Fatalf("failed to insert: %v", err)
	}

	worker := NewSyncWorker(prov, server.URL)

	// Run sync synchronously for test
	worker.sync(ctx)

	// Verify it was deleted
	var count int
	err = prov.QueryRow(ctx, "SELECT COUNT(*) FROM telemetry_buffer").Scan(&count)
	if err != nil {
		t.Fatalf("failed to query count: %v", err)
	}
	if count != 0 {
		t.Errorf("expected 0 records after sync, got %d", count)
	}
}

func TestSQLiteBufferMetric(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	os.Setenv("OHC_TELEMETRY_ENABLED", "true")
	defer os.Unsetenv("OHC_STANDALONE")
	defer os.Unsetenv("OHC_TELEMETRY_ENABLED")

	prov := dbtest.NewTestProvider(t)
	defer prov.Close()

	ctx := context.Background()

	_, err := prov.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS telemetry_buffer (
			id TEXT PRIMARY KEY DEFAULT (hex(randomblob(16))),
			metric_type TEXT NOT NULL,
			payload TEXT NOT NULL,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	bufferFn := SQLiteBufferMetric(prov)
	err = bufferFn(ctx, "test_metric", `{"foo":"bar"}`)
	if err != nil {
		t.Fatalf("expected no error: %v", err)
	}

	// wait a bit since it's a goroutine
	time.Sleep(100 * time.Millisecond)

	var count int
	err = prov.QueryRow(ctx, "SELECT COUNT(*) FROM telemetry_buffer").Scan(&count)
	if err != nil {
		t.Fatalf("failed to query count: %v", err)
	}
	if count != 1 {
		t.Errorf("expected 1 record, got %d", count)
	}
}
