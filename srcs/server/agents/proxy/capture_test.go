package proxy

import (
	"context"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/sdk/metric"
	"go.opentelemetry.io/otel/sdk/metric/metricdata"
)

func TestProxyCapture_Success(t *testing.T) {
	// Set up a test meter provider to capture metrics
	reader := metric.NewManualReader()
	provider := metric.NewMeterProvider(metric.WithReader(reader))
	otel.SetMeterProvider(provider)

	// Need to re-init the metrics with the new provider
	meter = provider.Meter("onehumancorp/mono/srcs/server/agents/proxy")
	var err error
	requestsCounter, err = meter.Int64Counter("ohc_agent_outbound_requests_total")
	require.NoError(t, err)
	latencyHisto, err = meter.Float64Histogram("ohc_agent_outbound_request_latency_seconds")
	require.NoError(t, err)

	// Set up backend server
	backend := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
		w.Write([]byte("OK"))
	}))
	defer backend.Close()

	// Set up proxy
	proxy := NewProxyCapture()

	// Create request directed to proxy but with backend URL
	req, err := http.NewRequest("GET", backend.URL, nil)
	require.NoError(t, err)

	recorder := httptest.NewRecorder()

	// Execute request through proxy
	proxy.ServeHTTP(recorder, req)

	// Verify response
	assert.Equal(t, http.StatusOK, recorder.Code)
	assert.Equal(t, "OK", recorder.Body.String())

	// Verify metrics
	var rm metricdata.ResourceMetrics
	err = reader.Collect(context.Background(), &rm)
	require.NoError(t, err)

	foundCounter := false
	for _, sm := range rm.ScopeMetrics {
		for _, m := range sm.Metrics {
			if m.Name == "ohc_agent_outbound_requests_total" {
				foundCounter = true
				data, ok := m.Data.(metricdata.Sum[int64])
				require.True(t, ok)
				require.NotEmpty(t, data.DataPoints)
				assert.Equal(t, int64(1), data.DataPoints[0].Value)
			}
		}
	}
	assert.True(t, foundCounter, "Counter metric not found")
}

func TestProxyCapture_Failure(t *testing.T) {
	// Set up a test meter provider to capture metrics
	reader := metric.NewManualReader()
	provider := metric.NewMeterProvider(metric.WithReader(reader))
	otel.SetMeterProvider(provider)

	// Need to re-init the metrics with the new provider
	meter = provider.Meter("onehumancorp/mono/srcs/server/agents/proxy")
	var err error
	requestsCounter, err = meter.Int64Counter("ohc_agent_outbound_requests_total")
	require.NoError(t, err)
	latencyHisto, err = meter.Float64Histogram("ohc_agent_outbound_request_latency_seconds")
	require.NoError(t, err)

	// Set up proxy
	proxy := NewProxyCapture()

	// Create request directed to non-existent backend
	req, err := http.NewRequest("GET", "http://127.0.0.1:0", nil) // Port 0 should fail
	require.NoError(t, err)

	recorder := httptest.NewRecorder()

	// Execute request through proxy
	proxy.ServeHTTP(recorder, req)

	// Verify response
	assert.Equal(t, http.StatusBadGateway, recorder.Code)

	// Verify metrics
	var rm metricdata.ResourceMetrics
	err = reader.Collect(context.Background(), &rm)
	require.NoError(t, err)

	foundCounter := false
	for _, sm := range rm.ScopeMetrics {
		for _, m := range sm.Metrics {
			if m.Name == "ohc_agent_outbound_requests_total" {
				foundCounter = true
				data, ok := m.Data.(metricdata.Sum[int64])
				require.True(t, ok)
				require.NotEmpty(t, data.DataPoints)
				assert.Equal(t, int64(1), data.DataPoints[0].Value)
			}
		}
	}
	assert.True(t, foundCounter, "Counter metric not found")
}
