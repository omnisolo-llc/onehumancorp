# Harness Efficiency Telemetry & Observability Gap Analysis

## Title
Implement Missing Cloud vs. Standalone Harness Telemetry and Expand Metrics Visualization

## Problem Statement
Currently, the OHC platform tracks certain metrics for agent harness executions in Cloud vs. Standalone modes, but lacks full visibility into key inefficiencies, particularly execution latency and initialization latency comparisons. This leaves Swarm operators blind to environment-specific performance bottlenecks that delay AI agent responses for users like Maya or Carlos, ultimately leading to degraded user experience.

## Research Report
Based on a review of the `telemetry` module and Grafana dashboards (`monitoring/dashboards/harness_efficiency.json`):
- `HarnessInitLatency` and `HarnessDbIoLatency` metrics are properly configured in Prometheus (`telemetry.go`).
- `HarnessExecutionLatency` is instrumented in the backend code but is missing from the `harness_efficiency.json` dashboard, which means operators cannot track actual task execution time per mode.
- Missing insight: Is standalone mode slower during DB I/O or purely execution? Data points currently lack visualization for end-to-end execution.

## Design Doc
1. **PromQL Updates**: Add a new dashboard panel for `Harness Execution Latency (P95)` to `monitoring/dashboards/harness_efficiency.json`. Use `histogram_quantile(0.95, sum(rate(harness_execution_latency_bucket[5m])) by (le, deployment_mode))`.
2. **Dashboard Layout**: Place the new execution latency panel alongside Init Latency and DB I/O Latency for a holistic view. Ensure visual consistency with the OHC Aesthetic Injector style.
3. **Telemetry Coverage Check**: Ensure all agents correctly invoke `RecordHarnessExecutionLatency` upon completion of a task.

## Implementation Prompt
Update the `harness_efficiency.json` Grafana dashboard to include a panel for `Harness Execution Latency (P95)` grouped by `deployment_mode`. The visualization must match the existing UI scheme and utilize Prometheus histograms.

## Priority
P1

## Estimated Scope
Small
