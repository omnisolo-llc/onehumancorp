package network

import (
	"io"
	"net/http"
	"net/http/httptest"
	"net/url"
	"os"
	"testing"
	"time"
)

func TestNetworkBridgeProxy_BlocksInvalidDomains(t *testing.T) {
	proxy := NewNetworkBridgeProxy("/tmp/test-proxy-blocked.sock", []string{"example.com"})
	err := proxy.Start()
	if err != nil {
		t.Fatalf("Failed to start proxy: %v", err)
	}
	defer proxy.Stop()

	// Wait for the unix socket to be ready
	for i := 0; i < 10; i++ {
		if proxy.IsReady() {
			if _, err := os.Stat("/tmp/test-proxy-blocked.sock"); err == nil {
				break
			}
		}
		time.Sleep(100 * time.Millisecond)
	}

	// Create a test server to represent the proxy listener (we can also test using standard HTTP via unix socket)
    // Wait, the easiest way to test is to create an http client using the unix socket as transport

    // Custom transport that dials the unix socket
    // However, the proxy expects normal HTTP requests with Host headers.
    // Actually, we can use the http.Client with a custom Transport

    // Instead of dealing with unix socket dialing in the test, we can just test the HTTP handler directly
    // since we export the server/listener
    req, _ := http.NewRequest("GET", "http://forbidden.com/", nil)
    req.Host = "forbidden.com"

    // Record the response
    rr := httptest.NewRecorder()

    // The handler is in proxy.server.Handler
    proxy.server.Handler.ServeHTTP(rr, req)

    if rr.Code != http.StatusForbidden {
        t.Errorf("Expected status 403 Forbidden, got %d", rr.Code)
    }
}

func TestNetworkBridgeProxy_ForwardsValidDomains(t *testing.T) {
	// Start a backend test server
    backendServer := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
        io.WriteString(w, "OK")
    }))
    defer backendServer.Close()

    backendURL, _ := url.Parse(backendServer.URL)
    host := backendURL.Hostname()

	proxy := NewNetworkBridgeProxy("/tmp/test-proxy-allowed.sock", []string{host})
	err := proxy.Start()
	if err != nil {
		t.Fatalf("Failed to start proxy: %v", err)
	}
	defer proxy.Stop()

	for i := 0; i < 10; i++ {
		if proxy.IsReady() {
			if _, err := os.Stat("/tmp/test-proxy-allowed.sock"); err == nil {
				break
			}
		}
		time.Sleep(100 * time.Millisecond)
	}

    req, _ := http.NewRequest("GET", backendServer.URL+"/", nil)
    req.Host = host

    rr := httptest.NewRecorder()

    proxy.server.Handler.ServeHTTP(rr, req)

    if rr.Code != http.StatusOK {
        t.Errorf("Expected status 200 OK, got %d", rr.Code)
    }

    body, _ := io.ReadAll(rr.Body)
    if string(body) != "OK" {
        t.Errorf("Expected body 'OK', got '%s'", string(body))
    }
}
