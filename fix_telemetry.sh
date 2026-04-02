cat << 'INNER_EOF' > srcs/server/telemetry/telemetry.go
package telemetry

import (
	"context"
	"encoding/json"
	"log/slog"
	"net/http"
	"os"
	"regexp"
	"time"

	"github.com/prometheus/client_golang/prometheus"
	"github.com/prometheus/client_golang/prometheus/promhttp"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/attribute"
	otelprom "go.opentelemetry.io/otel/exporters/prometheus"
	"go.opentelemetry.io/otel/metric"
	sdkmetric "go.opentelemetry.io/otel/sdk/metric"
)

var (
	meter                    metric.Meter
	requestCounter           metric.Int64Counter
	latencyHistogram         metric.Float64Histogram
	tokenUsageCounter        metric.Int64Counter
	tokenBurnRateGauge       metric.Float64Gauge
	agentApiCallsCounter     metric.Int64Counter
	humanInteractionsCounter metric.Int64Counter
	meetingEventsCounter     metric.Int64Counter
	swarmTasksCompletedCounter metric.Int64Counter
)

func InitTelemetry() (func(), error) {
	if os.Getenv("OHC_STANDALONE") == "true" && os.Getenv("OHC_TELEMETRY_ENABLED") != "true" {
		return func() {}, nil
	}
	exporter, err := otelprom.New(otelprom.WithRegisterer(prometheus.DefaultRegisterer))
	if err != nil {
		return nil, err
	}
	provider := sdkmetric.NewMeterProvider(sdkmetric.WithReader(exporter))
	otel.SetMeterProvider(provider)
	meter = provider.Meter("github.com/onehumancorp/mono/ohc")
	err = InitWithMeter(meter)
	if err != nil {
		return nil, err
	}
	return func() {
		_ = provider.Shutdown(context.Background())
	}, nil
}

type mockableMeter interface {
	Int64Counter(name string, options ...metric.Int64CounterOption) (metric.Int64Counter, error)
	Float64Histogram(name string, options ...metric.Float64HistogramOption) (metric.Float64Histogram, error)
	Float64Gauge(name string, options ...metric.Float64GaugeOption) (metric.Float64Gauge, error)
}

func InitWithMeter(m mockableMeter) error {
	var err error
	var errs []error

	swarmTasksCompletedCounter, err = m.Int64Counter(
		"ohc_swarm_tasks_completed",
		metric.WithDescription("Total swarm tasks completed by agents"),
	)
	if err != nil {
		errs = append(errs, err)
	}

	requestCounter, err = m.Int64Counter(
		"http_requests_total",
		metric.WithDescription("Total number of HTTP requests"),
	)
	if err != nil {
		errs = append(errs, err)
	}

	latencyHistogram, err = m.Float64Histogram(
		"http_request_duration_seconds",
		metric.WithDescription("HTTP request latency in seconds"),
	)
	if err != nil {
		errs = append(errs, err)
	}

	tokenUsageCounter, err = m.Int64Counter(
		"ohc_token_usage_total",
		metric.WithDescription("Total tokens used by agents"),
	)
	if err != nil {
		errs = append(errs, err)
	}

	tokenBurnRateGauge, err = m.Float64Gauge(
		"ohc_token_burn_rate_forecast",
		metric.WithDescription("Predicted moving average of token burn rate per minute per tenant"),
	)
	if err != nil {
		errs = append(errs, err)
	}

	agentApiCallsCounter, err = m.Int64Counter(
		"ohc_agent_api_calls_total",
		metric.WithDescription("Total API calls made by or for agents"),
	)
	if err != nil {
		errs = append(errs, err)
	}

	humanInteractionsCounter, err = m.Int64Counter(
		"ohc_human_interactions_total",
		metric.WithDescription("Total human-agent interactions"),
	)
	if err != nil {
		errs = append(errs, err)
	}

	meetingEventsCounter, err = m.Int64Counter(
		"ohc_meeting_events_total",
		metric.WithDescription("Total meeting room events"),
	)
	if err != nil {
		errs = append(errs, err)
	}

	if len(errs) > 0 {
		return errs[0]
	}

	return nil
}

func Middleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		start := time.Now()

		next.ServeHTTP(w, r)

		duration := time.Since(start).Seconds()

		if r.URL.Path != "/healthz" && r.URL.Path != "/readyz" {
			if requestCounter != nil && latencyHistogram != nil {
				attributes := metric.WithAttributes(
					attribute.String("method", r.Method),
					attribute.String("path", r.URL.Path),
				)
				requestCounter.Add(r.Context(), 1, attributes)
				latencyHistogram.Record(r.Context(), duration, attributes)
			}
			if Verbosity >= 2 {
				slog.Info("recorded request", "component", "telemetry", "method", r.Method, "path", r.URL.Path, "duration", duration)
			}
		}
	})
}

var Verbosity = 1 // Default level

func MetricsHandler() http.Handler {
	return promhttp.Handler()
}

func RecordTokenUsage(ctx context.Context, agentID, role, model, tokenType string, count int64) {
	if tokenUsageCounter == nil {
		return
	}
	tokenUsageCounter.Add(ctx, count, metric.WithAttributes(
		attribute.String("agent_id", agentID),
		attribute.String("role", role),
		attribute.String("model", model),
		attribute.String("type", tokenType),
	))

	if BufferMetricFunc != nil {
		payloadBytes, _ := json.Marshal(map[string]interface{}{
			"agent_id": agentID,
			"role":     role,
			"model":    model,
			"type":     tokenType,
			"count":    count,
		})
		_ = BufferMetricFunc(ctx, "token_usage", string(payloadBytes))
	}
}

func RecordAgentApiCall(ctx context.Context, agentID, role, api string) {
	if agentApiCallsCounter == nil {
		return
	}
	agentApiCallsCounter.Add(ctx, 1, metric.WithAttributes(
		attribute.String("agent_id", agentID),
		attribute.String("role", role),
		attribute.String("api", api),
	))

	if BufferMetricFunc != nil {
		payloadBytes, _ := json.Marshal(map[string]interface{}{
			"agent_id": agentID,
			"role":     role,
			"api":      api,
		})
		_ = BufferMetricFunc(ctx, "agent_api_call", string(payloadBytes))
	}
}

func RecordHumanInteraction(ctx context.Context, interactionType string) {
	if humanInteractionsCounter == nil {
		return
	}
	humanInteractionsCounter.Add(ctx, 1, metric.WithAttributes(
		attribute.String("type", interactionType),
	))

	if BufferMetricFunc != nil {
		payloadBytes, _ := json.Marshal(map[string]interface{}{
			"type": interactionType,
		})
		_ = BufferMetricFunc(ctx, "human_interaction", string(payloadBytes))
	}
}

func RecordMeetingEvent(ctx context.Context, eventType string) {
	if meetingEventsCounter == nil {
		return
	}
	meetingEventsCounter.Add(ctx, 1, metric.WithAttributes(
		attribute.String("type", eventType),
	))

	if BufferMetricFunc != nil {
		payloadBytes, _ := json.Marshal(map[string]interface{}{
			"type": eventType,
		})
		_ = BufferMetricFunc(ctx, "meeting_event", string(payloadBytes))
	}
}

// RecordSwarmTaskCompleted increments the global counter for completed swarm tasks.
func RecordSwarmTaskCompleted(ctx context.Context, agentID, taskID string) {
	if swarmTasksCompletedCounter == nil {
		return
	}
	swarmTasksCompletedCounter.Add(ctx, 1, metric.WithAttributes(
		attribute.String("agent_id", agentID),
		attribute.String("task_id", taskID),
	))

	if BufferMetricFunc != nil {
		payloadBytes, _ := json.Marshal(map[string]interface{}{
			"agent_id": agentID,
			"task_id":  taskID,
		})
		_ = BufferMetricFunc(ctx, "swarm_task_completed", string(payloadBytes))
	}
}

// Global buffer function pointer to inject dependency without circular imports.
var BufferMetricFunc func(ctx context.Context, metricType string, payload string) error

func RecordTokenBurnRate(ctx context.Context, organizationID string, rate float64) {
	if tokenBurnRateGauge != nil {
		tokenBurnRateGauge.Record(ctx, rate, metric.WithAttributes(
			attribute.String("organization_id", organizationID),
		))
	}
}

var (
	emailRegex   = regexp.MustCompile(`[a-zA-Z0-9._%+\-]+@[a-zA-Z0-9.\-]+\.[a-zA-Z]{2,}`)
	phoneRegex   = regexp.MustCompile(`\b\d{3}[-.]?\d{3}[-.]?\d{4}\b`)
	ssnRegex     = regexp.MustCompile(`\b\d{3}-\d{2}-\d{4}\b`)
)

// RedactPII removes sensitive info before logging.
func RedactPII(input string) string {
	s := emailRegex.ReplaceAllString(input, "[REDACTED_EMAIL]")
	s = phoneRegex.ReplaceAllString(s, "[REDACTED_PHONE]")
	s = ssnRegex.ReplaceAllString(s, "[REDACTED_SSN]")
	return s
}

func LogAgentExecution(ctx context.Context, agentID, role, api, eventType, content string) {
	slog.InfoContext(ctx, "agent execution trace",
		"component", "telemetry",
		"agent_id", agentID,
		"role", role,
		"api", api,
		"event_type", eventType,
		"content", RedactPII(content),
	)
}

INNER_EOF
