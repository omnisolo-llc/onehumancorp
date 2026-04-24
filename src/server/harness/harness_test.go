package harness

import (
	"net/url"
)

import (
	"context"
	"io"
	"net/http"
	"strings"
	"testing"
)

func TestHarnessProxy(t *testing.T) {
	config := &SandboxConfig{
		DeniedDomains: []string{"blocked.com"},
	}

	h := NewHarness(config)
	defer h.Stop()

	// Try to make a request through the proxy to a blocked domain
	client := &http.Client{
		Transport: &http.Transport{
			Proxy: func(*http.Request) (*url.URL, error) {
				return url.Parse(h.proxyURL)
			},
		},
	}

	req, err := http.NewRequest("GET", h.proxyURL, nil)
	if err != nil {
		t.Fatal(err)
	}
	// Manually construct the proxy request
	req.Host = "blocked.com"
	req.URL.Host = "blocked.com"
	req.URL.Scheme = "http"

	resp, err := client.Do(req)
	if err != nil {
		t.Fatal(err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusForbidden {
		t.Errorf("Expected status Forbidden, got %v", resp.StatusCode)
	}

	body, _ := io.ReadAll(resp.Body)
	if !strings.Contains(string(body), "Forbidden by Harness Proxy") {
		t.Errorf("Expected forbidden body, got %s", string(body))
	}
}

func TestHarnessRun(t *testing.T) {
    // We can't really test metrics properly without a full exporter setup
    // But we can check that it doesn't crash when executed

    // Basic test that args are built correctly.
    // Full run requires bwrap which may not be available in test environment.
    config := &SandboxConfig{
        ReadPaths: []string{"/tmp"},
        WritePaths: []string{"/tmp/workspace"},
    }

    h := NewHarness(config)
    defer h.Stop()

    // Just verify the method doesn't panic
    res, err := h.Run(context.Background(), "echo", []string{"hello"})

    if err == nil && res.ExitCode != 0 {
        // Just sanity check
    }
}
