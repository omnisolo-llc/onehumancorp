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
   - Introduce a new counter metric to track task tokens with labels: outcome (success/failed), agent role, and model.

2. **ROI Calculation in Telemetry**:
   - Add a gauge metric for calculating agent ROI efficiency score, such as a calculated score per agent role.
   - Update the existing token usage tracking functions to optionally accept task context to enable downstream join operations in Prometheus/Grafana.

3. **Code Changes**:
   - Update `src/server/telemetry/telemetry.go`: Modify existing token usage records to include task context, and add a method to record task outcomes with the total tokens consumed.
   - Update `src/server/orchestration/statemachine/machine.go`: When a task reaches final statuses like `SUCCESS` or `FAILED`, emit the final token tally using the new telemetry method. This may require a DB lookup to query the tokens consumed by the task entity before transitioning to the final state.

4. **Database Changes**:
   - Ensure the database tables tracking the task or agent execution retain token consumption metrics, agent roles, and models so they can be queried upon state transition. Modify migrations if needed.

5. **Grafana Updates**:
   - Add an "Agent Efficiency & ROI" dashboard section to existing monitoring files.
   - Add visualizations for "Tokens per Success" by Agent Role.
   - Add "Waste Ratio" (Tokens spent on failed tasks vs successful ones).

## Implementation Prompt
You are an Implementer. Implement Agent Token ROI analytics:
1. In `src/server/telemetry/telemetry.go`:
   - Add an integer counter metric (e.g. `ohc_token_usage_by_outcome`) with labels for outcome, agent role, and model.
   - Add a float gauge metric for efficiency (e.g. `ohc_agent_efficiency_gauge`) with labels for agent role.
   - Create a function to record task resolution efficiency (recording outcome, role, model, tokens).
2. In `src/server/orchestration/statemachine/machine.go` (or wherever tasks are finalized):
   - Capture the total tokens consumed during the task's lifecycle (this may require a DB lookup of `tokens_consumed`, `agent_role`, and `model` columns from the DB table via the existing transaction).
   - Upon transition to `SUCCESS` or `FAILED`, call the new telemetry method for task resolution efficiency.
3. In `deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json` (or a new ROI dashboard):
   - Create a Heatmap showing Token Efficiency by Agent Role.
   - Create a Stat panel showing "Swarm ROI" (Total Successes / Total USD Cost).
4. Verify by running `bazelisk test //...` to ensure all tests pass.

## Priority
P1

## Estimated Scope
Medium
