<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# Missing Hybrid Telemetry Dashboard & Platform Metrics Coverage Gap

## Problem Statement

As OHC transitions its orchestrator layer to handle workloads both locally (Standalone) and via our multi-tenant Kubernetes backend (Cloud), operational visibility has become fragmented. We are currently flying blind on how different metrics (e.g., SQLite lock contention locally vs. PostgreSQL lock contention in the cloud) behave under load. Furthermore, crucial metrics such as agent sub-task queue depth, transition delays, tool autocorrection counts, and cross-mode synchronization duration are being recorded in the Go OpenTelemetry (`srcs/server/telemetry/telemetry.go` and `metrics.go`) implementation but are missing visualization representations in our current Grafana dashboards (`kairos_dashboard.json`). We cannot efficiently analyze, debug, or correct swarm behaviors when critical insights are left off the glass.

## Research Report

- **Telemetry Traces Existing**: A review of `srcs/server/telemetry/telemetry.go`, `metrics.go`, and related files reveals extensive coverage for Cloud vs Standalone metrics. Counters like `sqlite_lock_contention`, `sqlite_throttled_request`, `postgres_lock_contention`, `sub_agent_queue_delay`, `task_claim_contention`, `tool_autocorrection_total`, and prediction gauges such as `ohc_token_burn_rate_predicted_24h` are already actively collected.
- **Observability Gap**: The current `kairos_dashboard.json` dashboard lacks panels to display this mode-specific data. Key absent metrics include:
  - SQLite vs. Postgres connection/contention metrics.
  - Sub-agent queue length and queue delay (critical for tracking orchestration backlogs).
  - Agent task transitions and transition latency.
  - Token budget alerting and predictive burn rates (`ohc_token_burn_rate_predicted_24h`).
  - Swarm execution traces and capability violation counts.
- **Bottlenecks Identified**: Without dashboards comparing the task queue lengths and processing latencies between cloud and standalone workloads, we are unable to proactively spot scaling bottlenecks or memory constraints specific to a deployment topology. Standalone instances relying on polling and SQLite may exhibit very different latency characteristics than Cloud modes using Postgres `SKIP LOCKED` and Rueidis.
- **Recommendation**: Update the central Grafana dashboard to explicitly include hybrid mode panel sections, and unify metric collections to represent both the local and cloud experiences side by side.

## Design Doc

- **Architecture**:
  - The telemetry module already captures the necessary values using the `metric` package from OpenTelemetry.
  - The missing link is purely in visualization. We must update the `kairos_dashboard.json` configuration to ingest these specific `ohc_...` Prometheus metrics.
- **Visualization Strategy**:
  - Add a **Hybrid Deployment Health** section: Compare Cloud vs Standalone throughput and latency side-by-side. Add a panel mapping SQLite lock events versus Postgres retry/lock events.
  - Add a **Swarm Queue Diagnostics** section: Chart `ohc_sub_agent_queue_length` and `ohc_task_claim_contention_total` over time to monitor orchestration health.
  - Add a **Token Prediction & Cost Efficiency** section: Display the `ohc_token_burn_rate_predicted_24h` and `ohc_token_budget_alert_total` metrics. Track the overall `ohc_usd_burn_rate_forecast`.
- **UI Flow / Operator Experience**: The SRE or operator opens the OHC KAIROS Metrics Dashboard. They can see distinct rows corresponding to the agent task queue performance, token budget safety, and backend data-store contention, all sliced by the `tenant_id` and deployment type (`cloud` vs `standalone`).

## Implementation Prompt

Update the Grafana dashboard JSON configuration (`srcs/server/monitoring/dashboards/kairos_dashboard.json`) to include new panels and rows for the currently orphaned metrics collected in the Go backend. Specifically, implement:
1. A new row for **Hybrid Data Store Metrics** tracking SQLite (`ohc_sqlite_lock_contention_total`, `ohc_sqlite_throttled_request_total`) versus Postgres (`ohc_postgres_lock_contention_total`).
2. A new row for **Swarm Task & Queue Diagnostics** plotting `ohc_sub_agent_queue_length`, `ohc_task_claim_contention_total`, and `ohc_tool_autocorrection_total`.
3. A new row for **Token Forecasting & Costs** tracking the `ohc_token_burn_rate_predicted_24h` gauge and `ohc_token_budget_alert_total` counter.
Do not write new Go code, as the metrics are already exposed via Prometheus. Focus entirely on modifying the Grafana JSON file to define the panels.

## Priority
P1

## Estimated Scope
Medium
</div>