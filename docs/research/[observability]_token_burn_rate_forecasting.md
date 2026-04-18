# Title: Implement Token Burn Rate Forecasting Engine and Metrics

## Problem Statement
While basic metrics exist (e.g., `ohc_token_usage_total`), a critical audit of `TELEMETRY_REPORT.md` and the existing Grafana provisioning reveals a systemic gap: there is no dedicated metric tracking **Token Burn Rate Forecasting**. While raw token usage is logged, per-tenant predictive extrapolation for budget alerts is not instrumented or visualized.

## Research Report
The existing telemetry infrastructure captures raw usage via `ohc_token_usage_total`, but lacks the ability to forecast future consumption. This creates a critical gap for OHC Cloud-Native multi-tenant users who need predictive budget alerts before they hit rate limits or exhaust funds.

Our analysis of `srcs/server/telemetry/` shows that a `TokenBurnRateForecast` metric (Gauge) should be added. A background worker must be implemented to calculate the moving average burn rate using historical `ohc_token_usage_total` data and emit predictive cost alerts.

Furthermore, the "Hybrid Telemetry Review" dashboard (`hybrid-telemetry.json`) lacks specific panels for token forecasting and fine-grained agent API error rate breakdowns.

## Design Doc
1. Define a new Prometheus metric in `srcs/server/telemetry/telemetry.go` (or `token_forecast_worker.go` if it already exists, or create a new `token_forecast_worker.go`): `TokenBurnRateForecast` (Gauge), tagged by `tenant_id` and `mode`.
2. Implement a background worker (`TokenBurnRateForecaster` or similar) in the Orchestration Hub that periodically calculates the moving average burn rate based on recent `ohc_token_usage_total` and updates the `TokenBurnRateForecast` gauge.
3. Update `deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json` (or create a dedicated token forecasting dashboard if appropriate) to include panels for:
   - Predictive Token Burn Rate by Tenant.
   - Fine-grained agent API error rate breakdowns.
4. Ensure the UI panels apply the OHC Premium Glassmorphism styling natively inside Grafana's Text/HTML panels.

## Implementation Prompt
You are an Implementer. Implement the design above:
1. Add `TokenBurnRateForecast` Prometheus Gauge metric in the telemetry package.
2. Implement a background worker in `srcs/server/orchestration/` (e.g., `token_forecast_engine.go`) or `srcs/server/telemetry/` (e.g., `token_forecast_worker.go`) that calculates the moving average token usage and updates the forecast metric. Make sure this worker has a stop mechanism (`sync.Once`) and handles background loop properly without holding database transactions open.
3. Update `deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json` to include a new panel for token forecasting using OHC premium tokens (e.g., `backdrop-filter: blur(20px)`, `font-family: 'Outfit', 'Inter'`).
4. Write 100% coverage unit tests for the new worker and metrics.
5. Verify tests pass using `bazel test //...` or `bazelisk test //...`.

## Priority
P1

## Estimated Scope
Medium
