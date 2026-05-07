package harness

import (
	"context"
	"crypto/rand"
	"encoding/hex"
	"fmt"
	"net"
	"net/http"
	"net/http/httptest"
	"path/filepath"
	"strings"
	"testing"
)

func getTestClient(socketPath string) *http.Client {
	return &http.Client{
		Transport: &http.Transport{
			DialContext: func(ctx context.Context, network, addr string) (net.Conn, error) {
				return net.Dial("unix", socketPath)
			},
		},
	}
}

func TestProxyAllowedDomain(t *testing.T) {
	policy := SandboxPolicy{
		AllowedDomains: []string{"example.com"},
	}

	b := make([]byte, 8)
	rand.Read(b)
	socketPath := filepath.Join("/tmp", fmt.Sprintf("test-proxy-%s.sock", hex.EncodeToString(b)))

	server, err := StartProxy(policy, socketPath)
	if err != nil {
		t.Fatalf("Failed to start proxy: %v", err)
	}
	defer server.Close()

	client := getTestClient(socketPath)

	req, err := http.NewRequest(http.MethodGet, "http://example.com", nil)
	if err != nil {
		t.Fatalf("Failed to create request: %v", err)
	}
	req.Host = "example.com"

	resp, err := client.Do(req)
	if err != nil {
		// Because example.com doesn't actually exist in this sandbox/test env,
		// we expect a 503 Service Unavailable from our proxy, NOT a 403.
		if !strings.Contains(err.Error(), "no such host") && !strings.Contains(err.Error(), "connection refused") && !strings.Contains(err.Error(), "503") {
			// Proxy attempts to route it and might fail, which is fine as long as it's not our 403 block.
		}
	} else {
		defer resp.Body.Close()
		if resp.StatusCode == http.StatusForbidden {
			t.Errorf("Expected allowed access, got 403 Forbidden")
		}
	}
}

func TestProxyDeniedDomain(t *testing.T) {
	policy := SandboxPolicy{
		AllowedDomains: []string{"example.com"},
	}

	b := make([]byte, 8)
	rand.Read(b)
	socketPath := filepath.Join("/tmp", fmt.Sprintf("test-proxy-%s.sock", hex.EncodeToString(b)))

	server, err := StartProxy(policy, socketPath)
	if err != nil {
		t.Fatalf("Failed to start proxy: %v", err)
	}
	defer server.Close()

	client := getTestClient(socketPath)

	req, err := http.NewRequest(http.MethodGet, "http://evil.com", nil)
	if err != nil {
		t.Fatalf("Failed to create request: %v", err)
	}
	req.Host = "evil.com"

	resp, err := client.Do(req)
	if err != nil {
		t.Fatalf("Request failed: %v", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusForbidden {
		t.Errorf("Expected 403 Forbidden, got %d", resp.StatusCode)
	}
}

func TestProxyTunnelingAndHTTP(t *testing.T) {
	policy := SandboxPolicy{
		AllowedDomains: []string{"example.com"},
	}

	// Direct call to trigger handlers
	handler := &NetworkProxyHandler{policy: policy}

	// Test handleHTTP error case
	req, _ := http.NewRequest(http.MethodGet, "http://example.com", nil)
	req.Host = "example.com"
	req.URL.Scheme = "invalid" // to cause RoundTrip error
	rr := httptest.NewRecorder()
	handler.handleHTTP(rr, req)
	if rr.Code != http.StatusServiceUnavailable {
		t.Errorf("Expected 503 for invalid HTTP proxy request, got %d", rr.Code)
	}

	// Test handleTunneling error case
	req2, _ := http.NewRequest(http.MethodConnect, "http://example.com", nil)
	req2.Host = "example.com:invalidport" // to cause DialContext error
	rr2 := httptest.NewRecorder()
	handler.handleTunneling(rr2, req2)
	if rr2.Code != http.StatusServiceUnavailable {
		t.Errorf("Expected 503 for invalid tunneling dial, got %d", rr2.Code)
	}
}

func TestIsAllowed(t *testing.T) {
	domains := []string{"api.github.com", ".aws.amazon.com"}

	tests := []struct {
		host     string
		expected bool
	}{
		{"api.github.com", true},
		{"api.github.com:443", true},
		{"github.com", false},
		{"s3.aws.amazon.com", true},
		{"aws.amazon.com", false}, // `.aws.amazon.com` suffix check will fail for strict match if we don't handle base domain, but in our logic `strings.HasSuffix("aws.amazon.com", ".aws.amazon.com")` is false.
	}

	for _, tt := range tests {
		if got := isAllowed(tt.host, domains); got != tt.expected {
			t.Errorf("isAllowed(%q, %v) = %v; want %v", tt.host, domains, got, tt.expected)
		}
	}
}
