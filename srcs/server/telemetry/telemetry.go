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
	meter            metric.Meter
	requestCounter   metric.Int64Counter
	latencyHistogram metric.Float64Histogram

	tokenUsageCounter          metric.Int64Counter
	tokenBurnRateGauge         metric.Float64Gauge
	agentApiCallsCounter       metric.Int64Counter
	agentApiErrorsCounter      metric.Int64Counter
	humanInteractionsCounter   metric.Int64Counter
	meetingEventsCounter       metric.Int64Counter
	swarmTasksCompletedCounter metric.Int64Counter
	swarmTaskTransitionsCounter metric.Int64Counter
	taskEnqueuedCounter metric.Int64Counter
	taskFailedCounter metric.Int64Counter
	cacheHitsCounter           metric.Int64Counter
	cacheMissesCounter         metric.Int64Counter
	AutoDreamMemoriesIngestedCounter metric.Int64Counter
	AutoDreamMemoriesCompressedCounter metric.Int64Counter
	TeammateMeshBroadcastsCounter    metric.Int64Counter
	TeammateMeshDirectMessagesCounter metric.Int64Counter
	TaskQueueLengthGauge       metric.Int64UpDownCounter
	TaskProcessingLatency      metric.Float64Histogram

	SyncCompletedCount metric.Int64Counter
	SyncFailedCount    metric.Int64Counter
	SyncEscalationsCount metric.Int64Counter
	RateLimitExceededCount metric.Int64Counter
	SyncLatency        metric.Float64Histogram
	SyncPayloadSize    metric.Float64Histogram

	emailRegex = regexp.MustCompile(`[a-zA-Z0-9._%+\-]+@[a-zA-Z0-9.\-]+\.[a-zA-Z]{2,}`)
	phoneRegex = regexp.MustCompile(`\b\d{3}[-.]?\d{3}[-.]?\d{4}\b`)
	ssnRegex   = regexp.MustCompile(`\b\d{3}-\d{2}-\d{4}\b`)
)

func RedactPII(input string) string {
	s := emailRegex.ReplaceAllString(input, "[REDACTED_EMAIL]")
	s = phoneRegex.ReplaceAllString(s, "[REDACTED_PHONE]")
	s = ssnRegex.ReplaceAllString(s, "[REDACTED_SSN]")
	return s
}

// RedactInterfacePII deeply scrubs maps, slices, and strings for PII.
func RedactInterfacePII(val interface{}) interface{} {
	switch v := val.(type) {
	case string:
		return RedactPII(v)
	case map[string]interface{}:
		for k, val := range v {
			v[k] = RedactInterfacePII(val)
		}
		return v
	case []interface{}:
		for i, val := range v {
			v[i] = RedactInterfacePII(val)
		}
		return v
	case []string:
		res := make([]string, len(v))
		for i, str := range v {
			res[i] = RedactPII(str)
		}
		return res
	case []map[string]interface{}:
		for i, m := range v {
			for k, val := range m {
				m[k] = RedactInterfacePII(val)
			}
			v[i] = m
		}
		return v
	default:
		return val
	}
}

// InitTelemetry configures and starts the OpenTelemetry metrics provider with a Prometheus exporter.
//
// Accepts no parameters.
// Returns (func(), error).
// Produces errors: Explicit error handling.
// Has no side effects.
func InitTelemetry() (func(), error) {
	if os.Getenv("OHC_STANDALONE") == "true" && os.Getenv("OHC_TELEMETRY_ENABLED") != "true" {
		// Enforce user data privacy and local sovereignty in Standalone Mode.
		// Exporter is strictly opt-in and disabled by default.
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

// InitWithMeter initializes metrics using the provided meter
// We take any interface that implements the needed method to allow easy mocking
type mockableMeter interface {
	Int64Counter(name string, options ...metric.Int64CounterOption) (metric.Int64Counter, error)
	Int64UpDownCounter(name string, options ...metric.Int64UpDownCounterOption) (metric.Int64UpDownCounter, error)
	Float64Histogram(name string, options ...metric.Float64HistogramOption) (metric.Float64Histogram, error)
	Float64Gauge(name string, options ...metric.Float64GaugeOption) (metric.Float64Gauge, error)
}

// InitWithMeter functionality.
// Accepts parameters: m mockableMeter (No Constraints).
// Returns error.
// Produces errors: Explicit error handling.
// Has no side effects.
func InitWithMeter(m mockableMeter) error {
	var err error
	var errs []error
	requestCounter, err = m.Int64Counter(
		"http_requests_total",
		metric.WithDescription("Total number of HTTP requests"),
	)
	if err != nil {
		errs = append(errs, err)
	}

	SyncLatency, err = m.Float64Histogram(
		"ohc.sync.latency",
		metric.WithDescription("Latency of the sync process"),
	)
	if err != nil {
		errs = append(errs, err)
	}

	SyncPayloadSize, err = m.Float64Histogram(
		"ohc.sync.payload_size",
		metric.WithDescription("Size of the sync payload"),
	)
	if err != nil {
		errs = append(errs, err)
	}

	AutoDreamMemoriesCompressedCounter, err = m.Int64Counter(
		"ohc_autodream_memories_compressed_total",
		metric.WithDescription("Total number of agent sessions compressed into AutoDream memories"),
	)
	if err != nil {
		errs = append(errs, err)
	}

	SyncEscalationsCount, err = m.Int64Counter(
		"ohc.sync.escalations.count",
		metric.WithDescription("Total successfully synced missions with CLOUD_ESCALATION status"),
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

	agentApiErrorsCounter, err = m.Int64Counter(
		"ohc_agent_api_errors_total",
		metric.WithDescription("Total API errors made by or for agents"),
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

	swarmTasksCompletedCounter, err = m.Int64Counter(
		"ohc_swarm_tasks_completed",
		metric.WithDescription("Total swarm tasks completed"),
	)
	if err != nil {
		errs = append(errs, err)
	}

	swarmTaskTransitionsCounter, err = m.Int64Counter(
		"ohc_swarm_task_transitions_total",
		metric.WithDescription("Total number of swarm task state transitions"),
	)
	if err != nil {
		errs = append(errs, err)
	}

	taskEnqueuedCounter, err = m.Int64Counter(
		"ohc_task_enqueued_total",
		metric.WithDescription("Total number of tasks enqueued"),
	)
	if err != nil {
		errs = append(errs, err)
	}

	taskFailedCounter, err = m.Int64Counter(
		"ohc_task_failed_total",
		metric.WithDescription("Total number of tasks failed"),
	)
	if err != nil {
		errs = append(errs, err)
	}

	cacheHitsCounter, err = m.Int64Counter(
		"ohc_cache_hits_total",
		metric.WithDescription("Total cache hits for LLM operations"),
	)
	if err != nil {
		errs = append(errs, err)
	}

	cacheMissesCounter, err = m.Int64Counter(
		"ohc_cache_misses_total",
		metric.WithDescription("Total cache misses for LLM operations"),
	)
	if err != nil {
		errs = append(errs, err)
	}

	AutoDreamMemoriesIngestedCounter, err = m.Int64Counter(
		"ohc_autodream_memories_ingested_total",
		metric.WithDescription("Total number of AutoDream memories ingested"),
	)
	if err != nil {
		errs = append(errs, err)
	}

	TaskQueueLengthGauge, err = m.Int64UpDownCounter(
		"ohc_task_queue_length",
		metric.WithDescription("Current length of the shared task queue"),
	)
	if err != nil {
		errs = append(errs, err)
	}

	TaskProcessingLatency, err = m.Float64Histogram(
		"ohc_task_processing_latency_seconds",
		metric.WithDescription("Task processing latency in seconds"),
		metric.WithUnit("s"),
	)
	if err != nil {
		errs = append(errs, err)
	}

	TeammateMeshBroadcastsCounter, err = m.Int64Counter(
		"teammate_mesh_broadcasts_total",
		metric.WithDescription("Total number of Teammate Mesh broadcast messages sent"),
	)
	if err != nil {
		errs = append(errs, err)
	}

	TeammateMeshDirectMessagesCounter, err = m.Int64Counter(
		"teammate_mesh_direct_messages_total",
		metric.WithDescription("Total number of Teammate Mesh direct messages sent"),
	)
	if err != nil {
		errs = append(errs, err)
	}

	SyncCompletedCount, err = m.Int64Counter(
		"sync_completed_count",
		metric.WithDescription("Total successfully synced rows"),
	)
	if err != nil {
		errs = append(errs, err)
	}

	SyncFailedCount, err = m.Int64Counter(
		"sync_failed_count",
		metric.WithDescription("Total failed synced rows"),
	)
	if err != nil {
		errs = append(errs, err)
	}

	RateLimitExceededCount, err = m.Int64Counter(
		"api_rate_limit_exceeded_count",
		metric.WithDescription("Total number of API rate limit exceeded (HTTP 429) occurrences"),
	)
	if err != nil {
		errs = append(errs, err)
	}

	if len(errs) > 0 {
		return errs[0]
	}

	return nil
}

// Middleware injects telemetry instrumentation into an HTTP handler chain.
//
//   - next: http.Handler; The next HTTP handler in the request pipeline.
//
// Accepts parameters: next http.Handler (No Constraints).
// Returns http.Handler.
// Produces no errors.
// Has no side effects.
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

// Verbosity controls the detail level of standard output logging for the telemetry module.  Constraints: Defaults to 1. Set to 2 or higher for verbose request logging.
// Accepts no parameters.
// Returns nothing.
// Produces no errors.
// Has no side effects.
var Verbosity = 1 // Default level

// MetricsHandler provides an HTTP handler that exposes the collected Prometheus metrics.
//
// Accepts no parameters.
// Returns http.Handler.
// Produces no errors.
// Has no side effects.
func MetricsHandler() http.Handler {
	return promhttp.Handler()
}

// RecordTokenUsage increments the global counter for LLM tokens consumed by the workforce.
//
//   - ctx: context.Context; The context of the active trace or request.
//   - agentID: string; The identifier of the agent consuming the tokens.
//   - role: string; The role of the agent.
//   - model: string; The specific AI model being inferred (e.g., gpt-4o).
//   - tokenType: string; The type of tokens (e.g., prompt or completion).
//   - count: int64; The number of tokens consumed.
//
// Accepts parameters: ctx context.Context, agentID, role, model, tokenType string, count int64 (No Constraints).
// Returns nothing.
// Produces no errors.
// Has no side effects.
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

// RecordAgentApiCall increments the global counter for external tool or API invocations made by agents.
//
//   - ctx: context.Context; The context of the active trace or request.
//   - agentID: string; The identifier of the agent making the call.
//   - role: string; The role of the agent.
//   - api: string; The name or route of the invoked API/tool.
//
// Accepts parameters: ctx context.Context, agentID, role, api string (No Constraints).
// Returns nothing.
// Produces no errors.
// Has no side effects.
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

// RecordAgentApiError increments the global counter for external tool or API invocations errors made by agents.
//
//   - ctx: context.Context; The context of the active trace or request.
//   - agentID: string; The identifier of the agent making the call.
//   - role: string; The role of the agent.
//   - api: string; The name or route of the invoked API/tool.
//
// Accepts parameters: ctx context.Context, agentID, role, api string (No Constraints).
// Returns nothing.
// Produces no errors.
// Has no side effects.
func RecordAgentApiError(ctx context.Context, agentID, role, api string) {
	if agentApiErrorsCounter == nil {
		return
	}
	agentApiErrorsCounter.Add(ctx, 1, metric.WithAttributes(
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
		_ = BufferMetricFunc(ctx, "agent_api_error", string(payloadBytes))
	}
}

// RecordHumanInteraction increments the global counter for events involving direct human oversight.
//
//   - ctx: context.Context; The context of the active trace or request.
//   - interactionType: string; The category of interaction (e.g., approval, handoff).
//
// Accepts parameters: ctx context.Context, interactionType string (No Constraints).
// Returns nothing.
// Produces no errors.
// Has no side effects.
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

// RecordMeetingEvent increments the global counter for collaborative meeting room actions.
//
//   - ctx: context.Context; The context of the active trace or request.
//   - eventType: string; The nature of the meeting event (e.g., start, message, end).
//
// Accepts parameters: ctx context.Context, eventType string (No Constraints).
// Returns nothing.
// Produces no errors.
// Has no side effects.
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

// LogAgentExecution provides structured JSON logging for agent execution traces.
//
//   - ctx: context.Context; The context of the active trace or request.
//   - agentID: string; The identifier of the agent.
//   - role: string; The role of the agent.
//   - api: string; The API or tool being executed.
//   - eventType: string; The specific type of the event (e.g. task, status).
//   - content: string; The content or message payload associated with the execution.
//
// Accepts parameters: ctx context.Context, agentID, role, api, eventType, content string (No Constraints).
// Returns nothing.
// Produces no errors.
// Has no side effects.
func LogAgentExecution(ctx context.Context, agentID, role, api, eventType, content string) {
	var parsed interface{}
	if err := json.Unmarshal([]byte(content), &parsed); err == nil {
		redacted := RedactInterfacePII(parsed)
		if redactedBytes, err := json.Marshal(redacted); err == nil {
			content = string(redactedBytes)
		} else {
			content = RedactPII(content)
		}
	} else {
		content = RedactPII(content)
	}

	slog.InfoContext(ctx, "agent execution trace",
		"component", "telemetry",
		"agent_id", agentID,
		"role", role,
		"api", api,
		"event_type", eventType,
		"content", content,
	)
}

// Global buffer function pointer to inject dependency without circular imports.
var BufferMetricFunc func(ctx context.Context, metricType string, payload string) error

// RecordTokenBurnRate updates the forecast gauge for a tenant's token burn rate.
func RecordTokenBurnRate(ctx context.Context, organizationID string, rate float64) {
	if tokenBurnRateGauge != nil {
		tokenBurnRateGauge.Record(ctx, rate, metric.WithAttributes(
			attribute.String("organization_id", organizationID),
		))
	}
}

// RecordSwarmTaskCompleted increments the global counter for completed swarm tasks.
func RecordSwarmTaskCompleted(ctx context.Context, missionID string) {
	if swarmTasksCompletedCounter == nil {
		return
	}
	swarmTasksCompletedCounter.Add(ctx, 1, metric.WithAttributes(
		attribute.String("mission_id", missionID),
	))

	if BufferMetricFunc != nil {
		payloadBytes, _ := json.Marshal(map[string]interface{}{
			"mission_id": missionID,
		})
		_ = BufferMetricFunc(ctx, "swarm_task_completed", string(payloadBytes))
	}
}

// RecordCacheHit increments the global counter for LLM cache hits.
func RecordCacheHit(ctx context.Context, operation string, cacheType string) {
	if cacheHitsCounter == nil {
		return
	}
	cacheHitsCounter.Add(ctx, 1, metric.WithAttributes(
		attribute.String("operation", operation),
		attribute.String("cache_type", cacheType),
	))
}

// RecordTeammateMeshBroadcast increments the global counter for Teammate Mesh broadcasts.
func RecordTeammateMeshBroadcast(ctx context.Context, channel string) {
	if TeammateMeshBroadcastsCounter == nil {
		return
	}
	TeammateMeshBroadcastsCounter.Add(ctx, 1, metric.WithAttributes(
		attribute.String("channel", channel),
	))
}

// RecordTeammateMeshDirectMessage increments the global counter for Teammate Mesh direct messages.
func RecordTeammateMeshDirectMessage(ctx context.Context) {
	if TeammateMeshDirectMessagesCounter == nil {
		return
	}
	TeammateMeshDirectMessagesCounter.Add(ctx, 1)
}

// RecordAutoDreamMemoryIngested increments the counter when AutoDream ingests a memory.
func RecordAutoDreamMemoryIngested(ctx context.Context, agentID string) {
	if AutoDreamMemoriesIngestedCounter == nil {
		return
	}
	AutoDreamMemoriesIngestedCounter.Add(ctx, 1, metric.WithAttributes(
		attribute.String("agent_id", agentID),
	))
}

// RecordAutoDreamMemoryCompressed increments the counter when an agent session is compressed.
func RecordAutoDreamMemoryCompressed(ctx context.Context, agentID string) {
	if AutoDreamMemoriesCompressedCounter == nil {
		return
	}
	AutoDreamMemoriesCompressedCounter.Add(ctx, 1, metric.WithAttributes(
		attribute.String("agent_id", agentID),
	))
}

// RecordTaskQueueLength modifies the queue length gauge.
func RecordTaskQueueLength(ctx context.Context, amount int64) {
	if TaskQueueLengthGauge == nil {
		return
	}
	TaskQueueLengthGauge.Add(ctx, amount)
}

// RecordTaskProcessed Latency
func RecordTaskProcessed(ctx context.Context, latency time.Duration) {
	if TaskProcessingLatency == nil {
		return
	}
	TaskProcessingLatency.Record(ctx, latency.Seconds())
}

// RecordSyncEscalation increments the global counter for synced cloud escalations.
func RecordSyncEscalation(ctx context.Context, count int64) {
	if SyncEscalationsCount == nil {
		return
	}
	SyncEscalationsCount.Add(ctx, count)
}

// RecordSyncLatency records the latency of the sync process.
func RecordSyncLatency(ctx context.Context, latency time.Duration) {
	if SyncLatency == nil {
		return
	}
	SyncLatency.Record(ctx, latency.Seconds())
}

// RecordSyncPayloadSize records the size of the sync payload.
func RecordSyncPayloadSize(ctx context.Context, size int64) {
	if SyncPayloadSize == nil {
		return
	}
	SyncPayloadSize.Record(ctx, float64(size))
}

// RecordSwarmTaskTransition increments the counter for task state transitions.
func RecordSwarmTaskTransition(ctx context.Context, missionID string, oldStatus string, newStatus string) {
	if swarmTaskTransitionsCounter == nil {
		return
	}
	swarmTaskTransitionsCounter.Add(ctx, 1, metric.WithAttributes(
		attribute.String("mission_id", missionID),
		attribute.String("old_status", oldStatus),
		attribute.String("new_status", newStatus),
	))
}

// RecordTaskEnqueued increments the counter for tasks enqueued.
func RecordTaskEnqueued(ctx context.Context, taskID string) {
	if taskEnqueuedCounter == nil {
		return
	}
	taskEnqueuedCounter.Add(ctx, 1, metric.WithAttributes(
		attribute.String("task_id", taskID),
	))
}

// RecordTaskFailed increments the counter for tasks failed.
func RecordTaskFailed(ctx context.Context, taskID string, errStr string) {
	if taskFailedCounter == nil {
		return
	}
	taskFailedCounter.Add(ctx, 1, metric.WithAttributes(
		attribute.String("task_id", taskID),
		attribute.String("error", errStr),
	))
}

// RecordCacheMiss increments the global counter for LLM cache misses.
func RecordCacheMiss(ctx context.Context, operation string, cacheType string) {
	if cacheMissesCounter == nil {
		return
	}
	cacheMissesCounter.Add(ctx, 1, metric.WithAttributes(
		attribute.String("operation", operation),
		attribute.String("cache_type", cacheType),
	))
}
