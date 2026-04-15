package telemetry

import (
	"context"
	"encoding/json"
	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/metric"
)

var (
	minimaxLatencyHistogram metric.Float64Histogram
	minimaxCallsCounter     metric.Int64Counter
	minimaxErrorsCounter    metric.Int64Counter
)

func initMinimaxMetrics(m mockableMeter) error {
	var err error
	var errs []error

	minimaxLatencyHistogram, err = m.Float64Histogram(
		"ohc_minimax_api_latency_seconds",
		metric.WithDescription("Latency of Minimax API calls"),
		metric.WithUnit("s"),
	)
	if err != nil {
		errs = append(errs, err)
	}

	minimaxCallsCounter, err = m.Int64Counter(
		"ohc_minimax_api_calls_total",
		metric.WithDescription("Total number of Minimax API calls"),
	)
	if err != nil {
		errs = append(errs, err)
	}

	minimaxErrorsCounter, err = m.Int64Counter(
		"ohc_minimax_api_errors_total",
		metric.WithDescription("Total number of failed Minimax API calls"),
	)
	if err != nil {
		errs = append(errs, err)
	}

	if len(errs) > 0 {
		return errs[0]
	}
	return nil
}

// RecordMinimaxCall records metrics for a Minimax API call.
func RecordMinimaxCall(ctx context.Context, operation string, durationSeconds float64, err error) {
	attrs := metric.WithAttributes(attribute.String("operation", operation))

	if minimaxCallsCounter != nil {
		minimaxCallsCounter.Add(ctx, 1, attrs)
	}
	if minimaxLatencyHistogram != nil {
		minimaxLatencyHistogram.Record(ctx, durationSeconds, attrs)
	}
	if err != nil && minimaxErrorsCounter != nil {
		minimaxErrorsCounter.Add(ctx, 1, attrs)
	}

	if BufferMetricFunc != nil {
		payloadMap := map[string]interface{}{
			"operation":        operation,
			"duration_seconds": durationSeconds,
		}
		if err != nil {
			payloadMap["error"] = err.Error()
		}
		redacted := RedactInterfacePII(payloadMap)
		payloadBytes, _ := json.Marshal(redacted)
		_ = BufferMetricFunc(ctx, "ohc_minimax_api_latency_seconds", string(payloadBytes))
	}
}
