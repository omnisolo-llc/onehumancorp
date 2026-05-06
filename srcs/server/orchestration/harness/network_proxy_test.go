package harness

import (
	"context"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"
)

func TestNetworkProxy_AllowedDomain(t *testing.T) {
	policy := &SandboxPolicy{
		AllowedDomains: []string{"example.com"},
	}
	proxy := NewNetworkProxy(policy)

	req := httptest.NewRequest(http.MethodGet, "http://example.com/test", nil)

	// In test, handleHTTP will try to RoundTrip, let's just test isAllowed directly to avoid actual network calls
	if !proxy.isAllowed(req.Host) {
		t.Errorf("Expected domain %s to be allowed", req.Host)
	}
}

func TestNetworkProxy_DeniedDomain(t *testing.T) {
	policy := &SandboxPolicy{
		AllowedDomains: []string{"example.com"},
	}
	proxy := NewNetworkProxy(policy)

	req := httptest.NewRequest(http.MethodGet, "http://malicious.com/test", nil)
	w := httptest.NewRecorder()

	proxy.handleRequest(w, req)

	res := w.Result()
	if res.StatusCode != http.StatusForbidden {
		t.Errorf("Expected status Forbidden, got %d", res.StatusCode)
	}
}

func TestNetworkProxy_StartStop(t *testing.T) {
	policy := &SandboxPolicy{
		AllowedDomains: []string{"example.com"},
	}
	proxy := NewNetworkProxy(policy)

	go func() {
		err := proxy.Start(":0") // Random port
		if err != nil && err != http.ErrServerClosed {
			t.Errorf("Unexpected error starting server: %v", err)
		}
	}()

	time.Sleep(100 * time.Millisecond) // Give it a moment to start
	err := proxy.Stop(context.Background())
	if err != nil {
		t.Errorf("Failed to stop proxy: %v", err)
	}
}

func TestNetworkProxy_HandleHTTP(t *testing.T) {
	policy := &SandboxPolicy{
		AllowedDomains: []string{"127.0.0.1"},
	}
	proxy := NewNetworkProxy(policy)

	// Create a dummy backend server to proxy to
	backend := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
		w.Write([]byte("backend ok"))
	}))
	defer backend.Close()

	req := httptest.NewRequest(http.MethodGet, backend.URL, nil)
	req.RequestURI = req.URL.RequestURI() // httptest sets this
	w := httptest.NewRecorder()

	proxy.handleRequest(w, req)

	res := w.Result()
	if res.StatusCode != http.StatusOK {
		t.Errorf("Expected status OK, got %d", res.StatusCode)
	}
}

func TestNetworkProxy_HandleTunneling_Error(t *testing.T) {
	policy := &SandboxPolicy{
		AllowedDomains: []string{"example.com"},
	}
	proxy := NewNetworkProxy(policy)

	req := httptest.NewRequest(http.MethodConnect, "http://example.com:invalidport", nil)
	req.Host = "example.com:invalidport"
	w := httptest.NewRecorder()

	proxy.handleRequest(w, req)

	res := w.Result()
	if res.StatusCode != http.StatusServiceUnavailable {
		t.Errorf("Expected status ServiceUnavailable due to bad dial, got %d", res.StatusCode)
	}
}
