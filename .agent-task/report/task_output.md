# [Observability] Add Harness Execution Latency to Grafana Dashboard

## Problem Statement
The OHC platform tracks multiple key metrics for AI agent harness efficiency, including `harness_init_latency` and `harness_db_io_latency`. However, while `HarnessExecutionLatency` is instrumented in the backend `telemetry` module (`srcs/server/telemetry/telemetry.go`), it is completely absent from the Grafana dashboard (`monitoring/dashboards/harness_efficiency.json`). This creates an observability gap, making it impossible for operators to compare end-to-end harness execution times between Cloud and Standalone deployment modes visually.

## Research Report
- **Telemetry Verification**: `HarnessExecutionLatency` is correctly registered and recorded in `srcs/server/telemetry/telemetry.go`.
- **Dashboard Analysis**: A review of `monitoring/dashboards/harness_efficiency.json` reveals panels for "Harness Init Latency" and "Harness DB I/O Latency". The panel for "Harness Execution Latency" is missing.
- **Impact**: Without this visualization, diagnosing performance bottlenecks that occur specifically during the execution phase (after initialization and outside of DB I/O) is difficult, particularly when analyzing mode-specific throughput differences.

## Design Doc
- **Target**: `monitoring/dashboards/harness_efficiency.json`
- **Addition**: A new timeseries panel added to the `panels` array.
- **Panel Specifications**:
  - Title: "Harness Execution Latency (P95)"
  - Prometheus Expression: `histogram_quantile(0.95, sum(rate(harness_execution_latency_bucket[5m])) by (le, deployment_mode))`
  - Legend Format: `{{deployment_mode}} P95`
  - Position: Place to the right of the existing panels (e.g., `x: 24`, `y: 3`, matching height `8` and width `12`).
- No changes to Go codebase are required, as the metric is already correctly exposed.

## Implementation Prompt
Update `monitoring/dashboards/harness_efficiency.json` to include a new Grafana panel for "Harness Execution Latency (P95)". The panel should query the Prometheus metric `harness_execution_latency_bucket`, displaying the P95 latency grouped by `deployment_mode`. Ensure the new panel follows the visual styling (e.g., timeseries format, ms unit) of the existing Init and DB I/O panels and is positioned logically on the dashboard grid.

## Priority
P2 (Medium)

## Estimated Scope
Small
