package network

import (
	"io"
	"net/http"
	"net/http/httptest"
	"net/url"
	"os/exec"
	"strings"
	"testing"
	"time"
)

func TestNetworkBridge(t *testing.T) {
	// Start a dummy backend server to proxy to
	backend := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
		w.Write([]byte("Backend Response"))
	}))
	defer backend.Close()

	backendURL, _ := url.Parse(backend.URL)
	backendHost := backendURL.Host

	bridge := NewNetworkBridge("/tmp/test_bridge.sock", []string{strings.Split(backendHost, ":")[0]})
	err := bridge.Start()
	if err != nil {
		t.Fatalf("Failed to start bridge: %v", err)
	}
	defer bridge.Stop()

	time.Sleep(100 * time.Millisecond)

	// Simulate client side (inside bwrap)
	clientSocat := exec.Command("socat", "TCP-LISTEN:8081,fork", "UNIX-CLIENT:/tmp/test_bridge.sock")
	err = clientSocat.Start()
	if err != nil {
		t.Fatalf("Failed to start client socat: %v", err)
	}
	defer clientSocat.Process.Kill()

	time.Sleep(100 * time.Millisecond)

	// Test allowed domain
	req, _ := http.NewRequest("GET", backend.URL, nil)
	proxyUrl, _ := url.Parse("http://127.0.0.1:8081")
	client := &http.Client{Transport: &http.Transport{Proxy: http.ProxyURL(proxyUrl)}}

	resp, err := client.Do(req)
	if err != nil {
		t.Fatalf("Request failed: %v", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		t.Errorf("Expected status 200, got %d", resp.StatusCode)
	}

	body, _ := io.ReadAll(resp.Body)
	if string(body) != "Backend Response" {
		t.Errorf("Expected 'Backend Response', got '%s'", string(body))
	}

	// Test blocked domain
	reqBlocked, _ := http.NewRequest("GET", "http://google.com/", nil)
	respBlocked, err := client.Do(reqBlocked)
	if err != nil {
		t.Fatalf("Request failed: %v", err)
	}
	defer respBlocked.Body.Close()

	if respBlocked.StatusCode != http.StatusForbidden {
		t.Errorf("Expected status 403, got %d", respBlocked.StatusCode)
	}
}
