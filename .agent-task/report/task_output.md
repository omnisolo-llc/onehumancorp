# Cost Efficiency & Observability Insights

## Problem Statement
The OHC platform must track Return on Agent (ROA) to evaluate swarm cost efficiency across Cloud and Standalone modes. However, the core observability metric, `ohc_mission_cost_cents`, is missing from the OpenTelemetry implementation, preventing accurate tracking of per-mission and per-tenant costs.

## Research Report
1. **Metrics Audit**: Reviewed `src/server/telemetry/telemetry.go` and `telemetry_test.go`. The system tracks `ohc_agent_cost_estimate_usd` via `AgentCostEstimateUSD` but lacks the required `ohc_mission_cost_cents`.
2. **Cost Blueprint Alignment**: According to `docs/business/cost-blueprint.md`, `ohc_mission_cost_cents` is the core metric to track ROA via Prometheus.
3. **Bottleneck & Telemetry Findings**: The lack of granular, cent-based tracking per mission obscures the cost differences between multi-tenant cloud setups (where LLM costs dominate) and local standalone setups (where local models might be "free" but mission duration varies).

## Design Doc
- **Telemetry Update**: Add `MissionCostCents` as a `Float64Counter` in `src/server/telemetry/telemetry.go`.
- **Labels/Attributes**: Include `tenant_id`, `mission_id`, `agent_id`, and `role` to allow granular slicing in Grafana.
- **Recording Function**: Implement `RecordMissionCostCents` alongside existing `RecordAgentCost`.

## Implementation Prompt
1. Add `MissionCostCents` (`Float64Counter`) initialized as `ohc_mission_cost_cents` in `telemetry.go`.
2. Add `RecordMissionCostCents(ctx context.Context, tenantID, missionID, agentID, role string, costCents float64)`.
3. Update `src/server/billing/tracker.go` or `src/server/harness/manager.go` to emit this metric when calculating mission costs.
4. Ensure 100% test coverage in `telemetry_test.go`.

## Priority
P1

## Estimated Scope
Small
