# [observability] UltraPlan Deliberation Efficiency & Participation Metrics

## Title
UltraPlan Deliberation Efficiency & Participation Metrics

## Problem Statement
The KAIROS UltraPlan system enables parallel deliberation between agents for complex missions. While we currently track the duration of deliberation phases, we lack granular visibility into the *quality* and *efficiency* of the deliberation process. Specifically, there is no telemetry for:
1. **Revision Cycle Count**: How many times a plan moves between `REVISION_REQUIRED` and `DELIBERATING` before reaching `APPROVED`.
2. **Agent Participation Density**: Which agents are contributing the most critiques and whether certain agents are "deliberation bottlenecks."
3. **Approval Velocity**: The ratio of successfully approved plans versus those that expire or are abandoned in revision loops.

## Research Report
An audit of `src/server/orchestration/ultraplan.rs` shows that the `UltraPlanManager` handles state transitions but only emits a generic `RecordDeliberationPhaseDuration` metric.
- The `critiques` list in the `StateMachine` is a JSON array, making it hard to query via Prometheus without explicit instrumentation.
- There is no counter for the total number of revision loops.
- Agent contribution is stored in the DB but not exposed as real-time observability.
This gap prevents OHC from identifying "infinite loops" in agent deliberation and optimizing the swarm's consensus protocols.

## Design Doc
1. **New Prometheus Metrics in `src/server/telemetry`**:
   - `ohc_ultraplan_critiques_total` (Counter): Labels: `agent_id`, `plan_id`, `mode`. Incremented in `SubmitCritique`.
   - `ohc_ultraplan_revision_cycles_total` (Counter): Labels: `plan_id`, `mode`. Incremented when transitioning from `REVISION_REQUIRED` to `DELIBERATING`.
   - `ohc_ultraplan_approvals_total` (Counter): Labels: `mode`. Incremented when phase becomes `APPROVED`.
   - `ohc_ultraplan_abandonments_total` (Counter): Labels: `mode`. Incremented if a plan is deleted or failed during deliberation.

2. **Code Changes**:
   - Update `src/server/orchestration/ultraplan.rs`:
     - Inside `SubmitCritique`, call `telemetry.RecordUltraPlanCritique(ctx, planID, agentID)`.
     - Inside `modifyStateMachine` or `UpdatePlanStatus`, detect the `REVISION_REQUIRED` -> `DELIBERATING` transition and call `telemetry.RecordUltraPlanRevision(ctx, planID)`.
   - Update `src/server/telemetry/telemetry/mod.rs` to include these new metrics and record functions.

3. **Grafana Updates**:
   - Add a "Deliberation Efficiency" row to `kairos_hybrid_metrics.json`.
   - Include a Bar Gauge showing critiques by `agent_id`.
   - Include a Time Series showing Revision Cycles vs Approvals.

## Implementation Prompt
You are an Implementer. Implement UltraPlan efficiency metrics as follows:
1. Modify `src/server/telemetry/telemetry/mod.rs` to add:
   - `UltraPlanCritiquesTotal` (Int64Counter) with labels `agent_id`, `plan_id`, `mode`.
   - `UltraPlanRevisionCyclesTotal` (Int64Counter) with labels `plan_id`, `mode`.
   - `UltraPlanApprovalsTotal` (Int64Counter) with label `mode`.
   - Export helper functions: `RecordUltraPlanCritique(ctx, planID, agentID)`, `RecordUltraPlanRevision(ctx, planID)`, and `RecordUltraPlanApproval(ctx)`. Use `kairos.GetMode()` for the mode label.
2. In `src/server/orchestration/ultraplan.rs`:
   - In `SubmitCritique`, call `RecordUltraPlanCritique`.
   - In `modifyStateMachine`, detect if `newPhase == "APPROVED"` and `oldPhase != "APPROVED"`, then call `RecordUltraPlanApproval`.
   - Detect if the plan is transitioning from `REVISION_REQUIRED` back to a state where work resumes (e.g., `DELIBERATING`), and call `RecordUltraPlanRevision`.
3. Update `deploy/docker/grafana/provisioning/dashboards/kairos_hybrid_metrics.json` to visualize these metrics. Use a Bar Gauge for "Critiques by Agent" and a Stat panel for "Average Revision Cycles per Plan".
4. Ensure `bazelisk test //src/server/orchestration/...` passes.

## Priority
P2

## Estimated Scope
Medium
