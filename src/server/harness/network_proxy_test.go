package harness

import (
	"context"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"
)

type MockTelemetry struct {
	Violations []string
}

func (m *MockTelemetry) RecordSandboxViolation(ctx context.Context, violationType string, details string) {
	m.Violations = append(m.Violations, details)
}

func TestNetworkProxyAllowed(t *testing.T) {
	telemetry := &MockTelemetry{}
	proxy := NewNetworkProxy([]string{"example.com"}, telemetry)

	req := httptest.NewRequest(http.MethodGet, "http://example.com/foo", nil)
	rec := httptest.NewRecorder()

	proxy.ServeHTTP(rec, req)

	// We expect a BadGateway because the proxy tries to actually reach example.com and might fail in a sandbox,
	// but it should NOT be Forbidden (403).
	if rec.Code == http.StatusForbidden {
		t.Fatalf("Expected request to be allowed, got 403 Forbidden")
	}

	if len(telemetry.Violations) != 0 {
		t.Fatalf("Expected 0 violations, got %d", len(telemetry.Violations))
	}
}

func TestNetworkProxyBlocked(t *testing.T) {
	telemetry := &MockTelemetry{}
	proxy := NewNetworkProxy([]string{"example.com"}, telemetry)

	req := httptest.NewRequest(http.MethodGet, "http://evil.com/foo", nil)
	rec := httptest.NewRecorder()

	proxy.ServeHTTP(rec, req)

	if rec.Code != http.StatusForbidden {
		t.Fatalf("Expected request to be blocked with 403, got %d", rec.Code)
	}

	if len(telemetry.Violations) != 1 {
		t.Fatalf("Expected 1 violation, got %d", len(telemetry.Violations))
	}

	if !strings.Contains(telemetry.Violations[0], "evil.com") {
		t.Fatalf("Expected violation details to contain 'evil.com', got %s", telemetry.Violations[0])
	}
}

func TestNetworkProxySubdomain(t *testing.T) {
	telemetry := &MockTelemetry{}
	proxy := NewNetworkProxy([]string{"example.com"}, telemetry)

	// Allowed subdomain
	req1 := httptest.NewRequest(http.MethodGet, "http://api.example.com/foo", nil)
	rec1 := httptest.NewRecorder()
	proxy.ServeHTTP(rec1, req1)
	if rec1.Code == http.StatusForbidden {
		t.Fatalf("Expected subdomain to be allowed, got 403")
	}

	// Blocked different domain ending in same string
	req2 := httptest.NewRequest(http.MethodGet, "http://notexample.com/foo", nil)
	rec2 := httptest.NewRecorder()
	proxy.ServeHTTP(rec2, req2)
	if rec2.Code != http.StatusForbidden {
		t.Fatalf("Expected similar domain to be blocked, got %d", rec2.Code)
	}
}
