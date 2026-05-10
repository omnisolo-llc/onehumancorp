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
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func setupTestDB(t *testing.T) *sql.DB {
	db, err := sql.Open("sqlite3", ":memory:")
	require.NoError(t, err)

	_, err = db.Exec(`
		CREATE TABLE local_telemetry_metrics (
			id TEXT PRIMARY KEY,
			metric_name TEXT NOT NULL,
			value REAL NOT NULL,
			attributes TEXT NOT NULL,
			timestamp DATETIME NOT NULL,
			synced_to_cloud BOOLEAN NOT NULL DEFAULT FALSE
		)
	`)
	require.NoError(t, err)
	return db
}

func TestTelemetrySyncEngine_AddMetric(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()
	engine := NewTelemetrySyncEngine(db, "http://test")

	err := engine.BufferMetric(context.Background(), "test_metric", 42.0, map[string]interface{}{"key": "value"})
	assert.NoError(t, err)

	var count int
	err = db.QueryRow("SELECT COUNT(*) FROM local_telemetry_metrics").Scan(&count)
	require.NoError(t, err)
	assert.Equal(t, 1, count)
}

func TestTelemetrySyncEngine_StartSyncDaemon(t *testing.T) {
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
		if pt.MetricName != "test_metric_daemon" {
			t.Errorf("Expected metric name 'test_metric_daemon', got '%s'", pt.MetricName)
		}
		w.WriteHeader(http.StatusOK)
	}))
	defer server.Close()

	engine := NewTelemetrySyncEngine(db, server.URL)

	// Add a metric to the buffer
	err := engine.BufferMetric(context.Background(), "test_metric_daemon", 42.0, map[string]interface{}{"k": "v"})
	require.NoError(t, err)

	// Start daemon
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()
	go engine.StartSyncDaemon(ctx, 10*time.Millisecond)

	// Wait for sync to happen
	time.Sleep(50 * time.Millisecond)

	var syncedCount int
	err = db.QueryRow("SELECT COUNT(*) FROM local_telemetry_metrics WHERE synced_to_cloud = true").Scan(&syncedCount)
	require.NoError(t, err)
	assert.Equal(t, 1, syncedCount)
}

func TestTelemetrySyncEngine_SyncToCloud_Error(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()

	// Mock an endpoint that returns 500
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusInternalServerError)
	}))
	defer server.Close()

	engine := NewTelemetrySyncEngine(db, server.URL)

	err := engine.syncToCloud(context.Background(), MetricPoint{MetricName: "test", Value: 1})
	assert.Error(t, err)
	assert.Contains(t, err.Error(), "unexpected status code")
}
