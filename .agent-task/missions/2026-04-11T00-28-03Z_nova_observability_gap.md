---
status: DONE
agent: Nova
priority: P1
---

# Title: Implement Hybrid Observability Token Burn Rate & Local Metrics Sync

## Problem Statement
The OHC Hybrid Architecture (OHC-HA) currently exhibits observability gaps and mode-specific bottlenecks.
1. **Cloud-Native Mode:** While Prometheus captures basic metrics like `ohc_token_usage_total`, there is no specific tracking or forecasting for per-tenant Token Burn Rates. This prevents proactive budget alerting. Furthermore, Grafana dashboards (`deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json`) lack dedicated panels for token forecasting and detailed agent API error rate breakdowns.
2. **Standalone Desktop Mode:** Local SQLite (`swarm.db`) experiences severe `database is locked` errors during swarm execution burst workloads due to high concurrency. Additionally, local telemetry is not effectively captured and synced when transitioning online.

## Research Report
A comprehensive audit of `OBSERVABILITY_AUDIT_REPORT.md` and `TELEMETRY_REPORT.md` highlights the divergent bottlenecks between Cloud-Native and Standalone modes:
- **Cloud-Native:** The primary inefficiencies arise from network latency to external LLMs and PostgreSQL lock contention during massive `agent_missions` bulk updates. Error rates spike during aggressive pod scaling.
- **Standalone:** Single-user local Go backend operations are constrained by host I/O limits, rapidly hitting SQLite lock contention as exponential backoff is exhausted during parallel operations.

## Design Doc
To address these gaps, the following architectural updates are proposed:
1. **Token Burn Rate Forecasting Engine (Cloud-Native):**
   - **Component:** Implement a new backend background worker in the Orchestration Hub.
   - **Logic:** Calculate a moving average burn rate using `ohc_token_usage_total` and emit predictive cost alerts/metrics (`ohc_token_burn_rate_forecast` grouped by `organization_id`).
   - **Visualization:** Update Grafana provisioning (`deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json`) to include a "Token Burn Rate Forecast" panel.

2. **Standalone Concurrency Throttling (Standalone):**
   - **Component:** Introduce a dynamic concurrency limiter in `DelegateMission` (e.g., in `srcs/server/orchestration/hub.go`).
   - **Logic:** When `OHC_STANDALONE` mode is active, strictly throttle parallel agent writes to SQLite, ensuring zero-error stability by trading off raw throughput.

3. **Standalone Local Metric Buffer (Hybrid):**
   - **Component:** Local SQLite metrics buffer.
   - **Logic:** Aggregate local agent execution telemetry in Standalone mode and sync to the Cloud DB when an active connection is established to ensure holistic Swarm Intelligence observability.

## Implementation Prompt
Hello Implementer, please execute the following tasks:
1. **Token Burn Rate Engine:** Implement a backend background worker in the Orchestration Hub that calculates the moving average token burn rate and emits predictive alerts. Add a "Token Burn Rate Forecast" panel to `deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json` tracking the `ohc_token_burn_rate_forecast` metric. Apply OHC Glassmorphism CSS tokens using HTML/CSS text panels if needed.
2. **Standalone Concurrency Throttling:** Update task delegation logic (e.g., `DelegateMission`) to include a dynamic concurrency limiter that activates only in `OHC_STANDALONE` mode to prevent SQLite `database is locked` errors.
3. **Local Metric Buffer:** Implement a Standalone local metric buffer to aggregate agent telemetry and sync with the cloud when online.
4. **Testing:** Ensure comprehensive tests are written for all features and pass via `bazelisk test //...`. Do not break existing API contracts.

## Priority
P1

## Estimated Scope
Medium
