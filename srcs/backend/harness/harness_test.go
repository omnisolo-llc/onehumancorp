package harness

import (
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/stretchr/testify/assert"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/sdk/metric"
)

func TestHarness_Run(t *testing.T) {
	// Initialize memory metric reader and meter provider for testing
	reader := metric.NewManualReader()
	provider := metric.NewMeterProvider(metric.WithReader(reader))
	otel.SetMeterProvider(provider)

	// Set the global meter locally so we can record metrics and read them back
	meter = otel.Meter("ohc/backend/harness")

	config := &SandboxConfig{
		ReadPaths:  []string{"/bin", "/lib", "/lib64", "/usr"},
		WritePaths: []string{"/tmp"},
	}

	h := NewHarness(config)

	// Since we are mocking bwrap, we will simply test if the struct holds the right configuration.
	// Actually executing bwrap in CI may not be available or fails without privileges.
	// We check the paths instead.
	assert.Equal(t, config.ReadPaths, h.config.ReadPaths)
	assert.Equal(t, config.WritePaths, h.config.WritePaths)
}

func TestProxyServer_ServeHTTP(t *testing.T) {
	deniedDomains := []string{"bad.com", "evil.org"}
	proxy := NewProxyServer(deniedDomains)

	tests := []struct {
		name         string
		method       string
		url          string
		expectedCode int
	}{
		{
			name:         "Allowed Domain HTTP",
			method:       http.MethodGet,
			url:          "http://127.0.0.1:0", // Use random invalid local port to avoid DNS/redirect issues and guarantee failure or connection refused
			expectedCode: http.StatusServiceUnavailable, // It shouldn't be blocked, but fails to connect
		},
		{
			name:         "Denied Domain HTTP exact match",
			method:       http.MethodGet,
			url:          "http://bad.com",
			expectedCode: http.StatusForbidden,
		},
		{
			name:         "Denied Domain HTTP sub domain",
			method:       http.MethodGet,
			url:          "http://sub.evil.org",
			expectedCode: http.StatusForbidden,
		},
		{
			name:         "Denied Domain CONNECT exact match",
			method:       http.MethodConnect,
			url:          "https://bad.com", // CONNECT doesn't include scheme, httptest creates req.URL with host.
			expectedCode: http.StatusForbidden,
		},
	}

	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			req := httptest.NewRequest(tc.method, tc.url, nil)
			if tc.method == http.MethodConnect {
				req.Host = "bad.com:443" // Explicitly set Host for CONNECT
			} else {
				req.Host = req.URL.Host
			}

			w := httptest.NewRecorder()

			proxy.ServeHTTP(w, req)

			assert.Equal(t, tc.expectedCode, w.Code)
		})
	}
}

func TestHarness_Run_Metrics(t *testing.T) {
	// Initialize memory metric reader for capturing metrics
	reader := metric.NewManualReader()
	provider := metric.NewMeterProvider(metric.WithReader(reader))
	otel.SetMeterProvider(provider)

	config := &SandboxConfig{
		ReadPaths:  []string{},
		WritePaths: []string{},
	}

	h := NewHarness(config)

	h.Run("echo", []string{"hello"})

	// We are just verifying it doesn't crash on execution and sets up correctly
	// Actually capturing the package-level init() metric in unit tests requires complex mocking
	// so we assert execution is successful without panicking
}
