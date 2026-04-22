package network

import (
	"net/http"
	"net/http/httptest"
	"net/url"
	"strings"
	"testing"
)

func TestNetworkBridgeProxy_AllowedDomain(t *testing.T) {
	// Setup a local test server to act as our "allowed domain" destination
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
		w.Write([]byte("OK"))
	}))
	defer ts.Close()

	u, _ := url.Parse(ts.URL)
	host := u.Host // e.g. 127.0.0.1:45321
	hostname := u.Hostname()

	proxy := NewNetworkBridgeProxy("/tmp/test.sock", []string{hostname})

	req := httptest.NewRequest(http.MethodGet, ts.URL, nil)
	req.Host = host // set host explicitly to mock domain matching
	w := httptest.NewRecorder()

	proxy.ServeHTTP(w, req)

	if w.Code != http.StatusOK {
		t.Errorf("Expected allowed request to succeed with 200, got %d. Body: %s", w.Code, w.Body.String())
	}
	if w.Body.String() != "OK" {
		t.Errorf("Expected body 'OK', got %s", w.Body.String())
	}
}

func TestNetworkBridgeProxy_ForbiddenDomain(t *testing.T) {
	proxy := NewNetworkBridgeProxy("/tmp/test.sock", []string{"example.com"})

	req := httptest.NewRequest(http.MethodGet, "http://malicious.com/", nil)
	w := httptest.NewRecorder()

	proxy.ServeHTTP(w, req)

	if w.Code != http.StatusForbidden {
		t.Errorf("Expected forbidden request, got %d", w.Code)
	}
	body := w.Body.String()
	if !strings.Contains(body, "Forbidden") {
		t.Errorf("Expected body to contain Forbidden, got %s", body)
	}
}

func TestNetworkBridgeProxy_ConnectForbiddenDomain(t *testing.T) {
	proxy := NewNetworkBridgeProxy("/tmp/test.sock", []string{"example.com"})

	req := httptest.NewRequest(http.MethodConnect, "http://malicious.com:443/", nil)
	req.Host = "malicious.com:443"
	w := httptest.NewRecorder()

	proxy.ServeHTTP(w, req)

	if w.Code != http.StatusForbidden {
		t.Errorf("Expected forbidden connect request, got %d", w.Code)
	}
}
