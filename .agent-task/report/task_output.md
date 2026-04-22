# [observability]_hybrid_telemetry_gap_resolution.md

## Title
Resolve Hybrid Telemetry Observability Gaps: Implement Token Burn Rate Forecasting & Standalone SQLite Throttling

## Problem Statement
The OHC platform currently lacks critical observability components required for Swarm self-correction and operational health monitoring across its hybrid deployment models. Specifically, from the perspective of a non-technical business owner or swarm operator, there is no visibility into how quickly AI tokens are being consumed (token burn rate forecasting), which is essential for budget management and alerting. Furthermore, in Standalone mode, the local desktop swarm often fails under heavy workloads due to `database is locked` errors caused by excessive parallel writes to the shared SQLite file. This degrades the user experience by causing task failures and stalled AI agent operations.

## Research Report
An audit of the OHC Hybrid Architecture (OHC-HA) telemetry has revealed divergent bottleneck profiles:
1.  **Cloud-Native Bottlenecks:** While API nodes scale efficiently, heavy concurrency causes network latency to external LLMs and PostgreSQL lock contention during massive `agent_missions` bulk updates.
2.  **Standalone Desktop Bottlenecks:** The local Go backend, utilizing a SQLite `swarm.db`, faces severe lock contention. The lack of concurrent connection handling limits throughput, causing failures during task delegation bursts.
3.  **Observability Gaps:** The current Grafana "Hybrid Telemetry Review" dashboard (`deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json`) missing critical panels for token burn rate forecasting. Additionally, while total token usage (`ohc_token_usage_total`) is tracked, no backend logic exists to extrapolate this into actionable predictive metrics (`ohc_token_burn_rate_predicted_24h`).

References:
- `docs/technical/reports/observability-audit-report.md`
- `deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json`

## Design Doc
To address these issues, the system requires two primary architectural enhancements:
1.  **Token Burn Rate Forecasting Engine:**
    -   **Entity Type:** Background Worker (within Orchestration Hub).
    -   **Key Relationship:** Analyzes existing `ohc_token_usage_total` telemetry to calculate a rolling average.
    -   **Integration:** Emits a new Prometheus metric (`ohc_token_burn_rate_predicted_24h`) exposing 24-hour token usage projections per tenant. Updates to the Grafana dashboard will visualize this metric.
2.  **Standalone SQLite Concurrency Throttling:**
    -   **Entity Type:** Dynamic Concurrency Limiter.
    -   **Key Relationship:** Integrates within the `DelegateMission` workflow.
    -   **Integration:** Evaluates the runtime mode. When running in Standalone mode, it restricts the number of concurrent database write operations to the shared SQLite instance, effectively queueing agent tasks to eliminate `database is locked` errors. This trades raw throughput for stability.

## Implementation Prompt
Implement a unified observability and stability upgrade for the OHC platform:
1.  **Forecasting Engine:** Build a background service that continuously calculates the predicted 24-hour token usage per tenant based on current burn rates. Expose this data via a new Prometheus metric named `ohc_token_burn_rate_predicted_24h`.
2.  **Dashboard Update:** Update the "Hybrid Telemetry Review" Grafana dashboard to include a new panel visualizing the token burn rate forecast.
3.  **Standalone Throttling:** Introduce a concurrency control mechanism in the core task delegation flow (`DelegateMission`). This mechanism must detect if the system is running in Standalone mode and, if so, enforce a strict limit on parallel SQLite writes to completely eliminate database lock exhaustion errors.

**Acceptance Criteria:**
- The new `ohc_token_burn_rate_predicted_24h` metric is available on the `/metrics` endpoint.
- The Grafana dashboard successfully displays the new forecasting panel.
- In Standalone mode, heavy burst task delegations do not result in `database is locked` panics or exhausted retries, achieving 100% task success rate under load.
- E2E tests verify the throttling behavior in Standalone environments.

## Priority
P1

## Estimated Scope
Medium

issue_id: 3961