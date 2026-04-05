package telemetry

import (
	"context"
	"encoding/json"
	"io"
	"net/http"
	"net/http/httptest"
	"os"
	"testing"
	"time"

)

func TestSyncWorker(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	provider := newTestProvider(t)
	defer provider.Close()

	// Insert test data
	_, err := provider.Exec(ctx, "INSERT INTO telemetry_buffer (id, metric_type, payload) VALUES (?, ?, ?)", "id-1", "test", "{\"val\": 1}")
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	serverCalled := false
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		serverCalled = true
		if r.Header.Get("X-OHC-Conflict-Resolution") != "force-local" {
			t.Errorf("expected force-local header")
		}

		body, _ := io.ReadAll(r.Body)
		var payload map[string]interface{}
		if err := json.Unmarshal(body, &payload); err != nil {
			t.Errorf("invalid json payload: %v", err)
		}

		metrics := payload["metrics"].([]interface{})
		if len(metrics) != 1 {
			t.Errorf("expected 1 metric, got %d", len(metrics))
		}
		w.WriteHeader(http.StatusOK)
	}))
	defer server.Close()

	worker := NewSyncWorker(provider, server.URL)
	worker.sync(ctx)

	if !serverCalled {
		t.Errorf("expected sync server to be called")
	}

	var count int
	row := provider.QueryRow(ctx, "SELECT COUNT(*) FROM telemetry_buffer")
	if err := row.Scan(&count); err != nil {
		t.Fatalf("failed to count telemetry_buffer: %v", err)
	}
	if count != 0 {
		t.Errorf("expected 0 records in telemetry_buffer after sync, got %d", count)
	}
}

func TestSyncWorkerFailure(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	provider := newTestProvider(t)
	defer provider.Close()

	_, err := provider.Exec(ctx, "INSERT INTO telemetry_buffer (id, metric_type, payload) VALUES (?, ?, ?)", "id-2", "test", "{\"val\": 2}")
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	serverCalled := false
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		serverCalled = true
		w.WriteHeader(http.StatusInternalServerError)
	}))
	defer server.Close()

	worker := NewSyncWorker(provider, server.URL)
	worker.sync(ctx)

	if !serverCalled {
		t.Errorf("expected sync server to be called")
	}

	var count int
	row := provider.QueryRow(ctx, "SELECT COUNT(*) FROM telemetry_buffer")
	if err := row.Scan(&count); err != nil {
		t.Fatalf("failed to count telemetry_buffer: %v", err)
	}
	if count != 1 {
		t.Errorf("expected 1 record in telemetry_buffer after failed sync, got %d", count)
	}
}
