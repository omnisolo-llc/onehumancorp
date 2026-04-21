package network

import (
	"context"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/backend/harness"
)

// DummySandboxAdapter implements orchestration.SandboxAdapter for tests.
type DummySandboxAdapter struct{}

func (d *DummySandboxAdapter) EmitViolation(ctx context.Context, violationType string, agentID string, path string) {}

func TestNetworkBridgeProxy(t *testing.T) {
	// Start a dummy target server.
	targetServer := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Write([]byte("Target OK"))
	}))
	defer targetServer.Close()

	// Parse target server port.
	targetURL := targetServer.URL

	// Find available port for Proxy
	proxyPort := 8089

	// Start Proxy Server (simulating host side).
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	go func() {
		harness.StartProxy(ctx, "127.0.0.1:8089", []string{"localhost", "127.0.0.1"}, &DummySandboxAdapter{}, "agent-1")
	}()
	time.Sleep(100 * time.Millisecond)

	// Initialize Bridge
	bridge, err := NewSocatBridge(proxyPort)
	if err != nil {
		t.Fatalf("Failed to create bridge: %v", err)
	}
	err = bridge.Start(ctx)
	if err != nil {
		t.Fatalf("Failed to start bridge: %v", err)
	}
	defer bridge.Stop()
	time.Sleep(100 * time.Millisecond)

	// Initialize BwrapExecutor with proxy.
	executor := harness.NewBwrapExecutor(bridge.HostSocketPath)

	// Try allowed domain request inside sandbox.
	out, err := executor.Execute(context.Background(), "curl", "-s", targetURL)
	if err != nil && !strings.Contains(err.Error(), "executable file not found") { // Ignore if bwrap/curl missing
		t.Errorf("Execute returned error (allowed domain): %v", err)
	}
	if err == nil && !strings.Contains(string(out), "Target OK") {
		t.Errorf("Unexpected output (allowed domain): %s", string(out))
	}

	// Try denied domain request inside sandbox.
	out, err = executor.Execute(context.Background(), "curl", "-s", "http://google.com")
	if err != nil && !strings.Contains(err.Error(), "executable file not found") {
		t.Errorf("Execute returned error (denied domain): %v", err)
	}
	if err == nil && !strings.Contains(string(out), "Forbidden") && !strings.Contains(string(out), "Could not resolve host") {
		t.Errorf("Unexpected output (denied domain): %s", string(out))
	}
}
