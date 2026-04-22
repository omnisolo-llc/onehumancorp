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

	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

func TestNetworkBridgeProxy_AllowedDomain(t *testing.T) {
	targetServer := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
		w.Write([]byte("Target Response"))
	}))
	defer targetServer.Close()

	targetURL, _ := url.Parse(targetServer.URL)
	targetHost := targetURL.Host
	if strings.Contains(targetHost, ":") {
		targetHost = strings.Split(targetHost, ":")[0]
	}

	proxy := NewNetworkBridgeProxy("agent-123", []string{targetHost})
	proxyURL, err := proxy.Start()
	if err != nil {
		t.Fatalf("Failed to start proxy: %v", err)
	}
	defer proxy.Stop()

	proxyUrlParsed, _ := url.Parse(proxyURL)
	client := &http.Client{
		Transport: &http.Transport{
			DialContext: func(ctx context.Context, network, addr string) (net.Conn, error) {
				return net.Dial("unix", proxyUrlParsed.Path)
			},
		},
	}

	req, _ := http.NewRequestWithContext(context.Background(), "GET", targetServer.URL, nil)
	resp, err := client.Do(req)
	if err != nil {
		t.Fatalf("Request failed: %v", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		t.Errorf("Expected status OK, got %v", resp.Status)
	}

	body, _ := io.ReadAll(resp.Body)
	if string(body) != "Target Response" {
		t.Errorf("Expected 'Target Response', got '%s'", string(body))
	}
}

func TestNetworkBridgeProxy_DeniedDomain(t *testing.T) {
	violationCount := 0
	originalFunc := telemetry.BufferMetricFunc
	telemetry.BufferMetricFunc = func(ctx context.Context, name string, payload string) error {
		if name == "sandbox_violation_total" {
			violationCount++
		}
		return nil
	}
	defer func() { telemetry.BufferMetricFunc = originalFunc }()

	proxy := NewNetworkBridgeProxy("agent-123", []string{"example.com"})
	proxyURL, err := proxy.Start()
	if err != nil {
		t.Fatalf("Failed to start proxy: %v", err)
	}
	defer proxy.Stop()

	proxyUrlParsed, _ := url.Parse(proxyURL)
	client := &http.Client{
		Transport: &http.Transport{
			DialContext: func(ctx context.Context, network, addr string) (net.Conn, error) {
				return net.Dial("unix", proxyUrlParsed.Path)
			},
		},
	}

	req, _ := http.NewRequestWithContext(context.Background(), "GET", "http://denied.com", nil)
	resp, err := client.Do(req)
	if err != nil {
		t.Fatalf("Request failed: %v", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusForbidden {
		t.Errorf("Expected status Forbidden, got %v", resp.Status)
	}

	if violationCount == 0 {
		t.Errorf("Expected telemetry violation to be recorded")
	}
}

func TestNetworkBridgeProxy_Connect(t *testing.T) {
	targetServer := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
		w.Write([]byte("Connect Target Response"))
	}))
	defer targetServer.Close()

	targetURL, _ := url.Parse(targetServer.URL)
	targetHost := targetURL.Host

	proxy := NewNetworkBridgeProxy("agent-123", []string{strings.Split(targetHost, ":")[0]})
	proxyURL, err := proxy.Start()
	if err != nil {
		t.Fatalf("Failed to start proxy: %v", err)
	}
	defer proxy.Stop()

	proxyUrlParsed, _ := url.Parse(proxyURL)
	client := &http.Client{
		Transport: &http.Transport{
			DialContext: func(ctx context.Context, network, addr string) (net.Conn, error) {
				return net.Dial("unix", proxyUrlParsed.Path)
			},
		},
	}

	req2, _ := http.NewRequestWithContext(context.Background(), http.MethodConnect, targetServer.URL, nil)
	resp2, err2 := client.Do(req2)
	if err2 != nil {
		t.Fatalf("CONNECT request failed: %v", err2)
	}
	defer resp2.Body.Close()

	if resp2.StatusCode != http.StatusOK {
		t.Errorf("Expected status OK, got %v", resp2.Status)
	}
}
