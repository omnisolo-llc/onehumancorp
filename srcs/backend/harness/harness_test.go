package harness

import (
	"context"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/stretchr/testify/assert"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/sdk/metric"
	"go.opentelemetry.io/otel/sdk/metric/metricdata"
)

func TestHarnessMetricsAndPaths(t *testing.T) {
	reader := metric.NewManualReader()
	provider := metric.NewMeterProvider(metric.WithReader(reader))
	otel.SetMeterProvider(provider)

	config := &SandboxConfig{
		ReadPaths:  []string{"/bin"},
		WritePaths: []string{"/tmp"},
	}
	h := NewHarness(config)

	// Since we don't have bwrap in test environment, we expect an error
	_, err := h.Run("echo", []string{"hello"})
	assert.Error(t, err)

	rm := metricdata.ResourceMetrics{}
	err = reader.Collect(context.Background(), &rm)
	assert.NoError(t, err)

	found := false
	for _, sm := range rm.ScopeMetrics {
		for _, m := range sm.Metrics {
			if m.Name == "ohc_harness_executions_total" {
				found = true
				data := m.Data.(metricdata.Sum[int64])
				assert.Equal(t, int64(1), data.DataPoints[0].Value)
			}
		}
	}
	assert.True(t, found, "Expected ohc_harness_executions_total metric")
}

func TestProxy(t *testing.T) {
	// Setup a mock target server
	target := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
		w.Write([]byte("mock_target_ok"))
	}))
	defer target.Close()

	config := &SandboxConfig{
		DeniedDomains: []string{"bad.com"},
	}
	proxy := NewProxy(config)

	// Test blocked domain
	req := httptest.NewRequest("GET", "http://bad.com/", nil)
	rr := httptest.NewRecorder()
	proxy.ServeHTTP(rr, req)
	assert.Equal(t, http.StatusForbidden, rr.Code)

	// Test allowed domain (proxying to mock target)
	req2 := httptest.NewRequest("GET", target.URL, nil)
	// We need to set RequestURI manually since httptest.NewRequest sets it to the path
	req2.RequestURI = target.URL
	rr2 := httptest.NewRecorder()
	proxy.ServeHTTP(rr2, req2)
	assert.Equal(t, http.StatusOK, rr2.Code)
	assert.Equal(t, "mock_target_ok", rr2.Body.String())
}
