---
status: PENDING
agent: Researcher
priority: P1
---

# Title: Implement Token Burn Rate Forecasting Engine

## Problem Statement
A critical observability gap exists in the OHC Hybrid Architecture telemetry. While raw token usage (`ohc_token_usage_total`) is currently tracked, there is no dedicated instrumentation for Token Burn Rate Forecasting. This lack of predictive extrapolation prevents the human CEO from receiving proactive budget alerts and visualizing per-tenant consumption trends.

## Research Report
- **Market Context**: Competing platforms often obscure underlying LLM costs, leading to bill shock for high-volume agentic operations.
- **OHC Requirement**: As detailed in the "Hybrid Telemetry Review & Observability Gap Analysis" (`docs/research/OBSERVABILITY_AUDIT_REPORT.md`), there is a "Missing Metric Coverage" issue: "There is no dedicated metric tracking **Token Burn Rate Forecasting**. While raw token usage is logged, per-tenant predictive extrapolation for budget alerts is not instrumented or visualized."
- **Data Flow**: To bridge this gap, we must implement a Token Burn Rate Forecasting Engine. This background worker will calculate the moving average burn rate using `ohc_token_usage_total` data, emit predictive cost alerts, and feed this data to a new Grafana dashboard panel.

## Design Doc
- **Module Path**: `srcs/server/telemetry` or `srcs/server/orchestration`.
- **Architecture**:
  - **Background Worker**: Implement a background worker in the Orchestration Hub that calculates the token burn rate.
  - **Metric Implementation**: Utilize the OpenTelemetry Float64Gauge `ohc_token_burn_rate_forecast`.
  - **Grafana Panel**: Propose a new Grafana panel definition for the "Hybrid Telemetry Review" dashboard to visualize this extrapolation.

## Implementation Prompt
Hello Implementer agent!
1. Locate the telemetry package in `srcs/server/telemetry/telemetry.go`.
2. Ensure the `tokenBurnRateGauge` (mapped to `ohc_token_burn_rate_forecast`) is correctly tracking values.
3. Create a Go background worker (e.g., `TokenForecastWorker`) that periodically reads recent token usage and calculates the burn rate per tenant.
4. Expose this metric using the `telemetry.RecordTokenBurnRate` function.
5. Achieve >90% test coverage for the new worker logic and metric recording.

## Priority
P1

## Estimated Scope
Medium
