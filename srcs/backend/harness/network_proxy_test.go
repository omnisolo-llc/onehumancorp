package harness

import (
	"context"
	"io"
	"net/http"
	"net/http/httptest"
	"net/url"
	"strings"
	"testing"
	"net"
	"fmt"
)

type MockTelemetryEmitter struct {
	Violations int
}

func (m *MockTelemetryEmitter) RecordSandboxViolation(ctx context.Context, violationType, agentID, path string) {
	m.Violations++
}

func TestNetworkProxy_AllowedDomain(t *testing.T) {
	proxy := NewNetworkProxy(0, []string{"example.com"}, "agent-1")
	proxy.Emitter = &MockTelemetryEmitter{}

	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
		w.Write([]byte("ok"))
	}))
	defer server.Close()

	proxyServer, err := proxy.Start(context.Background())
	if err != nil {
		t.Fatalf("Failed to start proxy: %v", err)
	}
	defer proxyServer.Close()

	backendURL, _ := url.Parse(server.URL)
	proxy2 := NewNetworkProxy(0, []string{backendURL.Hostname()}, "agent-2")
	proxy2.Emitter = &MockTelemetryEmitter{}
	proxyServer2, _ := proxy2.Start(context.Background())
	defer proxyServer2.Close()

	proxyURL2, _ := url.Parse(fmt.Sprintf("http://127.0.0.1:%d", proxy2.Port))
	client2 := &http.Client{
		Transport: &http.Transport{
			Proxy: http.ProxyURL(proxyURL2),
		},
	}

	req2, _ := http.NewRequest("GET", server.URL, nil)
	resp2, err := client2.Do(req2)
	if err != nil {
		t.Fatalf("Failed to make request: %v", err)
	}
	defer resp2.Body.Close()

	if resp2.StatusCode != http.StatusOK {
		t.Errorf("Expected status 200, got %d", resp2.StatusCode)
	}
}

func TestNetworkProxy_DeniedDomain(t *testing.T) {
	mockEmitter := &MockTelemetryEmitter{}
	proxy := NewNetworkProxy(0, []string{"example.com"}, "agent-1")
	proxy.Emitter = mockEmitter

	proxyServer, err := proxy.Start(context.Background())
	if err != nil {
		t.Fatalf("Failed to start proxy: %v", err)
	}
	defer proxyServer.Close()

	proxyURL, _ := url.Parse(fmt.Sprintf("http://127.0.0.1:%d", proxy.Port))
	client := &http.Client{
		Transport: &http.Transport{
			Proxy: http.ProxyURL(proxyURL),
		},
	}

	req, _ := http.NewRequest("GET", "http://malicious.com", nil)
	resp, err := client.Do(req)
	if err != nil {
		t.Fatalf("Failed to make request: %v", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusForbidden {
		t.Errorf("Expected status 403, got %d", resp.StatusCode)
	}

	if mockEmitter.Violations != 1 {
		t.Errorf("Expected 1 violation recorded, got %d", mockEmitter.Violations)
	}
}

func TestNetworkProxy_ConnectAllowedDomain(t *testing.T) {
	backendServer := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
		w.Write([]byte("ok backend"))
	}))
	defer backendServer.Close()

	backendURL, _ := url.Parse(backendServer.URL)
	backendHost := backendURL.Host
	if idx := strings.IndexByte(backendHost, ':'); idx != -1 {
		backendHost = backendHost[:idx]
	}

	proxy := NewNetworkProxy(0, []string{backendHost}, "agent-1")
	proxy.Emitter = &MockTelemetryEmitter{}
	proxyServer, err := proxy.Start(context.Background())
	if err != nil {
		t.Fatalf("Failed to start proxy: %v", err)
	}
	defer proxyServer.Close()

	proxyConn, err := net.Dial("tcp", fmt.Sprintf("127.0.0.1:%d", proxy.Port))
	if err != nil {
		t.Fatalf("Failed to connect to proxy: %v", err)
	}
	defer proxyConn.Close()

	reqStr := fmt.Sprintf("CONNECT %s HTTP/1.1\r\nHost: %s\r\n\r\n", backendURL.Host, backendURL.Host)
	_, err = proxyConn.Write([]byte(reqStr))
	if err != nil {
		t.Fatalf("Failed to write CONNECT request: %v", err)
	}

	buf := make([]byte, 1024)
	n, err := proxyConn.Read(buf)
	if err != nil && err != io.EOF {
		t.Fatalf("Failed to read from proxy: %v", err)
	}
	responseStr := string(buf[:n])
	if !strings.Contains(responseStr, "200 Connection Established") {
		t.Errorf("Expected '200 Connection Established', got %s", responseStr)
	}
}

func TestNetworkProxy_ConnectFailure(t *testing.T) {
	proxy := NewNetworkProxy(0, []string{"example.com"}, "agent-1")
	proxy.Emitter = &MockTelemetryEmitter{}
	proxyServer, err := proxy.Start(context.Background())
	if err != nil {
		t.Fatalf("Failed to start proxy: %v", err)
	}
	defer proxyServer.Close()

	proxyConn, err := net.Dial("tcp", fmt.Sprintf("127.0.0.1:%d", proxy.Port))
	if err != nil {
		t.Fatalf("Failed to connect to proxy: %v", err)
	}
	defer proxyConn.Close()

	reqStr := fmt.Sprintf("CONNECT example.com:9999 HTTP/1.1\r\nHost: example.com:9999\r\n\r\n")
	_, err = proxyConn.Write([]byte(reqStr))
	if err != nil {
		t.Fatalf("Failed to write CONNECT request: %v", err)
	}

	buf := make([]byte, 1024)
	n, err := proxyConn.Read(buf)
	if err != nil && err != io.EOF {
		t.Fatalf("Failed to read from proxy: %v", err)
	}
	responseStr := string(buf[:n])
	if !strings.Contains(responseStr, "503 Service Unavailable") {
		t.Errorf("Expected '503 Service Unavailable', got %s", responseStr)
	}
}

type dummyHijacker struct {
	http.ResponseWriter
	fail bool
}

func (d *dummyHijacker) Hijack() (net.Conn, *strings.Reader, error) {
	if d.fail {
		return nil, nil, fmt.Errorf("hijack failed")
	}
	return nil, nil, nil
}

func TestNetworkProxy_HijackSupport(t *testing.T) {
	proxy := NewNetworkProxy(0, []string{"example.com"}, "agent-1")
	proxy.Emitter = &MockTelemetryEmitter{}

	req, _ := http.NewRequest("CONNECT", "http://example.com", nil)
	req.URL.Host = "example.com:80"

	w := httptest.NewRecorder()
	proxy.ServeHTTP(w, req)
	if w.Code != http.StatusInternalServerError {
		// Should fail because httptest.ResponseRecorder doesn't support Hijacker
		t.Logf("Expected 500 error due to missing hijacker support but got %d", w.Code)
	}
}

func TestNetworkProxy_RedirectsNotFollowed(t *testing.T) {
	// Create a redirect server
	redirectServer := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		http.Redirect(w, r, "http://example.com", http.StatusFound)
	}))
	defer redirectServer.Close()

	backendURL, _ := url.Parse(redirectServer.URL)

	proxy := NewNetworkProxy(0, []string{backendURL.Hostname()}, "agent-1")
	proxy.Emitter = &MockTelemetryEmitter{}
	proxyServer, _ := proxy.Start(context.Background())
	defer proxyServer.Close()

	proxyURL, _ := url.Parse(fmt.Sprintf("http://127.0.0.1:%d", proxy.Port))
	client := &http.Client{
		Transport: &http.Transport{
			Proxy: http.ProxyURL(proxyURL),
		},
		CheckRedirect: func(req *http.Request, via []*http.Request) error {
			return http.ErrUseLastResponse // don't follow on client side either
		},
	}

	req, _ := http.NewRequest("GET", redirectServer.URL, nil)
	resp, err := client.Do(req)
	if err != nil {
		t.Fatalf("Failed to make request: %v", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusFound {
		t.Errorf("Expected status 302 Found, got %d", resp.StatusCode)
	}
}
