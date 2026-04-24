# Implement ohc_mission_cost_cents Metric

## Problem Statement
The platform's core observability metric for calculating Return on Agent (ROA) is missing. The `docs/business/cost-blueprint.md` dictates that `ohc_mission_cost_cents` must be tracked to visualize ROA. However, the telemetry system currently only tracks `ohc_agent_cost_estimate_usd`. To accurately evaluate the swarm's cost efficiency and perform mode-specific throughput analysis (Cloud vs Standalone), this precise metric must be implemented as defined.

## Research Report
- Analyzed `docs/business/cost-blueprint.md`: explicitly requires `ohc_mission_cost_cents` via OpenTelemetry and Prometheus.
- Audited `src/server/telemetry/telemetry.go`: Found `AgentCostEstimateUSD` (Float64Counter) but `MissionCostCents` is completely absent.
- Cost metering and ROA calculations require granular, cent-based telemetry attached to specific missions and tenants.

## Design Doc
- **Entity**: `MissionCostCents` (OpenTelemetry `Float64Counter`).
- **Metric Name**: `ohc_mission_cost_cents`.
- **Attributes**: The metric must tag the data with `tenant_id`, `mission_id`, `agent_id`, and `role` to allow slicing by tenant and mission type in Grafana.
- **Integration Points**:
  - `src/server/telemetry/telemetry.go` for metric initialization and helper function.
  - Interceptors/middleware where agent costs are calculated to record the metric.

## Implementation Prompt
1. In `src/server/telemetry/telemetry.go`, initialize a new `Float64Counter` named `MissionCostCents` using the identifier `ohc_mission_cost_cents`.
2. Implement a `RecordMissionCostCents(ctx context.Context, tenantID string, missionID string, agentID string, role string, costCents float64)` function.
3. Hook `RecordMissionCostCents` into the relevant places where agent/mission costs are calculated (e.g., alongside `RecordAgentCost`).
4. Write 100% test coverage for the new functions in `src/server/telemetry/telemetry_test.go`.

## Priority
P1

## Estimated Scope
Small
