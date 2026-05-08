package telemetry

import (
	"context"
	"log"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/metric"
)

var (
	metricsMeter                   = otel.Meter("harness")
	tokenBurnRatePredicted24hGauge metric.Float64ObservableGauge
	tokenBudgetAlertTotalCounter   metric.Int64Counter
)

func init() {
	var err error
	tokenBurnRatePredicted24hGauge, err = metricsMeter.Float64ObservableGauge(
		"ohc_token_burn_rate_predicted_24h",
		metric.WithDescription("Predicted 24h token burn rate"),
	)
	if err != nil {
		log.Printf("Failed to create tokenBurnRatePredicted24hGauge: %v", err)
	}

	tokenBudgetAlertTotalCounter, err = metricsMeter.Int64Counter(
		"ohc_token_budget_alert_total",
		metric.WithDescription("Total token budget alerts"),
	)
	if err != nil {
		log.Printf("Failed to create tokenBudgetAlertTotalCounter: %v", err)
	}
}

func RecordTokenBurnRatePredicted24h(ctx context.Context, tenant_id string, mode string, val float64) error {
	bufferMetricHelper(ctx, "ohc_token_burn_rate_predicted_24h", val, map[string]interface{}{
		"tenant_id":       tenant_id,
		"deployment_mode": mode,
	})
	return nil
}

func RecordTokenBudgetAlert(ctx context.Context, tenant_id string, mode string) error {
	if tokenBudgetAlertTotalCounter != nil {
		opts := metric.WithAttributes(
			attribute.String("deployment_mode", mode),
			attribute.String("tenant_id", tenant_id),
		)
		tokenBudgetAlertTotalCounter.Add(ctx, 1, opts)
	}
	bufferMetricHelper(ctx, "ohc_token_budget_alert_total", 1, map[string]interface{}{
		"tenant_id":       tenant_id,
		"deployment_mode": mode,
	})
	return nil
}
