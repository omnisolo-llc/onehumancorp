package orchestration

import (
	"context"
	"net/http"
	"net/http/httptest"
	"path/filepath"
	"testing"
	"time"
)

func TestSIPDB_NetworkPartition_Chaos(t *testing.T) {
	defer ClearSemaphore()
	tmpDir := t.TempDir()
	dbPath := filepath.Join(tmpDir, "chaos_network.db")

	db, err := NewSIPDB(dbPath)
	if err != nil {
		t.Fatalf("Failed to create SIPDB: %v", err)
	}
	defer db.Close()

	ctx := context.Background()

	// Add a metric
	err = db.BufferMetric(ctx, "test_metric", "{\"val\": 1}")
	if err != nil {
		t.Fatalf("Failed to buffer metric: %v", err)
	}

	// Add a pending mission
	err = db.UpsertMission(ctx, "mission-1", "PENDING", "{\"role\": \"test\"}", false)
	if err != nil {
		t.Fatalf("Failed to upsert mission: %v", err)
	}

	// 1. Simulate 503 error
	server503 := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusServiceUnavailable)
	}))
	defer server503.Close()

	synced, err := db.SyncBufferedMetrics(ctx, server503.URL)
	if err == nil {
		t.Errorf("Expected error when remote endpoint returns 503, got nil")
	}
	if synced != 0 {
		t.Errorf("Expected 0 synced metrics, got %d", synced)
	}

	synced, err = db.SyncMissions(ctx, server503.URL)
	if synced != 0 {
		t.Errorf("Expected 0 synced missions, got %d", synced)
	}

	// 2. Simulate timeout/connection drop
	serverTimeout := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		time.Sleep(2 * time.Second) // simulate hang
	}))
	defer serverTimeout.Close()

	ctxTimeout, cancel := context.WithTimeout(ctx, 100*time.Millisecond)
	defer cancel()

	_, err = db.SyncBufferedMetrics(ctxTimeout, serverTimeout.URL)
	if err == nil {
		t.Errorf("Expected timeout error, got nil")
	}

	// Verify data is still intact locally (fail-safe)
	metricsCount := 0
	rows, _ := db.db.Query(ctx, "SELECT id FROM local_metrics_buffer")
	for rows.Next() {
		metricsCount++
	}
	rows.Close()
	if metricsCount != 1 {
		t.Errorf("Expected metric to remain in local DB after failed sync, got %d", metricsCount)
	}

	// Verify mission is still PENDING locally
	missions, _ := db.GetPendingMissions(ctx, "test")
	if len(missions) != 1 {
		t.Errorf("Expected mission to remain PENDING in local DB after failed sync, got %d", len(missions))
	}

	// 3. Simulate success
	serverSuccess := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
	}))
	defer serverSuccess.Close()

	synced, err = db.SyncBufferedMetrics(ctx, serverSuccess.URL)
	if err != nil {
		t.Errorf("Failed to sync metrics: %v", err)
	}
	if synced != 1 {
		t.Errorf("Expected 1 synced metric, got %d", synced)
	}

	synced, err = db.SyncMissions(ctx, serverSuccess.URL)
	if err != nil {
		t.Errorf("Failed to sync missions: %v", err)
	}
	if synced != 1 {
		t.Errorf("Expected 1 synced mission, got %d", synced)
	}

	// Ensure data is cleared/updated locally
	metricsCount = 0
	rows, _ = db.db.Query(ctx, "SELECT id FROM local_metrics_buffer")
	for rows.Next() {
		metricsCount++
	}
	rows.Close()
	if metricsCount != 0 {
		t.Errorf("Expected metric to be deleted after sync, got %d", metricsCount)
	}
}
