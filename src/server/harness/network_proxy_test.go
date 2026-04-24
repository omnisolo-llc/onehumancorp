package harness

import (
	"context"
	"io"
	"net/http"
	"net/http/httptest"
	"net/url"
	"strings"
	"testing"
)

func TestNetworkProxy_AllowedDomain(t *testing.T) {
	// Setup a mock target server
	targetServer := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
		w.Write([]byte("Target OK"))
	}))
	defer targetServer.Close()

	proxy := NewNetworkProxy("agent-123", []string{"127.0.0.1"})
	err := proxy.Start()
	if err != nil {
		t.Fatalf("Failed to start proxy: %v", err)
	}
	defer proxy.Close()

	proxyURL, _ := url.Parse(proxy.URL())
	client := &http.Client{
		Transport: &http.Transport{
			Proxy: http.ProxyURL(proxyURL),
		},
	}

	req, _ := http.NewRequestWithContext(context.Background(), "GET", targetServer.URL, nil)
	resp, err := client.Do(req)
	if err != nil {
		t.Fatalf("Request failed: %v", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		t.Errorf("Expected status 200 OK, got %v", resp.StatusCode)
	}

	body, _ := io.ReadAll(resp.Body)
	if string(body) != "Target OK" {
		t.Errorf("Expected 'Target OK', got %s", string(body))
	}
}

func TestNetworkProxy_DeniedDomain(t *testing.T) {
	proxy := NewNetworkProxy("agent-123", []string{"allowed.com"})
	err := proxy.Start()
	if err != nil {
		t.Fatalf("Failed to start proxy: %v", err)
	}
	defer proxy.Close()

	proxyURL, _ := url.Parse(proxy.URL())
	client := &http.Client{
		Transport: &http.Transport{
			Proxy: http.ProxyURL(proxyURL),
		},
	}

	// Make a request to a denied domain (which doesn't exist, but we mock the request URL anyway)
	req, _ := http.NewRequestWithContext(context.Background(), "GET", "http://forbidden.com", nil)
	resp, err := client.Do(req)
	if err != nil {
		t.Fatalf("Request failed: %v", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusForbidden {
		t.Errorf("Expected status 403 Forbidden, got %v", resp.StatusCode)
	}

	body, _ := io.ReadAll(resp.Body)
	if !strings.Contains(string(body), "Forbidden by Sandbox Proxy") {
		t.Errorf("Expected 'Forbidden by Sandbox Proxy', got %s", string(body))
	}
}

func TestNetworkProxy_Connect(t *testing.T) {
	proxy := NewNetworkProxy("agent-123", []string{}) // empty allowed domains => everything denied
	err := proxy.Start()
	if err != nil {
		t.Fatalf("Failed to start proxy: %v", err)
	}
	defer proxy.Close()

	proxyURL, _ := url.Parse(proxy.URL())
	client := &http.Client{
		Transport: &http.Transport{
			Proxy: http.ProxyURL(proxyURL),
		},
	}

	// Note: CONNECT request might be triggered automatically for HTTPS
	req, _ := http.NewRequestWithContext(context.Background(), "GET", "https://forbidden.com", nil)
	resp, err := client.Do(req)
	// Expecting failure because CONNECT gets rejected and the proxy might close connection or return 403
	if err == nil {
		t.Errorf("Expected request to fail due to forbidden proxy CONNECT, got success. Status: %d", resp.StatusCode)
		resp.Body.Close()
	}
}
