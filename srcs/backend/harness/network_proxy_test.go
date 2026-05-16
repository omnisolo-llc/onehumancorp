package harness

import (
	"context"
	"net/http"
	"net/http/httptest"
	"os"
	"testing"
)

func TestNetworkProxy_Allowed(t *testing.T) {
	os.Setenv("OHC_TELEMETRY_ENABLED", "true")
	os.Setenv("STANDALONE_MODE", "true")

	proxy := NewNetworkProxy([]string{"example.com"}, 8080)

	req := httptest.NewRequest("GET", "http://example.com/", nil)
	rr := httptest.NewRecorder()

	proxy.ServeHTTP(rr, req)

	if rr.Code == http.StatusForbidden {
		t.Errorf("expected not 403 Forbidden for allowed domain, got %v", rr.Code)
	}
}

func TestNetworkProxy_AllowedSubdomain(t *testing.T) {
	proxy := NewNetworkProxy([]string{"example.com"}, 8080)

	req := httptest.NewRequest("GET", "http://api.example.com/", nil)
	rr := httptest.NewRecorder()

	proxy.ServeHTTP(rr, req)

	if rr.Code == http.StatusForbidden {
		t.Errorf("expected not 403 Forbidden for allowed subdomain, got %v", rr.Code)
	}
}

func TestNetworkProxy_DeniedSuffixBypass(t *testing.T) {
	proxy := NewNetworkProxy([]string{"example.com"}, 8080)

	req := httptest.NewRequest("GET", "http://badexample.com/", nil)
	rr := httptest.NewRecorder()

	proxy.ServeHTTP(rr, req)

	if rr.Code != http.StatusForbidden {
		t.Errorf("expected 403 Forbidden for bypass domain, got %v", rr.Code)
	}
}

func TestNetworkProxy_Denied(t *testing.T) {
	proxy := NewNetworkProxy([]string{"example.com"}, 8080)

	req := httptest.NewRequest("GET", "http://malicious.com/", nil)
	rr := httptest.NewRecorder()

	proxy.ServeHTTP(rr, req)

	if rr.Code != http.StatusForbidden {
		t.Errorf("expected 403 Forbidden for denied domain, got %v", rr.Code)
	}
}

func TestNetworkProxy_StartStop(t *testing.T) {
	proxy := NewNetworkProxy([]string{}, 0)
	err := proxy.Start()
	if err != nil {
		t.Fatalf("expected no error starting proxy, got %v", err)
	}

	err = proxy.Stop(context.Background())
	if err != nil {
		t.Fatalf("expected no error stopping proxy, got %v", err)
	}
}
