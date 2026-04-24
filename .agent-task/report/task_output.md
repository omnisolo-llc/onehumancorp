# Hybrid Telemetry Review & Observability Gap Analysis

## Problem Statement
The OHC Agent Swarm operates across both Cloud-native (multi-tenant) and Standalone (local) contexts. Currently, there is an observability gap making it difficult to analyze efficiency, swarm health, bottleneck differentials (like queue depths and database lock contention), and per-tenant cost metrics across these two distinct deployment modes. We need to identify these gaps and propose concrete tasks for closing them, ensuring our platform adheres to the "Full-Spectrum Observability" core value.

## Research Report
- **Telemetry Review:** Analyzed existing Go metric definitions within `src/server/telemetry`, `src/server/monitoring`, and `src/server/orchestration/kairos`.
- **Existing Metrics:**
  - KAIROS orchestration metrics (`ohc_kairos_transitions_total`, `ohc_agent_task_queue_depth`) are tagged by `mode` and `status`.
  - Harness metrics (`harness_init_latency`, `harness_db_io_latency`) are visualized grouping by `deployment_mode` in `monitoring/dashboards/harness_efficiency.json`.
  - Forecasting metrics (`ohc_token_burn_rate_predicted_24h`, `ohc_token_budget_alert_total`) exist but lack tagging by deployment mode or tenant ID in the primary Go telemetry definitions.
- **Identified Gaps:**
  1. **Cost Efficiency:** No dashboard tracks per-tenant token burn rates or budget alerts, nor differentiates these between Cloud and Standalone modes.
  2. **Swarm Contention:** There is a lack of metrics or visualization tracking Redis Redlock contention rates (`ohc:lock:{tenant_id}...`) which is a critical bottleneck differential between single-node Standalone and multi-node Cloud deployments.
  3. **Queue Health:** While `ohc_agent_task_queue_depth` is tracked, the terminal `FAILED` dead-letter transition (missions getting stuck and archived) lacks specific visualization to determine if failure rates skew higher in Cloud vs Standalone.

## Design Doc
To address these observability gaps, we propose adding targeted metrics and a corresponding Grafana dashboard:
1.  **Metric Expansion (`src/server/telemetry/metrics.go`):**
    - Convert `TokenBurnRatePredicted24h` and `TokenBudgetAlertTotal` to `metric.Float64ObservableGauge` and `metric.Int64Counter` using a custom meter that injects `attribute.String("deployment_mode", mode)` and `attribute.String("tenant_id", tenant_id)`.
    - Introduce new metrics for Distributed Lock Contention (`ohc_redis_lock_contention_total`, `ohc_redis_lock_wait_duration`).
    - Introduce new metrics for Swarm Health (`ohc_mission_dead_letter_total`).
2.  **Dashboard Creation (`monitoring/dashboards/swarm_efficiency_dashboard.json`):**
    - Add panels tracking the newly labeled cost metrics, distributed lock contention, and dead-letter queue metrics.
    - Group all timeseries data by `deployment_mode`.
    - Inject the standard OHC glassmorphism UI token block (`<style> .panel-container { backdrop-filter: blur(20px)... </style>`) to maintain aesthetic excellence.

## Implementation Prompt
1. Refactor `src/server/telemetry/metrics.go` to ensure all AI Token/Cost metrics accept and record `deployment_mode` and `tenant_id` attributes.
2. Implement lock telemetry within the Redis Distributed Lock layer (e.g., `src/server/lib/memory/` or `src/server/ironclaw/`), emitting `ohc_redis_lock_contention_total`.
3. Create a new dashboard file at `monitoring/dashboards/swarm_efficiency_dashboard.json` with the following panels:
   - "Token Burn Rate per Tenant (Cloud vs Standalone)"
   - "Redis Lock Contention Rate"
   - "Dead Letter Queue Transitions"
4. Ensure the JSON includes the HTML Text panel for OHC Premium Token styles.
5. Provide comprehensive E2E UI verification if these dashboards are exposed in the tenant portal, or verify Prometheus metric emission in standard Bazel tests.

## Priority
P1

## Estimated Scope
Medium
