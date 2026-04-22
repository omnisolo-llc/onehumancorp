package harness

import (
	"context"
	"io"
	"net"
	"net/http"
	"net/http/httptest"
	"net/url"
	"strings"
	"testing"
	"time"
)

type mockSandboxTelemetryEmitter struct {
	violationCount int
}

func (m *mockSandboxTelemetryEmitter) EmitViolation(ctx context.Context, violationType string, agentID string, path string) {
	m.violationCount++
}

func TestNetworkProxy_ServeHTTP(t *testing.T) {
	mockEmitter := &mockSandboxTelemetryEmitter{}
	proxy := NewNetworkProxy([]string{"example.com"}, "test-agent", mockEmitter)

	// Create test server to act as target destination
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
		w.Write([]byte("Hello, client"))
	}))
	defer ts.Close()

	// Parse test server URL to get host
	tsURL, _ := url.Parse(ts.URL)

	// Test 1: Denied Domain
	req := httptest.NewRequest(http.MethodGet, "http://blocked.com/", nil)
	rr := httptest.NewRecorder()

	proxy.ServeHTTP(rr, req)

	if rr.Code != http.StatusForbidden {
		t.Errorf("Expected forbidden, got %v", rr.Code)
	}
	if mockEmitter.violationCount != 1 {
		t.Errorf("Expected 1 violation emitted, got %v", mockEmitter.violationCount)
	}

	// Add test server host to allowed domains
	proxy.AllowedDomains = append(proxy.AllowedDomains, tsURL.Hostname())

	// Test 2: Allowed Domain
	req = httptest.NewRequest(http.MethodGet, ts.URL, nil)
	req.RequestURI = "" // Required for client.Do inside proxy
	rr = httptest.NewRecorder()

	proxy.ServeHTTP(rr, req)

	if rr.Code != http.StatusOK {
		t.Errorf("Expected OK, got %v", rr.Code)
	}

	body, _ := io.ReadAll(rr.Body)
	if !strings.Contains(string(body), "Hello, client") {
		t.Errorf("Expected 'Hello, client', got %s", string(body))
	}
}

func TestNetworkProxy_HandleConnect(t *testing.T) {
	mockEmitter := &mockSandboxTelemetryEmitter{}
	proxy := NewNetworkProxy([]string{}, "test-agent", mockEmitter)

	// Start a dummy TCP server to simulate the destination
	l, err := net.Listen("tcp", "127.0.0.1:0")
	if err != nil {
		t.Fatalf("Failed to listen: %v", err)
	}
	defer l.Close()

	go func() {
		conn, err := l.Accept()
		if err == nil {
			defer conn.Close()
			conn.Write([]byte("server payload"))
		}
	}()

	proxy.AllowedDomains = append(proxy.AllowedDomains, "127.0.0.1")

	// Create a test server with our proxy as handler (needed for Hijack to work properly in some cases)
	// But httptest.NewServer supports Hijack? Let's use httptest.NewServer
	proxyServer := httptest.NewServer(proxy)
	defer proxyServer.Close()

	// Connect to our proxy
	conn, err := net.DialTimeout("tcp", strings.TrimPrefix(proxyServer.URL, "http://"), 2*time.Second)
	if err != nil {
		t.Fatalf("Failed to dial proxy: %v", err)
	}
	defer conn.Close()

	// Send CONNECT request
	connectReq := "CONNECT " + l.Addr().String() + " HTTP/1.1\r\nHost: " + l.Addr().String() + "\r\n\r\n"
	_, err = conn.Write([]byte(connectReq))
	if err != nil {
		t.Fatalf("Failed to write CONNECT req: %v", err)
	}

	// Read response
	buf := make([]byte, 1024)
	n, err := conn.Read(buf)
	if err != nil {
		t.Fatalf("Failed to read from proxy: %v", err)
	}

	resp := string(buf[:n])
	if !strings.Contains(resp, "200 Connection Established") {
		t.Errorf("Expected 200 Connection Established, got %s", resp)
	}

	// Read the forwarded payload
	n, err = conn.Read(buf)
	if err != nil && err != io.EOF {
		t.Fatalf("Failed to read payload: %v", err)
	}
	payload := string(buf[:n])
	if payload != "server payload" {
		t.Errorf("Expected 'server payload', got '%s'", payload)
	}
}

func TestNetworkProxy_StartStop(t *testing.T) {
	mockEmitter := &mockSandboxTelemetryEmitter{}
	proxy := NewNetworkProxy([]string{"example.com"}, "test-agent", mockEmitter)

	err := proxy.Start()
	if err != nil {
		t.Fatalf("Failed to start proxy: %v", err)
	}

	if proxy.Address == "" {
		t.Errorf("Expected proxy address to be set")
	}

	err = proxy.Stop()
	if err != nil {
		t.Errorf("Failed to stop proxy: %v", err)
	}
}
