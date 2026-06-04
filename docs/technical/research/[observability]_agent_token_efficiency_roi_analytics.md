# [observability] Agent Token Efficiency & ROI Analytics

## Title
Agent Token Efficiency & ROI Analytics

## Problem Statement
The OHC swarm consumes significant LLM tokens across various providers. While we have basic counters for token usage and burn-rate forecasts, we lack **context-aware efficiency metrics**. We cannot currently answer:
1. **Tokens Per Successful Task**: Which agents or models are most "efficient" at completing tasks with the fewest tokens?
2. **Cost-to-Success Ratio**: What is the USD cost of a successful task completion versus a failed one?
3. **Redundant Reasoning Waste**: How many tokens are spent in `REVISION_REQUIRED` states or `FAILED` attempts compared to productive output?

## Research Report
Current telemetry in `src/server/telemetry/telemetry.go` tracks `ohc_token_usage_total` with labels for `agent_id`, `role`, `model`, and `type`. However, this data is decoupled from the **outcome** of the tasks.
- The `TokenForecastWorker` focuses on rate-limiting and billing projections, not performance optimization.
- There is no mechanism to correlate a specific task's total token consumption with its final `SUCCESS` or `FAILURE` state in Prometheus.
- ROI (Return on Investment) analysis is currently a manual manual calculation from DB logs.

## Design Doc
1. **Outcome-Labeled Token Metrics**:
   - Instead of just a global counter, we need a way to attribute tokens to the final outcome of a mission or task.
   - Introduce `ohc_task_tokens_total` (Counter) with labels: `mission_id`, `outcome` (success/failed), `agent_role`, `model`.

2. **ROI Calculation in Telemetry**:
   - Add `ohc_agent_roi_efficiency_score` (Gauge): A calculated score `(Tasks Completed) / (Tokens Consumed * 1000)` per `agent_role`.
   - Update `RecordTokenUsage` to optionally accept a `taskID` to enable downstream join operations in Prometheus/Grafana.

3. **Code Changes**:
   - Update `src/server/telemetry/telemetry.go`:
     - Modify `RecordTokenUsage` to include task context.
     - Add `RecordTaskOutcome(ctx, taskID, status, tokensConsumed)`.
   - Update `src/server/orchestration/statemachine/machine.go`:
     - When a task reaches `SUCCESS` or `FAILED`, emit the final token tally.

4. **Grafana Updates**:
   - Add an "Agent Efficiency & ROI" dashboard section.
   - Visualizations:
     - "Tokens per Success" by Agent Role.
     - "Waste Ratio" (Tokens spent on failed tasks vs successful ones).

## Implementation Prompt
You are an Implementer. Implement Agent Token ROI analytics:
1. In `src/server/telemetry/telemetry.go`:
   - Add `ohc_token_usage_by_outcome` (Int64Counter) with labels `outcome`, `agent_role`, `model`.
   - Add `ohc_agent_efficiency_gauge` (Float64Gauge) with labels `agent_role`.
   - Create a function `RecordTaskResolutionEfficiency(ctx, outcome string, role string, model string, tokens int64)`.
2. In `src/server/orchestration/statemachine/machine.go` (or wherever tasks are finalized):
   - Capture the total tokens consumed during the task's lifecycle (this may require a DB lookup or passing the count through the state machine).
   - Upon transition to `SUCCESS` or `FAILED`, call `telemetry.RecordTaskResolutionEfficiency`.
3. In `deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json` (or a new ROI dashboard):
   - Create a Heatmap showing Token Efficiency by Agent Role.
   - Create a Stat panel showing "Swarm ROI" (Total Successes / Total USD Cost).
4. Verify by running `bazelisk test //src/server/telemetry/...`.

## Priority
P1

## Estimated Scope
Medium
