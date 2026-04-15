package telemetry

import (
	"bytes"
	"context"
	"fmt"
	"log/slog"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	"github.com/prometheus/client_golang/prometheus"
	"go.opentelemetry.io/otel/metric"
)

func TestInitTelemetry(t *testing.T) {
	prometheus.DefaultRegisterer = prometheus.NewRegistry()
	t.Setenv("OHC_MULTITENANT", "true")

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
	if swarmTasksCompletedCounter == nil {
		t.Error("expected swarmTasksCompletedCounter to be initialized")
	}
	if SIPSyncLatencyRecorder == nil {
		t.Error("expected SIPSyncLatencyRecorder to be initialized")
	}
	if SIPSyncPayloadSizeRecorder == nil {
		t.Error("expected SIPSyncPayloadSizeRecorder to be initialized")
	}
	if AgentTransitionLatency == nil {
		t.Error("expected AgentTransitionLatency to be initialized")
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
type mockMeter struct {
	failCounters   bool
	failHistograms bool
}

// mockFloat64Histogram implements metric.Float64Histogram
type mockFloat64Histogram struct {
	metric.Float64Histogram
}

func (m *mockFloat64Histogram) Record(ctx context.Context, value float64, options ...metric.RecordOption) {
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

func (m *mockMeter) Int64UpDownCounter(name string, options ...metric.Int64UpDownCounterOption) (metric.Int64UpDownCounter, error) {
	if m.failCounters {
		return nil, fmt.Errorf("mock error")
	}
	return &mockInt64UpDownCounter{}, nil
}

func (m *mockMeter) Float64Histogram(name string, options ...metric.Float64HistogramOption) (metric.Float64Histogram, error) {
	if m.failHistograms {
		return nil, fmt.Errorf("mock histogram error")
	}
	return &mockFloat64Histogram{}, nil
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

type mockInt64UpDownCounter struct {
	metric.Int64UpDownCounter
}

func (m *mockInt64UpDownCounter) Add(ctx context.Context, incr int64, options ...metric.AddOption) {}

func (m *mockInt64UpDownCounter) Enabled(ctx context.Context) bool {
	return true
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
	t.Setenv("OHC_MULTITENANT", "true")

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
	originalSwarmTasksCompletedCounter := swarmTasksCompletedCounter

	defer func() {
		requestCounter = originalRequestCounter
		latencyHistogram = originalLatencyHistogram
		tokenUsageCounter = originalTokenUsageCounter
		agentApiCallsCounter = originalAgentApiCallsCounter
		humanInteractionsCounter = originalHumanInteractionsCounter
		meetingEventsCounter = originalMeetingEventsCounter
		swarmTasksCompletedCounter = originalSwarmTasksCompletedCounter
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
			t.Setenv("OHC_MULTITENANT", "true")

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

func TestRecordSQLiteLockMetrics(t *testing.T) {
	prometheus.DefaultRegisterer = prometheus.NewRegistry()
	InitTelemetry()

	// Just verifying these don't panic
	ctx := context.Background()
	RecordSQLiteLockContention(ctx, "test_op")
	RecordSQLiteRetryExhausted(ctx, "test_op")
}

func TestRecordFunctions(t *testing.T) {
	prometheus.DefaultRegisterer = prometheus.NewRegistry()
	t.Setenv("OHC_MULTITENANT", "true")

	cleanup, err := InitTelemetry()
	if err != nil {
		t.Fatalf("failed to init telemetry: %v", err)
	}
	defer cleanup()

	ctx := context.Background()

	t.Run("RecordTokenUsage", func(t *testing.T) {
		RecordTokenUsage(ctx, "agent-1", "developer", "gpt-4", "prompt", 100)
	})
	t.Run("RecordTokensSaved", func(t *testing.T) {
		RecordTokensSaved(ctx, "reason", "db", 100)
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

	t.Run("RecordTokenBurnRate", func(t *testing.T) {
		RecordTokenBurnRate(ctx, "acme-org", 123.45)
	})

	t.Run("RecordSwarmTaskCompleted", func(t *testing.T) {
		RecordSwarmTaskCompleted(ctx, "mission-123")
	})

	t.Run("RecordAgentTransitionLatency", func(t *testing.T) {
		RecordAgentTransitionLatency(ctx, "pending_to_running", 1.23)
	})

	t.Run("RecordToolAutoCorrection", func(t *testing.T) {
		RecordToolAutoCorrection(ctx, "agent-1", "developer", true)
	})

	t.Run("RecordDeliberationPhaseDuration", func(t *testing.T) {
		RecordDeliberationPhaseDuration(ctx, "plan-1", "PROPOSE", 1.23)
	})

	t.Run("RecordPostgresLockContention", func(t *testing.T) {
		RecordPostgresLockContention(ctx, "test_operation")
	})

	t.Run("RecordPostgresRetryExhausted", func(t *testing.T) {
		RecordPostgresRetryExhausted(ctx, "test_operation")
	})

	t.Run("RecordLLMNetworkLatency", func(t *testing.T) {
		RecordLLMNetworkLatency(ctx, "claude-3-5-sonnet", 1.23)
	})
}

func TestRecordFunctionsUninitialized(t *testing.T) {
	originalTokenUsageCounter := tokenUsageCounter
	originalAgentApiCallsCounter := agentApiCallsCounter
	originalHumanInteractionsCounter := humanInteractionsCounter
	originalMeetingEventsCounter := meetingEventsCounter
	originalTokenBurnRateGauge := tokenBurnRateGauge

	tokenUsageCounter = nil
	agentApiCallsCounter = nil
	humanInteractionsCounter = nil
	meetingEventsCounter = nil
	tokenBurnRateGauge = nil

	defer func() {
		tokenUsageCounter = originalTokenUsageCounter
		agentApiCallsCounter = originalAgentApiCallsCounter
		humanInteractionsCounter = originalHumanInteractionsCounter
		meetingEventsCounter = originalMeetingEventsCounter
		tokenBurnRateGauge = originalTokenBurnRateGauge
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

	t.Run("RecordTokenBurnRate Uninitialized", func(t *testing.T) {
		RecordTokenBurnRate(ctx, "acme-org", 123.45)
	})

	t.Run("RecordToolAutoCorrection Uninitialized", func(t *testing.T) {
		ToolAutoCorrectionTotal = nil
		RecordToolAutoCorrection(ctx, "agent-1", "developer", true)
	})

	t.Run("RecordDeliberationPhaseDuration Uninitialized", func(t *testing.T) {
		DeliberationPhaseDuration = nil
		RecordDeliberationPhaseDuration(ctx, "plan-1", "PROPOSE", 1.23)
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

func TestRedactInterfacePII(t *testing.T) {
	t.Run("String", func(t *testing.T) {
		res := RedactInterfacePII("email is test@example.com")
		if res != "email is [REDACTED_EMAIL]" {
			t.Errorf("Expected 'email is [REDACTED_EMAIL]', got %v", res)
		}
	})

	t.Run("Map", func(t *testing.T) {
		m := map[string]interface{}{
			"user":  "user@example.com",
			"other": "safe text",
		}
		res := RedactInterfacePII(m).(map[string]interface{})
		if res["user"] != "[REDACTED_EMAIL]" {
			t.Errorf("Expected [REDACTED_EMAIL], got %v", res["user"])
		}
		if res["other"] != "safe text" {
			t.Errorf("Expected 'safe text', got %v", res["other"])
		}
		// ensure original map is not mutated
		if m["user"] != "user@example.com" {
			t.Errorf("Expected original map to be unchanged, got %v", m["user"])
		}
	})

	t.Run("Slice of interface", func(t *testing.T) {
		s := []interface{}{"user@example.com", "safe text"}
		res := RedactInterfacePII(s).([]interface{})
		if res[0] != "[REDACTED_EMAIL]" {
			t.Errorf("Expected [REDACTED_EMAIL], got %v", res[0])
		}
		if s[0] != "user@example.com" {
			t.Errorf("Expected original slice to be unchanged, got %v", s[0])
		}
	})

	t.Run("Slice of string", func(t *testing.T) {
		s := []string{"user@example.com", "safe text"}
		res := RedactInterfacePII(s).([]string)
		if res[0] != "[REDACTED_EMAIL]" {
			t.Errorf("Expected [REDACTED_EMAIL], got %v", res[0])
		}
	})

	t.Run("Slice of map", func(t *testing.T) {
		s := []map[string]interface{}{
			{"email": "user@example.com"},
		}
		res := RedactInterfacePII(s).([]map[string]interface{})
		if res[0]["email"] != "[REDACTED_EMAIL]" {
			t.Errorf("Expected [REDACTED_EMAIL], got %v", res[0]["email"])
		}
		if s[0]["email"] != "user@example.com" {
			t.Errorf("Expected original map inside slice to be unchanged, got %v", s[0]["email"])
		}
	})

	t.Run("Default fallback", func(t *testing.T) {
		res := RedactInterfacePII(123)
		if res != 123 {
			t.Errorf("Expected 123, got %v", res)
		}
	})
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
	t.Setenv("OHC_MULTITENANT", "true")

	_, err := InitTelemetry()
	if err == nil {
		t.Errorf("Expected error from InitTelemetry due to registerer failure")
	}
}

func TestInitTelemetry_StandaloneOptOut(t *testing.T) {
	t.Setenv("OHC_MULTITENANT", "false")
	t.Setenv("OHC_TELEMETRY_ENABLED", "false")

	// Since we mock the actual metrics if initialized in other tests,
	// here we just ensure InitTelemetry returns quickly without error.
	cleanup, err := InitTelemetry()

	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if cleanup == nil {
		t.Fatal("expected dummy cleanup function, got nil")
	}
}

func TestInitTelemetry_StandaloneOptIn(t *testing.T) {
	t.Setenv("OHC_MULTITENANT", "false")
	t.Setenv("OHC_TELEMETRY_ENABLED", "true")

	originalReg := prometheus.DefaultRegisterer
	defer func() { prometheus.DefaultRegisterer = originalReg }()
	prometheus.DefaultRegisterer = prometheus.NewRegistry()

	cleanup, err := InitTelemetry()
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if cleanup == nil {
		t.Fatal("expected cleanup function, got nil")
	}
	cleanup()
}

func TestIdentityVerificationMetrics(t *testing.T) {
	IdentityVerificationSuccessTotal = nil
	IdentityVerificationFailureTotal = nil

	// Should not panic when nil
	RecordIdentityVerification(context.Background(), true)
	RecordIdentityVerification(context.Background(), false)
}

func TestSyncConflictResolvedMetric(t *testing.T) {
	SyncConflictsResolvedTotal = nil

	// Should not panic when nil
	RecordSyncConflictResolved(context.Background())
}

func TestOmniContextBytesRoutedMetric(t *testing.T) {
	OmniContextBytesRouted = nil

	// Should not panic when nil
	RecordOmniContextBytes(context.Background(), 1024)
}
