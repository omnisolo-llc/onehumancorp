package harness

import (
	"context"
	"net"
	"net/http"
	"net/http/httptest"
	"net/url"
	"testing"
)

type MockTelemetry struct {
	Violations []string
}

func (m *MockTelemetry) RecordViolation(ctx context.Context, violationType, details string) error {
	m.Violations = append(m.Violations, details)
	return nil
}

func TestNetworkProxy_Allowed(t *testing.T) {
	telemetry := &MockTelemetry{}
	proxy := NewNetworkProxy(8080, []string{"example.com"}, telemetry)

	req := httptest.NewRequest(http.MethodGet, "http://example.com", nil)
	req.Host = "example.com"
	rr := httptest.NewRecorder()

	proxy.ServeHTTP(rr, req)

	if rr.Code == http.StatusForbidden {
		t.Errorf("Expected not forbidden, got %v", rr.Code)
	}
	if len(telemetry.Violations) > 0 {
		t.Errorf("Expected 0 violations, got %d", len(telemetry.Violations))
	}
}

func TestNetworkProxy_AllowedWithPort(t *testing.T) {
	telemetry := &MockTelemetry{}
	proxy := NewNetworkProxy(8080, []string{"example.com"}, telemetry)

	req := httptest.NewRequest(http.MethodGet, "http://example.com:8080", nil)
	req.Host = "example.com:8080"
	req.URL.Host = "example.com:8080"
	rr := httptest.NewRecorder()

	proxy.ServeHTTP(rr, req)

	if rr.Code == http.StatusForbidden {
		t.Errorf("Expected not forbidden, got %v", rr.Code)
	}
}

func TestNetworkProxy_Denied(t *testing.T) {
	telemetry := &MockTelemetry{}
	proxy := NewNetworkProxy(8080, []string{"example.com"}, telemetry)

	req := httptest.NewRequest(http.MethodGet, "http://evil.com", nil)
	req.Host = "evil.com"
	rr := httptest.NewRecorder()

	proxy.ServeHTTP(rr, req)

	if rr.Code != http.StatusForbidden {
		t.Errorf("Expected Forbidden, got %v", rr.Code)
	}
	if len(telemetry.Violations) != 1 {
		t.Errorf("Expected 1 violation, got %d", len(telemetry.Violations))
	}
}

func TestNetworkProxy_ConnectDenied(t *testing.T) {
	telemetry := &MockTelemetry{}
	proxy := NewNetworkProxy(8080, []string{"example.com"}, telemetry)

	req := httptest.NewRequest(http.MethodConnect, "//example.com:443", nil)
	req.Host = "evil.com:443"
	req.URL.Host = "evil.com:443"
	req.URL.Scheme = ""
	rr := httptest.NewRecorder()

	proxy.ServeHTTP(rr, req)

	if rr.Code != http.StatusForbidden {
		t.Errorf("Expected Forbidden, got %v", rr.Code)
	}
}

func TestNetworkProxy_ConnectAllowedWithMock(t *testing.T) {
	telemetry := &MockTelemetry{}
	proxy := NewNetworkProxy(8080, []string{"example.com"}, telemetry)

	req := httptest.NewRequest(http.MethodConnect, "//example.com:443", nil)
	req.Host = "example.com:443"
	req.URL.Host = "example.com:443"
	req.URL.Scheme = ""
	rr := httptest.NewRecorder()

	proxy.ServeHTTP(rr, req)

	if rr.Code == http.StatusForbidden {
		t.Errorf("Expected not forbidden, got %v", rr.Code)
	}
}

func TestNetworkProxy_EmptyDomain(t *testing.T) {
	telemetry := &MockTelemetry{}
	proxy := NewNetworkProxy(8080, []string{"example.com"}, telemetry)

	req := httptest.NewRequest(http.MethodGet, "http://", nil)
	req.Host = ""
	req.URL.Host = ""
	rr := httptest.NewRecorder()

	proxy.ServeHTTP(rr, req)

	if rr.Code != http.StatusForbidden {
		t.Errorf("Expected Forbidden for empty domain, got %v", rr.Code)
	}
}

func TestNetworkProxy_StartStop(t *testing.T) {
	proxy := NewNetworkProxy(0, []string{}, nil)

	err := proxy.Start()
	if err != nil {
		t.Errorf("Failed to start proxy: %v", err)
	}

	err = proxy.Stop(context.Background())
	if err != nil {
		t.Errorf("Failed to stop proxy: %v", err)
	}

	proxyNil := NewNetworkProxy(0, []string{}, nil)
	err = proxyNil.Stop(context.Background())
	if err != nil {
		t.Errorf("Failed to stop nil proxy server: %v", err)
	}
}

func TestNetworkProxy_SuccessForward(t *testing.T) {
	targetServer := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("X-Custom-Header", "test-value")
		w.WriteHeader(http.StatusOK)
		w.Write([]byte("Hello, World!"))
	}))
	defer targetServer.Close()

	u, _ := url.Parse(targetServer.URL)
	host, _, _ := net.SplitHostPort(u.Host)

	telemetry := &MockTelemetry{}
	proxy := NewNetworkProxy(8080, []string{host}, telemetry)

	req := httptest.NewRequest(http.MethodGet, targetServer.URL, nil)
	req.Host = u.Host
	rr := httptest.NewRecorder()

	proxy.ServeHTTP(rr, req)

	if rr.Code != http.StatusOK {
		t.Errorf("Expected OK, got %v", rr.Code)
	}
	if rr.Header().Get("X-Custom-Header") != "test-value" {
		t.Errorf("Expected copied header")
	}
	if rr.Body.String() != "Hello, World!" {
		t.Errorf("Expected Hello, World!, got %s", rr.Body.String())
	}
}
