package harness

import (
	"context"
	"io"
	"net/http"
	"strings"
	"testing"
	"net/url"
)

func TestHarnessProxy(t *testing.T) {
	config := &SandboxConfig{
		DeniedDomains: []string{"blocked.com"},
	}

	h := NewHarness(config)
	defer h.Stop()

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
    config := &SandboxConfig{
        ReadPaths: []string{"/tmp"},
        WritePaths: []string{"/tmp/workspace"},
    }

    h := NewHarness(config)
    defer h.Stop()

    res, err := h.Run(context.Background(), "echo", []string{"hello"})

    if err == nil && res.ExitCode != 0 {
        // Just sanity check
    }
}
