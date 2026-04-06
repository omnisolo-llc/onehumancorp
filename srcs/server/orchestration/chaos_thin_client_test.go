package orchestration

import (
	"context"
	"net/http"
	"net/http/httptest"
	"path/filepath"
	"testing"
	"time"
)

// TestSIPDB_ThinClient_LatencySpike verifies that when the remote backend is slow
// the sync daemon and other HTTP clients timeout properly and don't block forever.
func TestSIPDB_ThinClient_LatencySpike(t *testing.T) {
	tmpDir := t.TempDir()
	dbPath := filepath.Join(tmpDir, "thin_client_latency.db")

	dbInstance, err := NewSIPDB(dbPath)
	if err != nil {
		t.Fatalf("Failed to create SIPDB: %v", err)
	}
	defer dbInstance.Close()

	// Simulate a slow remote backend
	slowServer := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		time.Sleep(2 * time.Second) // Force a timeout
		w.WriteHeader(http.StatusOK)
	}))
	defer slowServer.Close()

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	// Add some buffered metrics to sync
	err = dbInstance.BufferMetric(ctx, "test-metric", "{\"val\": 1}")
	if err != nil {
		t.Fatalf("Failed to buffer metric: %v", err)
	}

	// Overwrite HTTP client timeout inside the SIPDB just for this test
	// Actually SIPDB SyncBufferedMetrics uses hardcoded 10 second timeout.
	// But let's verify that even if we cancel the context, it respects it.

	shortCtx, shortCancel := context.WithTimeout(ctx, 100*time.Millisecond)
	defer shortCancel()

	start := time.Now()
	_, err = dbInstance.SyncBufferedMetrics(shortCtx, slowServer.URL)
	duration := time.Since(start)

	if err == nil {
		t.Fatalf("Expected SyncBufferedMetrics to fail due to context deadline exceeded, but it succeeded")
	}

	if duration > 1*time.Second {
		t.Errorf("Thin client took too long to fail-safe (%v), expected to timeout quickly", duration)
	} else {
		t.Logf("Thin client fail-safe triggered successfully in %v: %v", duration, err)
	}

	// 2. Test SyncContextSync fail-safe
	err = dbInstance.StoreEpisodicMemory(ctx, EpisodicMemory{
		MemoryID: "mem-1",
		Context:  "context",
	})
	if err != nil {
		t.Fatalf("Failed to store episodic memory: %v", err)
	}

	shortCtx2, shortCancel2 := context.WithTimeout(ctx, 100*time.Millisecond)
	defer shortCancel2()

	start2 := time.Now()
	_, err = dbInstance.SyncContextSync(shortCtx2, slowServer.URL)
	duration2 := time.Since(start2)

	// Since SyncContextSync ignores individual request errors in its loop, we check duration
	if duration2 > 1*time.Second {
		t.Errorf("SyncContextSync took too long to fail-safe (%v), expected to timeout quickly", duration2)
	} else {
		t.Logf("SyncContextSync fail-safe triggered successfully in %v", duration2)
	}

	// 3. Test HTTPCloudClient fail-safe
	client := NewHTTPCloudClient(slowServer.URL)

	shortCtx3, shortCancel3 := context.WithTimeout(ctx, 100*time.Millisecond)
	defer shortCancel3()

	start3 := time.Now()
	_, err = client.PushSanitizedMemory(shortCtx3, "mem-1", "context")
	duration3 := time.Since(start3)

	if err == nil {
		t.Fatalf("Expected PushSanitizedMemory to fail due to context deadline exceeded, but it succeeded")
	}

	if duration3 > 1*time.Second {
		t.Errorf("HTTPCloudClient took too long to fail-safe (%v), expected to timeout quickly", duration3)
	} else {
		t.Logf("HTTPCloudClient fail-safe triggered successfully in %v: %v", duration3, err)
	}
}

// TestSIPDB_ThinClient_ConnectionDrop verifies behavior when the connection is refused.
func TestSIPDB_ThinClient_ConnectionDrop(t *testing.T) {
	tmpDir := t.TempDir()
	dbPath := filepath.Join(tmpDir, "thin_client_conn_drop.db")

	dbInstance, err := NewSIPDB(dbPath)
	if err != nil {
		t.Fatalf("Failed to create SIPDB: %v", err)
	}
	defer dbInstance.Close()

	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()

	// Use a non-routable / non-listening port
	badURL := "http://127.0.0.1:0"

	err = dbInstance.BufferMetric(ctx, "test-metric", "{\"val\": 1}")
	if err != nil {
		t.Fatalf("Failed to buffer metric: %v", err)
	}

	start := time.Now()
	_, err = dbInstance.SyncBufferedMetrics(ctx, badURL)
	duration := time.Since(start)

	if err == nil {
		t.Fatalf("Expected SyncBufferedMetrics to fail due to connection refused, but it succeeded")
	}
	t.Logf("SyncBufferedMetrics fail-safe on connection drop triggered successfully in %v: %v", duration, err)
}
