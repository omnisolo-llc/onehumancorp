package dashboard

import (
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"sync/atomic"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// ChaosMiddleware simulates network latency and backend degradation.
// In Thin Client mode, clients will rely on API boundaries handling backpressure
// and latency gracefully. This middleware injects delays to simulate a busy
// shared-database or network partitions.
func ChaosMiddleware(next http.Handler, delay time.Duration, shouldFail *atomic.Bool) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if shouldFail.Load() {
			http.Error(w, `{"error": "backend_unavailable", "message": "Service degraded"}`, http.StatusServiceUnavailable)
			return
		}

		if delay > 0 {
			time.Sleep(delay)
		}

		next.ServeHTTP(w, r)
	})
}

// TestThinClientDegradation verifies that dashboard routes fail gracefully
// and adhere to context timeouts under high latency (simulating Thin Client degradation).
func TestThinClientDegradation(t *testing.T) {
	app, _, _ := newTestServer(t)

	shouldFail := &atomic.Bool{}
	shouldFail.Store(false)

	// Wrap the main handler with ChaosMiddleware
	chaosHandler := ChaosMiddleware(app.Mux(), 50*time.Millisecond, shouldFail)

	// We'll create a standalone test server instance that wraps the main mux with chaos
	srv := httptest.NewServer(chaosHandler)
	defer srv.Close()

	client := srv.Client()
	client.Timeout = 100 * time.Millisecond

	// Setup a valid auth token to bypass 401s
	token, _ := app.authStore.GenerateToken("user-1", "org-1", "admin")

	// Helper to send a request
	doReq := func() (*http.Response, error) {
		req, _ := http.NewRequest("GET", srv.URL+"/api/orgs/my", nil)
		req.Header.Set("Authorization", "Bearer "+token)
		return client.Do(req)
	}

	t.Run("Normal Latency (Graceful Success)", func(t *testing.T) {
		resp, err := doReq()
		if err != nil {
			t.Fatalf("Expected success, got error: %v", err)
		}
		defer resp.Body.Close()
		if resp.StatusCode != http.StatusOK {
			t.Errorf("Expected 200 OK, got %d", resp.StatusCode)
		}
	})

	t.Run("High Latency (Graceful Client Timeout)", func(t *testing.T) {
		// Re-wrap with higher latency exceeding client timeout
		slowHandler := ChaosMiddleware(app.Mux(), 200*time.Millisecond, shouldFail)
		slowSrv := httptest.NewServer(slowHandler)
		defer slowSrv.Close()

		slowClient := slowSrv.Client()
		slowClient.Timeout = 50 * time.Millisecond

		req, _ := http.NewRequest("GET", slowSrv.URL+"/api/orgs/my", nil)
		req.Header.Set("Authorization", "Bearer "+token)

		start := time.Now()
		_, err := slowClient.Do(req)
		elapsed := time.Since(start)

		if err == nil {
			t.Fatalf("Expected client timeout error, got nil")
		}

		// The error should be a context deadline exceeded or similar timeout error
		if !err.(interface{ Timeout() bool }).Timeout() {
			t.Errorf("Expected timeout error, got: %v", err)
		}

		if elapsed < 50*time.Millisecond {
			t.Errorf("Expected request to block until timeout (50ms), but it returned in %v", elapsed)
		}
	})

	t.Run("Backend Failure (Graceful 503)", func(t *testing.T) {
		shouldFail.Store(true)

		resp, err := doReq()
		if err != nil {
			t.Fatalf("Expected request to complete with 503, got network error: %v", err)
		}
		defer resp.Body.Close()

		if resp.StatusCode != http.StatusServiceUnavailable {
			t.Errorf("Expected 503 Service Unavailable, got %d", resp.StatusCode)
		}

		var payload map[string]string
		if err := json.NewDecoder(resp.Body).Decode(&payload); err != nil {
			t.Fatalf("Expected valid JSON response, got error: %v", err)
		}

		if payload["error"] != "backend_unavailable" {
			t.Errorf("Expected error key to be 'backend_unavailable', got '%s'", payload["error"])
		}
	})
}

// TestServerCtxCancellation verifies that the backend HTTP handler correctly stops
// processing when the client drops the connection prematurely (Thin Client drop).
func TestServerCtxCancellation(t *testing.T) {
	app, _, _ := newTestServer(t)

	// Create a custom handler that simulates a long-running operation
	// but listens for context cancellation.
	longRunningProcessed := &atomic.Bool{}
	app.Mux().HandleFunc("/api/test/long-running", func(w http.ResponseWriter, r *http.Request) {
		select {
		case <-time.After(200 * time.Millisecond):
			// Should not reach here if client cancels
			longRunningProcessed.Store(true)
			w.WriteHeader(http.StatusOK)
		case <-r.Context().Done():
			// Client disconnected early
			longRunningProcessed.Store(false)
			w.WriteHeader(http.StatusRequestTimeout) // Usually framework/Go writes 499 for context canceled
			return
		}
	})

	srv := httptest.NewServer(app.Mux())
	defer srv.Close()

	ctx, cancel := context.WithCancel(context.Background())
	req, _ := http.NewRequestWithContext(ctx, "GET", srv.URL+"/api/test/long-running", nil)

	// Start the request in a goroutine
	go func() {
		client := srv.Client()
		_, _ = client.Do(req)
	}()

	// Wait briefly, then cancel the client request to simulate Thin Client network drop
	time.Sleep(50 * time.Millisecond)
	cancel()

	// Wait enough time to ensure the long operation would have completed if not cancelled
	time.Sleep(300 * time.Millisecond)

	if longRunningProcessed.Load() {
		t.Errorf("Expected long running operation to be aborted via context cancellation, but it completed")
	}
}
