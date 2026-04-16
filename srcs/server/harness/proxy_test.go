package harness

import (
	"net/http"
	"net/http/httptest"
	"net/url"
	"testing"
)

func TestProxy_ServeHTTP(t *testing.T) {
	// Create a mock target server
	targetServer := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
		w.Write([]byte("Target OK"))
	}))
	defer targetServer.Close()

	parsedURL, _ := url.Parse(targetServer.URL)

	config := &SandboxConfig{
		DeniedDomains: []string{"bad.com"},
	}
	proxy := NewProxy(config)

	// Test blocked domain
	reqBlocked, _ := http.NewRequest("GET", "http://bad.com/test", nil)
	rrBlocked := httptest.NewRecorder()
	proxy.ServeHTTP(rrBlocked, reqBlocked)

	if status := rrBlocked.Code; status != http.StatusForbidden {
		t.Errorf("handler returned wrong status code: got %v want %v",
			status, http.StatusForbidden)
	}

	// Test allowed domain using the target server URL
	reqAllowed, _ := http.NewRequest("GET", targetServer.URL, nil)

	// We ensure the URL Host/Scheme is correctly passed on a GET request directly
	// To match the HTTP Proxy host resolution logic properly
	reqAllowed.Host = parsedURL.Host
	reqAllowed.URL.Host = parsedURL.Host
	reqAllowed.URL.Scheme = parsedURL.Scheme

	rrAllowed := httptest.NewRecorder()
	proxy.ServeHTTP(rrAllowed, reqAllowed)

	if status := rrAllowed.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v",
			status, http.StatusOK)
	}
}
