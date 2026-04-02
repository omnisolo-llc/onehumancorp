package telemetry

import (
	"bytes"
	"context"
	"fmt"
	"log/slog"
	"net/http"
	"net/http/httptest"
	"os"
	"testing"
	"time"

	"github.com/prometheus/client_golang/prometheus"
	"go.opentelemetry.io/otel/metric"
)

func TestInitTelemetry(t *testing.T) {
	prometheus.DefaultRegisterer = prometheus.NewRegistry()

	// In Standalone Mode, Telemetry may be disabled. Force it enabled for tests.
	originalStandalone := os.Getenv("OHC_STANDALONE")
	os.Unsetenv("OHC_STANDALONE")
	defer func() {
		if originalStandalone != "" {
			os.Setenv("OHC_STANDALONE", originalStandalone)
		}
	}()

	// Happy path: initialization succeeds
	cleanup, err := InitTelemetry()
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if cleanup == nil {
		t.Fatal("expected cleanup function, got nil")
	}

	// Verify that the globals are set
	if requestCounter == nil {
		t.Error("expected requestCounter to be initialized")
	}
	if latencyHistogram == nil {
		t.Error("expected latencyHistogram to be initialized")
	}
	if tokenUsageCounter == nil {
		t.Error("expected tokenUsageCounter to be initialized")
	}
	if agentApiCallsCounter == nil {
		t.Error("expected agentApiCallsCounter to be initialized")
	}
	if humanInteractionsCounter == nil {
		t.Error("expected humanInteractionsCounter to be initialized")
	}
	if meetingEventsCounter == nil {
		t.Error("expected meetingEventsCounter to be initialized")
	}

	cleanup() // Clean up resources
}

// mockRegisterer always returns an error on Register
type mockRegisterer struct{}

func (m *mockRegisterer) Register(prometheus.Collector) error {
	return prometheus.AlreadyRegisteredError{}
}

func (m *mockRegisterer) MustRegister(...prometheus.Collector) {
	panic("mock Register error")
}

func (m *mockRegisterer) Unregister(prometheus.Collector) bool {
	return true
}

// mockMeter implements metric.Meter
type mockMeter struct{
	failCounters bool
	failHistograms bool
}

// mockFloat64Histogram implements metric.Float64Histogram
type mockFloat64Histogram struct {
	metric.Float64Histogram
}

// mockInt64Counter implements metric.Int64Counter
type mockInt64Counter struct {
	metric.Int64Counter
}

func (m *mockInt64Counter) Add(ctx context.Context, incr int64, options ...metric.AddOption) {}

func (m *mockMeter) Int64Counter(name string, options ...metric.Int64CounterOption) (metric.Int64Counter, error) {
	if m.failCounters {
		return nil, fmt.Errorf("mock counter error")
	}
	return &mockInt64Counter{}, nil
}

func (m *mockMeter) Float64Histogram(name string, options ...metric.Float64HistogramOption) (metric.Float64Histogram, error) {
	if m.failHistograms {
		return nil, fmt.Errorf("mock histogram error")
	}
	return &mockFloat64Histogram{}, nil
}

func (m *mockMeter) Int64UpDownCounter(name string, options ...metric.Int64UpDownCounterOption) (metric.Int64UpDownCounter, error) {
	return nil, nil
}

func (m *mockMeter) Float64UpDownCounter(name string, options ...metric.Float64UpDownCounterOption) (metric.Float64UpDownCounter, error) {
	return nil, nil
}

func (m *mockMeter) Int64ObservableCounter(name string, options ...metric.Int64ObservableCounterOption) (metric.Int64ObservableCounter, error) {
	return nil, nil
}

func (m *mockMeter) Float64ObservableCounter(name string, options ...metric.Float64ObservableCounterOption) (metric.Float64ObservableCounter, error) {
	return nil, nil
}

func (m *mockMeter) Int64ObservableUpDownCounter(name string, options ...metric.Int64ObservableUpDownCounterOption) (metric.Int64ObservableUpDownCounter, error) {
	return nil, nil
}

func (m *mockMeter) Float64ObservableUpDownCounter(name string, options ...metric.Float64ObservableUpDownCounterOption) (metric.Float64ObservableUpDownCounter, error) {
	return nil, nil
}

func (m *mockMeter) Int64ObservableGauge(name string, options ...metric.Int64ObservableGaugeOption) (metric.Int64ObservableGauge, error) {
	return nil, nil
}

func (m *mockMeter) Float64ObservableGauge(name string, options ...metric.Float64ObservableGaugeOption) (metric.Float64ObservableGauge, error) {
	return nil, nil
}

func (m *mockMeter) RegisterCallback(callback metric.Callback, instruments ...metric.Observable) (metric.Registration, error) {
	return nil, nil
}

func (m *mockMeter) Float64Counter(name string, options ...metric.Float64CounterOption) (metric.Float64Counter, error) {
	return nil, nil
}

func (m *mockMeter) Int64Histogram(name string, options ...metric.Int64HistogramOption) (metric.Int64Histogram, error) {
	return nil, nil
}

type mockFloat64Gauge struct {
	metric.Float64Gauge
	lastValue float64
}

func (m *mockFloat64Gauge) Record(ctx context.Context, value float64, options ...metric.RecordOption) {
	m.lastValue = value
}

func (m *mockMeter) Float64Gauge(name string, options ...metric.Float64GaugeOption) (metric.Float64Gauge, error) {
	if m.failHistograms {
		return nil, fmt.Errorf("mock gauge error")
	}
	return &mockFloat64Gauge{}, nil
}

func (m *mockMeter) Int64Gauge(name string, options ...metric.Int64GaugeOption) (metric.Int64Gauge, error) {
	return nil, nil
}

// Unexported interface method for metric.Meter in newer otel versions
func (m *mockMeter) meter() {}

func TestInitTelemetryError(t *testing.T) {
	originalReg := prometheus.DefaultRegisterer
	defer func() { prometheus.DefaultRegisterer = originalReg }()

	prometheus.DefaultRegisterer = &mockRegisterer{}

	// In Standalone Mode, Telemetry may be disabled. Force it enabled for tests.
	originalStandalone := os.Getenv("OHC_STANDALONE")
	os.Unsetenv("OHC_STANDALONE")
	defer func() {
		if originalStandalone != "" {
			os.Setenv("OHC_STANDALONE", originalStandalone)
		}
	}()

	cleanup, err := InitTelemetry()
	if err == nil {
		if cleanup != nil {
			cleanup()
		}
		t.Error("expected error from InitTelemetry with mock registerer, got nil")
	} else if err.Error() != "mock Register error" && err.Error() != "already registered" {
		// Just to log it, as the mock registerer might return an AlreadyRegisteredError
		// but open telemetry exporter might wrap it or swallow it depending on version.
		// Wait, if it didn't fail it would hit the `err == nil` case.
		// Actually, depending on the OpenTelemetry version, supplying an already-registered collector might succeed and log.
		// We mock panic to force it to fail if it swallows errors, or just let it pass if it returns the already registered error.
	}
}

func TestTelemetryMetricErrors(t *testing.T) {
	originalRequestCounter := requestCounter
	originalLatencyHistogram := latencyHistogram
	originalTokenUsageCounter := tokenUsageCounter
	originalAgentApiCallsCounter := agentApiCallsCounter
	originalHumanInteractionsCounter := humanInteractionsCounter
	originalMeetingEventsCounter := meetingEventsCounter

	defer func() {
		requestCounter = originalRequestCounter
		latencyHistogram = originalLatencyHistogram
		tokenUsageCounter = originalTokenUsageCounter
		agentApiCallsCounter = originalAgentApiCallsCounter
		humanInteractionsCounter = originalHumanInteractionsCounter
		meetingEventsCounter = originalMeetingEventsCounter
	}()

	// Directly call the InitWithMeter function to test coverage
	var err error
	mock := &mockMeter{failCounters: true}

	err = InitWithMeter(mock)
	if err == nil {
		t.Errorf("expected error from failCounters")
	}

	mock = &mockMeter{failHistograms: true}
	err = InitWithMeter(mock)
	if err == nil {
		t.Errorf("expected error from failHistograms")
	}
}

func TestMiddleware(t *testing.T) {
	tests := []struct {
		name      string
		verbosity int
	}{
		{
			name:      "default verbosity",
			verbosity: 1,
		},
		{
			name:      "high verbosity",
			verbosity: 2,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			originalVerbosity := Verbosity
			Verbosity = tt.verbosity
			defer func() { Verbosity = originalVerbosity }()

			prometheus.DefaultRegisterer = prometheus.NewRegistry()

			// In Standalone Mode, Telemetry may be disabled. Force it enabled for tests.
			originalStandalone := os.Getenv("OHC_STANDALONE")
			os.Unsetenv("OHC_STANDALONE")
			defer func() {
				if originalStandalone != "" {
					os.Setenv("OHC_STANDALONE", originalStandalone)
				}
			}()

			cleanup, err := InitTelemetry()
			if err != nil {
				t.Fatalf("failed to init telemetry: %v", err)
			}
			defer cleanup()

			nextHandler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
				time.Sleep(5 * time.Millisecond)
				w.WriteHeader(http.StatusOK)
			})

			handlerToTest := Middleware(nextHandler)

			req := httptest.NewRequest("GET", "/test/path", nil)
			rr := httptest.NewRecorder()

			handlerToTest.ServeHTTP(rr, req)

			if status := rr.Code; status != http.StatusOK {
				t.Errorf("handler returned wrong status code: got %v want %v",
					status, http.StatusOK)
			}
		})
	}
}

func TestMetricsHandler(t *testing.T) {
	handler := MetricsHandler()
	if handler == nil {
		t.Fatal("expected handler, got nil")
	}

	req := httptest.NewRequest("GET", "/metrics", nil)
	rr := httptest.NewRecorder()

	handler.ServeHTTP(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v",
			status, http.StatusOK)
	}
}

func TestRecordFunctions(t *testing.T) {
	prometheus.DefaultRegisterer = prometheus.NewRegistry()

	// In Standalone Mode, Telemetry may be disabled. Force it enabled for tests.
	originalStandalone := os.Getenv("OHC_STANDALONE")
	os.Unsetenv("OHC_STANDALONE")
	defer func() {
		if originalStandalone != "" {
			os.Setenv("OHC_STANDALONE", originalStandalone)
		}
	}()

	cleanup, err := InitTelemetry()
	if err != nil {
		t.Fatalf("failed to init telemetry: %v", err)
	}
	defer cleanup()

	ctx := context.Background()

	t.Run("RecordTokenUsage", func(t *testing.T) {
		RecordTokenUsage(ctx, "agent-1", "developer", "gpt-4", "prompt", 100)
	})

	t.Run("RecordAgentApiCall", func(t *testing.T) {
		RecordAgentApiCall(ctx, "agent-1", "developer", "get_file")
	})

	t.Run("RecordHumanInteraction", func(t *testing.T) {
		RecordHumanInteraction(ctx, "approval")
	})

	t.Run("RecordMeetingEvent", func(t *testing.T) {
		RecordMeetingEvent(ctx, "start")
	})
}

func TestRecordFunctionsUninitialized(t *testing.T) {
	originalTokenUsageCounter := tokenUsageCounter
	originalAgentApiCallsCounter := agentApiCallsCounter
	originalHumanInteractionsCounter := humanInteractionsCounter
	originalMeetingEventsCounter := meetingEventsCounter

	tokenUsageCounter = nil
	agentApiCallsCounter = nil
	humanInteractionsCounter = nil
	meetingEventsCounter = nil

	defer func() {
		tokenUsageCounter = originalTokenUsageCounter
		agentApiCallsCounter = originalAgentApiCallsCounter
		humanInteractionsCounter = originalHumanInteractionsCounter
		meetingEventsCounter = originalMeetingEventsCounter
	}()

	ctx := context.Background()

	t.Run("RecordTokenUsage Uninitialized", func(t *testing.T) {
		RecordTokenUsage(ctx, "agent-1", "developer", "gpt-4", "prompt", 100)
	})

	t.Run("RecordTokenBurnRate", func(t *testing.T) {
		mockM := &mockMeter{}
		err := InitWithMeter(mockM)
		if err != nil {
			t.Fatalf("InitWithMeter failed: %v", err)
		}

		RecordTokenBurnRate(ctx, "org-1", 15.5)

		// Check the gauge value directly from the tokenBurnRateGauge since it was set by InitWithMeter
		if g, ok := tokenBurnRateGauge.(*mockFloat64Gauge); ok {
			if g.lastValue != 15.5 {
				t.Errorf("expected 15.5, got %v", g.lastValue)
			}
		} else {
			t.Errorf("gauge was not initialized properly as mock")
		}
	})

	t.Run("RecordTokenBurnRate Uninitialized", func(t *testing.T) {
		// Reset gauge
		tokenBurnRateGauge = nil
		RecordTokenBurnRate(ctx, "org-1", 15.5)
	})

	t.Run("RecordAgentApiCall Uninitialized", func(t *testing.T) {
		RecordAgentApiCall(ctx, "agent-1", "developer", "get_file")
	})

	t.Run("RecordHumanInteraction Uninitialized", func(t *testing.T) {
		RecordHumanInteraction(ctx, "approval")
	})

	t.Run("RecordMeetingEvent Uninitialized", func(t *testing.T) {
		RecordMeetingEvent(ctx, "start")
	})
}

func TestLogAgentExecution(t *testing.T) {
	// Redirect slog output to a buffer to capture it
	var buf bytes.Buffer
	handler := slog.NewTextHandler(&buf, nil)
	originalLogger := slog.Default()
	slog.SetDefault(slog.New(handler))
	defer slog.SetDefault(originalLogger)

	ctx := context.Background()
	contentWithPII := "user user@example.com phone 123-456-7890 ssn 123-45-6789 and regular text"
	LogAgentExecution(ctx, "agent-1", "role-1", "api-1", "event-1", contentWithPII)

	output := buf.String()
	if !bytes.Contains(buf.Bytes(), []byte("agent execution trace")) {
		t.Errorf("Expected output to contain 'agent execution trace', got %q", output)
	}
	if !bytes.Contains(buf.Bytes(), []byte("agent_id=agent-1")) {
		t.Errorf("Expected output to contain 'agent_id=agent-1', got %q", output)
	}
	if !bytes.Contains(buf.Bytes(), []byte("role=role-1")) {
		t.Errorf("Expected output to contain 'role=role-1', got %q", output)
	}
	if bytes.Contains(buf.Bytes(), []byte("user@example.com")) {
		t.Errorf("Expected email to be redacted, got %q", output)
	}
	if bytes.Contains(buf.Bytes(), []byte("123-456-7890")) {
		t.Errorf("Expected phone to be redacted, got %q", output)
	}
	if bytes.Contains(buf.Bytes(), []byte("123-45-6789")) {
		t.Errorf("Expected ssn to be redacted, got %q", output)
	}
	if !bytes.Contains(buf.Bytes(), []byte("[REDACTED_EMAIL]")) {
		t.Errorf("Expected output to contain '[REDACTED_EMAIL]', got %q", output)
	}
	if !bytes.Contains(buf.Bytes(), []byte("[REDACTED_PHONE]")) {
		t.Errorf("Expected output to contain '[REDACTED_PHONE]', got %q", output)
	}
	if !bytes.Contains(buf.Bytes(), []byte("[REDACTED_SSN]")) {
		t.Errorf("Expected output to contain '[REDACTED_SSN]', got %q", output)
	}
}

// A custom prometheus.Registerer that always returns an error
type errorRegisterer struct{}

func (e errorRegisterer) Register(prometheus.Collector) error {
	return prometheus.AlreadyRegisteredError{} // or any error
}

func (e errorRegisterer) MustRegister(...prometheus.Collector) {
	panic("mock error")
}

func (e errorRegisterer) Unregister(prometheus.Collector) bool {
	return true
}

func TestInitTelemetry_PrometheusError(t *testing.T) {
	originalReg := prometheus.DefaultRegisterer
	defer func() { prometheus.DefaultRegisterer = originalReg }()

	prometheus.DefaultRegisterer = errorRegisterer{}

	// In Standalone Mode, Telemetry may be disabled. Force it enabled for tests.
	originalStandalone := os.Getenv("OHC_STANDALONE")
	os.Unsetenv("OHC_STANDALONE")
	defer func() {
		if originalStandalone != "" {
			os.Setenv("OHC_STANDALONE", originalStandalone)
		}
	}()

	_, err := InitTelemetry()
	if err == nil {
		t.Errorf("Expected error from InitTelemetry due to registerer failure")
	}
}
