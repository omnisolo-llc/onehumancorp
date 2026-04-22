<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 24px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #ffffff; box-shadow: 0 8px 32px 0 rgba(0, 0, 0, 0.3);">

# Research Report: Cost Efficiency Analysis (Token Forecasting)

**Title:** [Observability] Implement Token Burn Rate Forecasting for Cost Efficiency Analysis
**Priority:** P1
**Estimated Scope:** Medium

## Problem Statement

OHC operates efficiently, yet tenant-specific AI operating costs can scale unpredictably. While our backend instrumentations (via `ohc_agent_token_usage_total` and `AgentCostEstimateUSD` in `srcs/server/telemetry/telemetry.go`) log exact token usage, we lack a system that dynamically extrapolates this data to forecast future burn rates. For business owners (like Carlos the Freelance Handyman or Priya the Boutique Owner), unpredicted operational spikes due to excessive agent queries can lead to unexpected cost escalations. The Orchestration Hub currently doesn't project these per-tenant API call volumes or storage usages proactively.

Without token forecasting:
- **Tenants** may inadvertently cause expensive LLM escalation.
- **Swarm Operators** miss vital anomalously high token consumption metrics per tenant.
- There is zero visibility into near-future agent ROI efficiency inside Grafana for stakeholders.

## Design Doc

### 1. Architecture

We propose the integration of a **Token Burn Rate Forecasting Engine** into the Orchestration Hub. This worker calculates the moving average burn rate using the existing Prometheus metric `ohc_token_usage_total` or `AgentCostEstimateUSD`, providing predictive metrics for the immediate future.

```mermaid
graph TD;
    A[Orchestration Hub] -->|Cloud Mode| B(Max Parallelism & Bulk PG Updates);
    A -->|Standalone Mode| C(Throttled I/O Queue & SQLite Retry Backoff);
    C -.->|Sync when Online| D[Cloud Observability Metric Buffer];
    A --> E{Token Burn Rate Engine};
    E -->|Extrapolate Usage| F[Grafana Forecasting Panel];
    E -->|Calculate Moving Avg| G(ohc_token_burn_rate_forecast Gauge);

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class A,B,E premium;
    class C,D,F,G premium;
```

### 2. Integration Points

1.  **Backend Changes:** Add a new metrics gauge, `ohc_token_burn_rate_forecast` to track the "Predicted moving average of token burn rate per minute per tenant". The Prometheus variable `tokenBurnRateGauge` is already stubbed in `srcs/server/telemetry/telemetry.go`, but it needs an accompanying worker.
2.  **Dashboard Visualization:** A Text/HTML panel representing "Agent Token Efficiency Forecast" needs to be added to the Grafana dashboards, adhering to OHC styling protocols.

### 3. Data Flow

| Stage | Action |
|-------|--------|
| **Data Source** | Agent calls `telemetry.RecordAgentCost` tracking tokens/costs per `tenant_id`. |
| **Forecaster** | A background routine measures delta usage over time intervals. |
| **Exporter** | Exposes `ohc_token_burn_rate_forecast` to Prometheus. |
| **Visualization** | Grafana Dashboard queries moving average. |

## Implementation Prompt

As an Implementer Agent, you will bridge the observability gap for cost-efficiency.

**Objectives:**
1.  **Develop Forecaster Background Worker:** In `srcs/server/telemetry/forecaster.go`, implement a routine (e.g., `StartTokenBurnForecaster`) that periodically samples raw token metrics, calculates the moving average token usage per tenant, and updates the `tokenBurnRateGauge` and `usdBurnRateGauge` in Prometheus.
2.  **Update Grafana Dashboards:** Update `deploy/docker/grafana/provisioning/dashboards/token-forecast.json` (and the helm counterpart) to visualize the new `ohc_token_burn_rate_forecast` metric. Include a Premium Glassmorphism styled Text panel for descriptive contexts (similar to other OHC dashboards).
3.  **Testing:** Add full test coverage in `srcs/server/telemetry/forecaster_test.go` to ensure accurate forecasting logic without regressions.

**Acceptance Criteria:**
- 100% test coverage in Go for the forecasting module.
- Successful `bazelisk test //...` across the backend.
- The new Prometheus metric `ohc_token_burn_rate_forecast` is successfully instrumented and queryable on the backend.
- The Grafana JSON dashboards successfully visualize token burn forecasts.

```yaml
issue_id: OHC-TELEMETRY-004
title: "Implement Token Burn Rate Forecasting Engine"
```
</div>
