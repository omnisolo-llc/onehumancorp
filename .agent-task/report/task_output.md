# Title: Fix Observability Gaps in KAIROS Sub-Agent Task Processing Dashboards

## Problem Statement
The Swarm Orchestration engine (KAIROS) currently lacks dashboard observability regarding queue wait times for Sub-Agents and database lock contention rates in hybrid architectures. This is an issue for Swarm Operators monitoring performance because Cloud-Native deployments (using Postgres `FOR UPDATE SKIP LOCKED`) and Standalone deployments (using SQLite) manifest different lock contention profiles that are currently invisible. Without these metrics, we cannot effectively analyze Mode Parity or perform Swarm Bursting correctly.

## Research Report
An analysis of `srcs/server/telemetry/telemetry.go` reveals that `SubAgentQueueDelayHistogram` (`metric.Float64Histogram`) and `TaskClaimContentionTotal` (`metric.Int64Counter`) are implemented in the Go backend code and used in `tasks.go`, they are not visualized in the downstream Grafana dashboards for Swarm Health.

Additionally, we need to make sure we're correctly exporting these into Grafana so Swarm Operators can monitor these wait times and lock contention. Grafana Dashboards `hybrid-telemetry.json` and `kairos_hybrid_metrics.json` do not currently show panels for `ohc_sub_agent_queue_delay_seconds` and `ohc_task_claim_contention_total`.

## Design Doc
- **New Visualization**:
  - Add a Dashboard Panel for **Sub-Agent Queue Delay**: A time series graph or heat map visualizing `ohc_sub_agent_queue_delay_seconds`.
  - Add a Dashboard Panel for **Task Claim Contention**: A time series or stat panel visualizing `ohc_task_claim_contention_total` rate by mode (e.g., Postgres vs SQLite).
- **Integration Points**:
  - Modify existing Prometheus Dashboards (e.g. `deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json`, `deploy/helm/ohc/dashboards/hybrid-telemetry.json`) to include the missing metrics.
- **Visual Excellence**: All new panels must align with OHC premium visual tokens.

## Implementation Prompt
Hello Implementer!

1. Open `deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json`.
2. Add new panels to visualize the `ohc_sub_agent_queue_delay_seconds` histogram (e.g. 95th percentile queue wait time).
3. Add new panels to visualize the rate of `ohc_task_claim_contention_total` grouped by `mode`.
4. Replicate these changes to the Helm equivalent `deploy/helm/ohc/dashboards/hybrid-telemetry.json` if applicable.
5. Ensure the panels follow the OHC styling guidelines and look beautiful and professional.

## Priority
P1

## Estimated Scope
Small
