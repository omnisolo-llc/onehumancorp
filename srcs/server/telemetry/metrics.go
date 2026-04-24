package telemetry

import (
	"go.opentelemetry.io/otel/metric"
)

var (
	// TokenBurnRateForecast tracks the predicted token burn rate for the next 24 hours per tenant.
	TokenBurnRateForecast metric.Float64Gauge
	// TokenBudgetAlertTotal tracks the total number of budget alerts emitted.
	TokenBudgetAlertTotal metric.Int64Counter
)

func initForecastingMetrics(m mockableMeter) error {
	var err error
	TokenBurnRateForecast, err = m.Float64Gauge(
		"ohc_token_burn_rate_forecast",
		metric.WithDescription("Predicted moving average of token burn rate for the next 24 hours per tenant"),
	)
	if err != nil {
		return err
	}

	TokenBudgetAlertTotal, err = m.Int64Counter(
		"ohc_token_budget_alert_total",
		metric.WithDescription("Total number of token budget alerts emitted"),
	)
	if err != nil {
		return err
	}

	return nil
}
